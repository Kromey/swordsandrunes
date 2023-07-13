use bevy::prelude::*;

use crate::{GameState, map::{MapSize, Map}, tiles::{TilePos, TileBundle}};

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
    asset_server: Res<AssetServer>,
    mut camera: Query<&mut Transform, With<PrimaryCamera>>,
) {
    let width = 80;
    let height = 45;

    let size = MapSize::new(width, height);
    let tiles = (0..size.len()).map(|i| {
        let pos = TilePos::from_index(i as usize, size);
        commands.spawn(TileBundle::floor(pos.x, pos.y, asset_server.load("tomb0.png"))).id()
    }).collect();

    let map = Map { tiles, size, };

    let walls = TileBundle::wall(0, 0, asset_server.load("catacombs2.png"));
    for i in 30..33 {
        let pos = TilePos::new(i, 22);
        if let Some(tile) = map.get(pos) {
            println!("{pos:?} => {} => {:?}", pos.as_index(size), TilePos::from_index(pos.as_index(size), size));
            commands.entity(tile).insert((walls.walkable, walls.transparent, walls.name.clone(), walls.texture.clone()));
        }
    }

    commands.insert_resource(map);

    if let Ok(mut transform) = camera.get_single_mut() {
        transform.translation = size.center().extend(transform.translation.z);
    }

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("orc.png"),
            transform: (size.center_tile() - TilePos::new(5, 0)).as_transform(1.0),
            ..Default::default()
        },
        Name::new("Orc"),
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("human_adventurer.png"),
            transform: size.center_tile().as_transform(1.0),
            ..Default::default()
        },
        Name::new("The Player"),
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
