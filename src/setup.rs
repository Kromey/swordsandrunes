use bevy::prelude::*;

use crate::{
    camera::PrimaryCamera,
    combat::HP,
    dungeon::generate_dungeon,
    fieldofview::HideOutOfSight,
    items::{Inventory, ItemList},
    mobs::MobList,
    rand::prelude::*,
    stats::{Attributes, Skill, SkillSheet},
    utils::SpriteLayer,
    GameState, TurnState,
};

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct Player;

fn setup_game(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    mut camera: Query<&mut Transform, With<PrimaryCamera>>,
    mob_list: Res<MobList>,
    item_list: Res<ItemList>,
    random: Res<Random>,
) {
    let width = 80;
    let height = 45;

    // === Generate Dungeon ===
    let (map, player_start) = generate_dungeon(
        width,
        height,
        &mut commands,
        &asset_server,
        random.from_entropy(),
    );

    // === Center Camera ===
    if let Ok(mut transform) = camera.get_single_mut() {
        transform.translation = map.size.center().extend(transform.translation.z);
    }

    // === Spawn Monsters ===
    let mut rng = random.from_entropy();
    for room in map.iter_rooms() {
        let n = rng.gen_range(0..=3);
        for tile in room.iter().choose_multiple(&mut rng, n) {
            let entity = if rng.gen_bool(0.2) {
                mob_list.spawn("Ogre", &mut commands, &asset_server)
            } else {
                mob_list.spawn("Orc", &mut commands, &asset_server)
            };
            commands
                .entity(entity)
                .insert(tile.as_transform(SpriteLayer::Actor));
        }
    }

    // === Spawn Potions ===
    let potion = item_list.get("Potion of Healing");
    for room in map.iter_rooms() {
        let n = rng.gen_range(0..=5).clamp(3, 5) - 3; // 0-3 = 0; 4-5 = 1-2
        for tile in room.iter().choose_multiple(&mut rng, n) {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("sprites/items/potions/brilliant_blue.png"),
                    transform: tile.as_transform(SpriteLayer::Item),
                    ..Default::default()
                },
                Name::new(potion.name.clone()),
                HideOutOfSight::Hide,
                potion.data,
            ));
        }
    }

    // === Spawn Player ===
    let mut skills = SkillSheet::new();
    skills.set("Defense", Skill::new(12));
    skills.set("Attack", Skill::new(15));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/human_adventurer.png"),
            transform: player_start.as_transform(SpriteLayer::Actor),
            ..Default::default()
        },
        HP::new(30),
        skills,
        Name::new("Player"),
        Attributes {
            strength: 12,
            dexterity: 12,
            intelligence: 12,
            perception: 12,
        },
        Inventory::new(26),
        Player,
    ));

    // === Update Game State ===
    commands.insert_resource(map);
    next_state.set(GameState::Running);
}

fn load_raws(mut commands: Commands) {
    commands.insert_resource(MobList::from_raws());
    commands.insert_resource(ItemList::from_raws());
}

/// Ensure the game starts ready for the player to choose their first action
fn start_turn(mut next_state: ResMut<NextState<TurnState>>) {
    next_state.set(TurnState::WaitingForPlayer);
}

#[derive(Debug, Default)]
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_raws)
            .add_systems(Update, setup_game.run_if(in_state(GameState::Setup)))
            .add_systems(OnExit(GameState::Setup), start_turn);
    }
}
