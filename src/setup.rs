use bevy::prelude::*;

use crate::{GameState, sprites::SpriteCollection};

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct Player;

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct MainCamera;

fn setup_camera(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
    ));

    next_state.set(GameState::MainMenu);
}

fn setup_game(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    sprite_collection: Res<SpriteCollection>,
) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: sprite_collection.characters.clone(),
            sprite: TextureAtlasSprite { index: 378, ..Default::default() },
            transform: Transform::from_translation(Vec3::ZERO),
            ..Default::default()
        },
        Player,
    ));

    next_state.set(GameState::Running);
}

#[derive(Debug, Default)]
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup_camera.run_if(in_state(GameState::Starting)))
            .add_systems(Update, setup_game.run_if(in_state(GameState::Setup)));
    }
}
