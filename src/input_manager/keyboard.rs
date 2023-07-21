//! This module is responsible for interpreting keyboard events
//! and turning them into [`Actions`]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use bevy::prelude::*;

mod keymap;
use keymap::KeyMap;

use crate::TurnState;

/// A game action that can be bound to a key
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum Action {
    /// Reveal the entire map
    RevealMap,
    /// Toggle showing the debug menu
    ToggleDebug,
    /// Walk north
    WalkNorth,
    /// Walk east
    WalkEast,
    /// Walk south
    WalkSouth,
    /// Walk west
    WalkWest,
    /// Zoom out
    ZoomOut,
    /// Zoom In
    ZoomIn,
}

impl Action {
    /// Returns true if this Action should only respond to `just_pressed` events
    const fn is_toggle(&self) -> bool {
        // use Action::*;

        // matches!(self, ToggleDebug)

        true
    }

    /// Returns true if this Action ends the player's turn
    const fn ends_turn(&self) -> bool {
        use Action::*;

        matches!(*self, WalkNorth | WalkEast | WalkWest | WalkSouth)
    }
}

/// A modifier key
#[non_exhaustive]
#[repr(usize)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ActionModifier {
    /// Shift
    Shift = 0,
    /// Ctrl
    Ctrl = 1,
    /// Alt
    Alt = 2,
}

/// Game actions that can be performed
#[derive(Debug, Resource)]
pub struct Actions {
    /// Current state of each action
    state: HashMap<Action, bool>,
    /// Active modifier keys: Shift, Ctrl, Alt
    ///
    /// Note that no distinction is made between left and right keys.
    modifiers: [bool; 3],
    /// Current keybindings
    bindings: HashMap<Action, Vec<KeyCode>>,
}

impl Actions {
    /// Whether or not to perform an action
    pub fn perform(&self, action: Action) -> bool {
        self.state.get(&action).copied().unwrap_or(false)
    }

    /// The status of a modifier key
    pub const fn modifier(&self, modifier: ActionModifier) -> bool {
        self.modifiers[modifier as usize]
    }

    /// Update actions state from current keyboard input
    fn update(&mut self, keys: &Input<KeyCode>) -> bool {
        let mut received_player_input = false;

        for (&action, boundkeys) in self.bindings.iter() {
            let state = if boundkeys.is_empty() {
                false
            } else if action.is_toggle() {
                keys.any_just_pressed(boundkeys.iter().copied())
            } else {
                keys.any_pressed(boundkeys.iter().copied())
            };

            self.state.insert(action, state);
            received_player_input |= state && action.ends_turn();
        }

        // The "Big Three" modifier keys
        self.modifiers[0] = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
        self.modifiers[1] = keys.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
        self.modifiers[2] = keys.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]);

        received_player_input
    }
}

impl Default for Actions {
    fn default() -> Self {
        Self {
            state: HashMap::new(),
            modifiers: [false; 3],
            bindings: KeyMap::default().action_keys(),
        }
    }
}

/// Update the [`Actions`] resource based on key presses
pub fn update_actions(
    mut actions: ResMut<Actions>,
    keys: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<TurnState>>,
) {
    if actions.update(keys.as_ref()) {
        next_state.0 = Some(TurnState::PlayerTurn);
    }
}
