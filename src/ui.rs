use bevy::prelude::*;

mod dungeon_ui;
pub mod messages;

pub use messages::Messages;

use crate::GameState;

#[derive(Debug)]
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Messages>()
            .add_systems(OnEnter(GameState::Running), dungeon_ui::spawn_dungeon_ui)
            .add_systems(OnExit(GameState::Running), dungeon_ui::despawn_dungeon_ui)
            .add_systems(
                Update,
                (
                    dungeon_ui::update_hp,
                    dungeon_ui::update_message_log,
                    dungeon_ui::update_looking_at,
                )
                    .run_if(in_state(GameState::Running)),
            );
    }
}
