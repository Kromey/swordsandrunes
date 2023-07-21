//! Input module
//!
//! This module is responsible to managing systems to handle user input.

use bevy::prelude::*;

mod keyboard;
pub use keyboard::{Action, ActionModifier, Actions};

use crate::TurnState;

/// Label for the input manager systems to facilitate relative ordering
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct InputManager;

/// Plugin to add the input handling system into the game
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InputManagerPlugin;

impl Plugin for InputManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_systems(
            Update,
            keyboard::update_actions
                .in_set(InputManager)
                .run_if(in_state(TurnState::WaitingForPlayer)),
        );
    }
}
