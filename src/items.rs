use std::{collections::HashMap, fs::File, io::BufReader};

use bevy::prelude::*;
use serde::Deserialize;

use crate::utils::get_dat_path;

pub mod inventory;

#[derive(Debug, Clone, Resource)]
pub struct ItemList {
    items: HashMap<String, Item>,
}

impl ItemList {
    pub fn from_raws() -> Self {
        let path = get_dat_path("items.yaml");
        let reader = BufReader::new(File::open(path).unwrap());

        Self {
            items: serde_yaml::Deserializer::from_reader(reader)
                .map(|document| {
                    let item = Item::deserialize(document).unwrap();
                    (item.name.to_lowercase(), item)
                })
                .collect(),
        }
    }

    pub fn get<S: AsRef<str>>(&self, item_name: S) -> &Item {
        self.items.get(&item_name.as_ref().to_lowercase()).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Component, Deserialize)]
pub struct Item {
    name: String,
    #[serde(flatten)]
    data: ItemData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ItemData {
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
