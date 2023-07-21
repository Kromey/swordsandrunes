use bevy::{
    prelude::*,
    window::{Window, WindowMode, WindowPlugin},
    DefaultPlugins,
};

pub mod bump;
pub mod camera;
pub mod combat;
pub mod dungeon;
pub mod fieldofview;
pub mod input_manager;
pub mod mobs;
pub mod movement;
pub mod setup;
pub mod utils;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default]
    Starting,
    MainMenu,
    AssetsLoading,
    Setup,
    Running,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum TurnState {
    #[default]
    WaitingForPlayer,
    PlayerTurn,
    MonsterTurn,
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
        info!("Switching from {state:?} to state {next:?}");
    }
}

fn skip_monster_turn(mut next_state: ResMut<NextState<TurnState>>) {
    warn!("Skipping monster turn!");
    next_state.0 = Some(TurnState::WaitingForPlayer);
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
        .add_state::<TurnState>()
        .add_systems(
            Update,
            (
                state_manager,
                skip_monster_turn.run_if(in_state(TurnState::MonsterTurn)),
                fieldofview::update_fov.run_if(in_state(GameState::Running)),
            ),
        )
        .add_plugins((
            bump::BumpPlugin,
            camera::CameraPlugin,
            combat::CombatPlugin,
            dungeon::DungeonPlugin,
            input_manager::InputManagerPlugin,
            movement::MovementPlugin,
            setup::SetupPlugin,
        ))
        .run();
}
