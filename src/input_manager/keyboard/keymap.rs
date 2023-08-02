//! The Keymap data structure stores mapping of actions to their bound keys

use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::{Action, ActionModifier};

/// The default keymap
const DEFAULT_KEYMAP: &str = include_str!("default_keymap.yaml");

/// A bound keycode with a modifier key (Shift/Ctrl/Alt)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BoundKey {
    Key(KeyCode),
    ModifiedKey { key: KeyCode, with: ActionModifier },
}

/// A representation of bound keycodes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActionKeys {
    /// No key has been bound
    Unbound,
    /// A single key has been bound
    Single(BoundKey),
    /// Multiple keys have been bound
    Multi(Vec<BoundKey>),
}

impl From<&ActionKeys> for Vec<BoundKey> {
    fn from(value: &ActionKeys) -> Self {
        match value {
            ActionKeys::Unbound => Vec::new(),
            ActionKeys::Single(key) => vec![*key],
            ActionKeys::Multi(keys) => keys.clone(),
        }
    }
}

impl From<ActionKeys> for Vec<BoundKey> {
    fn from(value: ActionKeys) -> Self {
        Self::from(&value)
    }
}

/// A set of all actions and their keybindings
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyMap {
    /// The collection of actions and keybindings
    boundkeys: HashMap<Action, ActionKeys>,
}

impl Default for KeyMap {
    fn default() -> Self {
        Self {
            boundkeys: serde_yaml::from_str(DEFAULT_KEYMAP).unwrap(),
        }
    }
}

impl KeyMap {
    /// Retrieve a map of actions to their bound keys
    pub fn action_keys(&self) -> HashMap<Action, Vec<BoundKey>> {
        self.boundkeys
            .iter()
            .map(|(action, keys)| (*action, keys.into()))
            .collect()
    }
}
