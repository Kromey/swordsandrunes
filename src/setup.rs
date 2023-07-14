use bevy::prelude::*;

use crate::{GameState, tiles::TilePos, dungeon::generate_dungeon};

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct Player;

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct PrimaryCamera;

fn setup_camera(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    commands.spawn((Camera2dBundle::default(), PrimaryCamera));

    next_state.set(GameState::MainMenu);
}

/// Update the camera's position when the player moves
/// 
/// FIXME: This is really janky as there may be a 1-frame delay, resulting in flickering
#[allow(clippy::type_complexity)]
fn camera_follow_player(
    mut camera_qry: Query<&mut Transform, With<PrimaryCamera>>,
    player_qry: Query<&Transform, (With<Player>, Without<PrimaryCamera>, Changed<Transform>)>,
) {
    if let Ok(mut camera_transform) = camera_qry.get_single_mut() {
        if let Ok(player_transform) = player_qry.get_single() {
            camera_transform.translation = player_transform.translation.truncate().extend(camera_transform.translation.z);
        }
    }
}

fn setup_game(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    mut camera: Query<&mut Transform, With<PrimaryCamera>>,
) {
    let width = 80;
    let height = 45;

    let (map, player_start) = generate_dungeon(width, height, &mut commands, &asset_server);

    if let Ok(mut transform) = camera.get_single_mut() {
        transform.translation = map.size.center().extend(transform.translation.z);
    }

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("orc.png"),
            transform: (map.size.center_tile() - TilePos::new(5, 0)).as_transform(1.0),
            ..Default::default()
        },
        Name::new("Orc"),
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("human_adventurer.png"),
            transform: player_start.as_transform(1.0),
            ..Default::default()
        },
        Name::new("The Player"),
        Player,
    ));

    commands.insert_resource(map);
    next_state.set(GameState::Running);
}

#[derive(Debug, Default)]
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup_camera.run_if(in_state(GameState::Starting)))
            .add_systems(Update, setup_game.run_if(in_state(GameState::Setup)))
            .add_systems(Update, camera_follow_player);
    }
}
