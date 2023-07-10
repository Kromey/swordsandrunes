use bevy::{sprite::TextureAtlas, prelude::*, asset::{LoadState, HandleId}};

use crate::GameState;

#[derive(Debug, Clone, Resource)]
pub struct SpriteCollection {
    pub characters: Handle<TextureAtlas>,
}

#[derive(Debug, Default, Clone, Resource, Deref, DerefMut)]
pub struct SpriteHandles(Vec<HandleId>);

fn init_collection(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    server: Res<AssetServer>,
) {
    let mut handles = SpriteHandles::default();

    let characters_img = server.load("characters.png");
    let characters_atlas = TextureAtlas::from_grid(
        characters_img.clone(),
        Vec2::splat(16.),
        54,
        12,
        Some(Vec2::splat(1.)),
        None,
    );
    let characters = texture_atlases.add(characters_atlas);
    handles.push(characters_img.id());

    commands.insert_resource(
        SpriteCollection {
            characters,
        }
    );
    commands.insert_resource(handles);
}

fn check_assets_ready(
    handles: Res<SpriteHandles>,
    server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if LoadState::Loaded == server.get_group_load_state(handles.iter().copied()) {
        println!("All assets are loaded!");
        next_state.set(GameState::Setup);
    } else {
        print!(".");
    }
}


pub struct SpritesPlugin;

impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_collection)
            .add_systems(Update, check_assets_ready.run_if(in_state(GameState::AssetsLoading)));
    }
}
