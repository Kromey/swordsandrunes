use std::{collections::HashMap, fs::File, io::BufReader};

use bevy::prelude::*;
use serde::Deserialize;

use crate::utils::get_dat_path;

pub mod inventory;
pub use inventory::Inventory;

#[derive(Debug, Clone, Resource)]
pub struct ItemList {
    items: HashMap<String, ItemData>,
}

impl ItemList {
    pub fn from_raws() -> Self {
        let path = get_dat_path("items.yaml");
        let reader = BufReader::new(File::open(path).unwrap());

        Self {
            items: serde_yaml::Deserializer::from_reader(reader)
                .map(|document| {
                    let item = ItemData::deserialize(document).unwrap();
                    (item.name.to_lowercase(), item)
                })
                .collect(),
        }
    }

    pub fn get<S: AsRef<str>>(&self, item_name: S) -> &ItemData {
        self.items.get(&item_name.as_ref().to_lowercase()).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Component, Deserialize)]
pub struct ItemData {
    pub name: String,
    #[serde(flatten)]
    pub data: Item,
}

impl PartialOrd for ItemData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ItemData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Item {
    Potion { effect: Effect },
    Scroll { effect: Effect },
    Weapon,
    Armor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Effect {
    Heal(u16),
    Harm(u16),
}
