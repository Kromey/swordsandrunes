use bevy::{
    prelude::*,
    window::{Window, WindowPlugin, WindowMode},
    DefaultPlugins,
};

pub mod camera;
pub mod dungeon;
pub mod input_manager;
pub mod map;
pub mod movement;
pub mod setup;
pub mod tiles;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default]
    Starting,
    MainMenu,
    AssetsLoading,
    Setup,
    Running,
}

fn state_manager(state: Res<State<GameState>>, mut next_state: ResMut<NextState<GameState>>) {
    // FIXME: Temporary system to "skip" states we're not utilizing yet
    #[allow(clippy::single_match)]
    match **state {
        // GameState::Starting => next_state.set(GameState::MainMenu),
        GameState::MainMenu => next_state.set(GameState::AssetsLoading),
        GameState::AssetsLoading => next_state.set(GameState::Setup), // FIXME: Load assets at startup
        // GameState::Setup => next_state.set(GameState::Running),
        _ => {}
    };

    if let Some(next) = next_state.0 {
        println!("Switching from {state:?} to state {next:?}");
    }
}

/// Run the game
pub fn run() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Swords and Runes".to_string(),
                mode: WindowMode::BorderlessFullscreen,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Update, bevy::window::close_on_esc)
        // Begin game configuration
        .add_state::<GameState>()
        .add_systems(Update, state_manager)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(input_manager::InputManagerPlugin)
        .add_plugins(movement::MovementPlugin)
        .add_plugins(setup::SetupPlugin)
        .run();
}
