use bevy::{
    prelude::{App, ClearColor, Color, PluginGroup, Update},
    window::{Window, WindowPlugin, WindowResolution},
    DefaultPlugins,
};

pub mod input_manager;

/// Initial width of the game window
const WINDOW_WIDTH: f32 = 800.;
/// Initial height of the game window
const WINDOW_HEIGHT: f32 = 600.;

/// Run the game
pub fn run() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Swords and Runes".to_string(),
                resolution: WindowResolution::from((WINDOW_WIDTH, WINDOW_HEIGHT)),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
