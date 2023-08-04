use bevy::{prelude::*, utils::HashSet};
use serde::Deserialize;
use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

use crate::{
    combat::{AttackEvent, HP},
    dungeon::{BlocksMovement, Map, TilePos},
    fieldofview::{FieldOfView, HideOutOfSight},
    setup::Player,
    stats::{Attributes, Skill, SkillSheet},
    utils::{get_dat_path, SpriteLayer},
    TurnState,
};

/// Marker component for mobs
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct Mob;

#[derive(Debug, Deserialize, Resource)]
pub struct MobList {
    mobs: HashMap<String, MobData>,
}

impl MobList {
    pub fn from_raws() -> Self {
        let path = get_dat_path("mobs.yaml");
        let reader = BufReader::new(File::open(path).unwrap());

        Self {
            mobs: serde_yaml::Deserializer::from_reader(reader)
                .map(|document| {
                    let mob = MobData::deserialize(document).unwrap();
                    (mob.name.to_lowercase(), mob)
                })
                .collect(),
        }
    }

    pub fn spawn<S: AsRef<str>>(
        &self,
        mob_name: S,
        commands: &mut Commands,
        asset_server: &AssetServer,
    ) -> Entity {
        self.mobs
            .get(&mob_name.as_ref().to_lowercase())
            .unwrap()
            .spawn(commands, asset_server)
    }
}

#[derive(Debug, Deserialize)]
pub struct MobData {
    name: String,
    sprite: String,
    #[serde(default = "default_blocks_movement")]
    blocks_movement: bool,
    #[serde(alias = "HP", alias = "hit_points")]
    hp: u16,
    defense: Skill,
    attack: Skill,
    #[serde(default = "Default::default")]
    attributes: Attributes,
}

impl MobData {
    pub fn spawn(&self, commands: &mut Commands, asset_server: &AssetServer) -> Entity {
        let mut skills = SkillSheet::new();
        skills.set("Defense", self.defense);
        skills.set("Attack", self.attack);
        let mut ec = commands.spawn((
            Name::new(self.name.clone()),
            SpriteBundle {
                texture: asset_server.load(self.sprite()),
                ..Default::default()
            },
            HP::new(self.hp),
            skills,
            self.attributes,
            HideOutOfSight::Hide,
            Mob,
        ));

        if self.blocks_movement {
            ec.insert(BlocksMovement);
        }

        ec.id()
    }

    pub fn sprite(&self) -> PathBuf {
        PathBuf::from("sprites").join(&self.sprite)
    }
}

fn default_blocks_movement() -> bool {
    true
}

#[allow(clippy::type_complexity)]
fn monster_ai(
    map: Res<Map>,
    fov_qry: Query<&FieldOfView>,
    mut attack: EventWriter<AttackEvent>,
    mut transform_qry_set: ParamSet<(
        Query<(Entity, &mut Transform), With<Mob>>,
        Query<(Entity, &Transform), With<Player>>,
        Query<&Transform, With<BlocksMovement>>,
    )>,
    mut next_state: ResMut<NextState<TurnState>>,
) {
    // Get the player's position first to avoid looking this up repeatedly
    if let Ok((player, player_pos)) = transform_qry_set.p1().get_single() {
        let player_tile = TilePos::from(player_pos);

        // NOTE: We assume below that any mob we end up moving is the only thing blocking a given tile
        // If this assumption changes, we'll need to change this to e.g. a HashMap tracking tile positions
        // and a count of how many entities are blocking it, and then we can decrement that count when
        // moving a monster; likewise when checking if a tile is blocked, we would instead check that
        // the blocking count is >0
        let mut unwalkable: HashSet<_> = transform_qry_set.p2().iter().map(TilePos::from).collect();

        for (monster, mut monster_pos) in transform_qry_set.p0().iter_mut() {
            let monster_tile = TilePos::from(*monster_pos);

            if let Some(tile_entity) = map.get(monster_tile) {
                if let Ok(&FieldOfView::Visible) = fov_qry.get(tile_entity) {
                    if monster_tile.distance(player_tile) <= 1 {
                        attack.send(AttackEvent::new(monster, player));
                    } else if let Some((path, _)) = pathfinding::directed::astar::astar(
                        &monster_tile,
                        |tile| {
                            map.neighbors_of(*tile).into_iter().filter_map(|tile| {
                                if !unwalkable.contains(&tile) {
                                    Some((tile, 1))
                                } else {
                                    None
                                }
                            })
                        },
                        |tile| tile.distance(player_tile),
                        |tile| *tile == player_tile,
                    ) {
                        unwalkable.remove(&monster_tile); // We're no longer blocking this tile, assume no one else is
                        unwalkable.insert(path[1]); // We are however blocking this next tile
                        *monster_pos = path[1].as_transform(SpriteLayer::Actor);
                    }
                }
            }
        }
    }

    // End the monsters' turn
    next_state.set(TurnState::WaitingForPlayer);
}

#[derive(Debug)]
pub struct MobsPlugin;

impl Plugin for MobsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, monster_ai.run_if(in_state(TurnState::MonsterTurn)));
    }
}
