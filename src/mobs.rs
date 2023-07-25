use bevy::{prelude::*, utils::HashSet};
use serde::Deserialize;
use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

use crate::{
    combat::{AttackEvent, Defense, Power, HP},
    dungeon::{BlocksMovement, Map, TilePos},
    fieldofview::FieldOfView,
    setup::Player,
    utils::get_dat_path,
    TurnState,
};

/// Marker component for mobs
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct Mob;

type MobMap = HashMap<String, MobData>;

#[derive(Debug, Deserialize, Resource)]
#[serde(from = "MobMap")]
pub struct MobList {
    names: HashMap<String, usize>,
    mobs: Vec<MobData>,
}

impl MobList {
    pub fn from_raws() -> Self {
        let path = get_dat_path("mobs.toml");
        let data = read_to_string(path).unwrap();
        toml::from_str(&data).unwrap()
    }

    pub fn spawn<S: AsRef<str>>(
        &self,
        mob_name: S,
        commands: &mut Commands,
        asset_server: &AssetServer,
    ) -> Entity {
        let id = self.names.get(mob_name.as_ref()).unwrap();
        let mob = self.mobs[*id].spawn(commands, asset_server);
        commands
            .entity(mob)
            .insert(Name::new(mob_name.as_ref().to_owned()));
        mob
    }
}

impl From<MobMap> for MobList {
    fn from(value: MobMap) -> Self {
        let (name_list, mobs) = value.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();
        let names = name_list
            .into_iter()
            .enumerate()
            .map(|(i, name)| (name, i))
            .collect();

        Self { names, mobs }
    }
}

#[derive(Debug, Deserialize)]
pub struct MobData {
    sprite: String,
    #[serde(default = "default_blocks_movement")]
    blocks_movement: bool,
    #[serde(alias = "HP", alias = "hit_points")]
    hp: u16,
    defense: u16,
    power: u16,
}

impl MobData {
    pub fn spawn(&self, commands: &mut Commands, asset_server: &AssetServer) -> Entity {
        let mut ec = commands.spawn((
            SpriteBundle {
                texture: asset_server.load(self.sprite()),
                ..Default::default()
            },
            HP::new(self.hp),
            Defense(self.defense),
            Power(self.power),
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
    mut monster_qry: Query<(Entity, &mut Transform), With<Mob>>,
    player_qry: Query<(Entity, &Transform), (With<Player>, Without<Mob>)>,
    mut attack: EventWriter<AttackEvent>,
    walkable_qry: Query<&Transform, (Without<BlocksMovement>, Without<Mob>)>,
) {
    // Get the player's position first to avoid looking this up repeatedly
    if let Ok((player, player_pos)) = player_qry.get_single() {
        let player_tile = TilePos::from(player_pos);

        let walkable: HashSet<_> = walkable_qry.iter().map(TilePos::from).collect();

        for (monster, mut monster_pos) in monster_qry.iter_mut() {
            let monster_tile = TilePos::from(*monster_pos);

            if let Some(tile_entity) = map.get(monster_tile) {
                if let Ok(&FieldOfView::Visible) = fov_qry.get(tile_entity) {
                    if monster_tile.distance(player_tile) <= 1 {
                        attack.send(AttackEvent::new(monster, player));
                    } else if let Some((path, _)) = pathfinding::directed::astar::astar(
                        &monster_tile,
                        |tile| {
                            map.neighbors_of(*tile).into_iter().filter_map(|tile| {
                                if walkable.contains(&tile) {
                                    Some((tile, 1))
                                } else {
                                    None
                                }
                            })
                        },
                        |tile| tile.distance(player_tile),
                        |tile| *tile == player_tile,
                    ) {
                        *monster_pos = path[1].as_transform(monster_pos.translation.z);
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct MobsPlugin;

impl Plugin for MobsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, monster_ai.run_if(in_state(TurnState::MonsterTurn)));
    }
}
