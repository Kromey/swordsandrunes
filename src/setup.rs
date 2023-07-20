use bevy::prelude::*;
use rand::{seq::IteratorRandom, Rng, SeedableRng};
use rand_xoshiro::Xoshiro512StarStar;

use crate::{camera::PrimaryCamera, dungeon::generate_dungeon, mobs::MobList, GameState};

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct Player;

fn setup_game(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    mut camera: Query<&mut Transform, With<PrimaryCamera>>,
    mob_list: Res<MobList>,
) {
    let width = 80;
    let height = 45;

    let (map, player_start) = generate_dungeon(width, height, &mut commands, &asset_server);

    if let Ok(mut transform) = camera.get_single_mut() {
        transform.translation = map.size.center().extend(transform.translation.z);
    }

    let mut rng = Xoshiro512StarStar::from_entropy();
    for room in map.iter_rooms() {
        let n = rng.gen_range(0..=3);
        for tile in room.iter().choose_multiple(&mut rng, n) {
            let entity = if rng.gen_bool(0.2) {
                mob_list.spawn("Ogre", &mut commands, &asset_server)
            } else {
                mob_list.spawn("Orc", &mut commands, &asset_server)
            };
            commands.entity(entity).insert(tile.as_transform(1.0));
        }
    }

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/human_adventurer.png"),
            transform: player_start.as_transform(1.0),
            ..Default::default()
        },
        Name::new("The Player"),
        Player,
    ));

    commands.insert_resource(map);
    next_state.set(GameState::Running);
}

fn load_raws(mut commands: Commands) {
    let mobs = MobList::from_raws();
    commands.insert_resource(mobs);
}

#[derive(Debug, Default)]
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup_game.run_if(in_state(GameState::Setup)))
            .add_systems(Startup, load_raws);
    }
}
