use bevy::prelude::*;

use crate::{camera::PrimaryCamera, dungeon::generate_dungeon, GameState};

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct Player;

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
        app.add_systems(Update, setup_game.run_if(in_state(GameState::Setup)));
    }
}
