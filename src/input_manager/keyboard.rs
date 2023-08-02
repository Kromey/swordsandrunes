//! This module is responsible for interpreting keyboard events
//! and turning them into [`Actions`]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use bevy::prelude::*;

mod keymap;
use keymap::KeyMap;

use crate::TurnState;

use self::keymap::BoundKey;

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
    /// Walk northeast
    WalkNortheast,
    /// Walk southeast
    WalkSoutheast,
    /// Walk southwest
    WalkSouthwest,
    /// Walk northwest
    WalkNorthwest,
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

        matches!(
            *self,
            WalkNorth
                | WalkEast
                | WalkWest
                | WalkSouth
                | WalkNortheast
                | WalkSoutheast
                | WalkSouthwest
                | WalkNorthwest
        )
    }
}

/// A modifier key
#[non_exhaustive]
#[repr(usize)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum ActionModifier {
    /// Shift
    Shift = 0,
    /// Ctrl
    Ctrl = 1,
    /// Alt
    Alt = 2,
}

impl ActionModifier {
    fn key_codes(&self) -> [KeyCode; 2] {
        match *self {
            Self::Shift => [KeyCode::ShiftLeft, KeyCode::ShiftRight],
            Self::Ctrl => [KeyCode::ControlLeft, KeyCode::ControlRight],
            Self::Alt => [KeyCode::AltLeft, KeyCode::AltRight],
        }
    }
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
    bindings: HashMap<Action, Vec<keymap::BoundKey>>,
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

        // The "Big Three" modifier keys
        self.modifiers = [
            keys.any_pressed(ActionModifier::Shift.key_codes()),
            keys.any_pressed(ActionModifier::Ctrl.key_codes()),
            keys.any_pressed(ActionModifier::Alt.key_codes()),
        ];
        let any_modifier = self.modifiers.iter().any(|&pressed| pressed);

        for (&action, boundkeys) in self.bindings.iter() {
            let state = if boundkeys.is_empty() {
                false
            } else if action.is_toggle() {
                boundkeys.iter().any(|&boundkey| match boundkey {
                    BoundKey::Key(keycode) => !any_modifier && keys.just_pressed(keycode),
                    BoundKey::ModifiedKey { key, with } => {
                        self.modifier(with) && keys.just_pressed(key)
                    }
                })
            } else {
                boundkeys.iter().any(|&boundkey| match boundkey {
                    BoundKey::Key(key) => !any_modifier && keys.pressed(key),
                    BoundKey::ModifiedKey { key, with } => self.modifier(with) && keys.pressed(key),
                })
            };

            self.state.insert(action, state);
            received_player_input |= state && action.ends_turn();
        }

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
        next_state.set(TurnState::PlayerTurn);
    }
}
