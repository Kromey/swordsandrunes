use bevy::prelude::*;

use crate::{sprites::SpriteCollection, GameState, map::{MapSize, Map}, tiles::{TilePos, TileBundle}};

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct Player;

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct PrimaryCamera;

fn setup_camera(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    commands.spawn((Camera2dBundle::default(), PrimaryCamera));

    next_state.set(GameState::MainMenu);
}

fn setup_game(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    sprite_collection: Res<SpriteCollection>,
    mut camera: Query<&mut Transform, With<PrimaryCamera>>,
) {
    let width = 80;
    let height = 45;

    let size = MapSize::new(width, height);
    let tiles = (0..size.len()).map(|i| {
        let pos = TilePos::from_index(i as usize, size);
        commands.spawn(TileBundle::floor(pos.x, pos.y, sprite_collection.objects.clone())).id()
    }).collect();

    commands.insert_resource(Map {
        tiles,
        size,
    });

    if let Ok(mut transform) = camera.get_single_mut() {
        transform.translation = size.center().extend(transform.translation.z);
    }

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: sprite_collection.characters.clone(),
            sprite: TextureAtlasSprite {
                index: 378,
                ..Default::default()
            },
            transform: size.center_tile().as_transform(1.0),
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
