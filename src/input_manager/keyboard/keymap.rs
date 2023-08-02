//! The Keymap data structure stores mapping of actions to their bound keys

use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::Action;

/// The default keymap
const DEFAULT_KEYMAP: &str = include_str!("default_keymap.yaml");

/// A representation of bound keycodes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BoundKey {
    /// No key has been bound
    Unbound,
    /// A single key has been bound
    Single(KeyCode),
    /// Multiple keys have been bound
    Multi(Vec<KeyCode>),
}

impl From<&BoundKey> for Vec<KeyCode> {
    fn from(value: &BoundKey) -> Self {
        match value {
            BoundKey::Unbound => Vec::new(),
            BoundKey::Single(key) => vec![*key],
            BoundKey::Multi(keys) => keys.clone(),
        }
    }
}

impl From<BoundKey> for Vec<KeyCode> {
    fn from(value: BoundKey) -> Self {
        Self::from(&value)
    }
}

/// A set of all actions and their keybindings
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyMap {
    /// The collection of actions and keybindings
    boundkeys: HashMap<Action, BoundKey>,
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
    pub fn action_keys(&self) -> HashMap<Action, Vec<KeyCode>> {
        self.boundkeys
            .iter()
            .map(|(action, keys)| (*action, keys.into()))
            .collect()
    }
}
