use std::{collections::HashMap, fs::File, io::BufReader, ops::Index};

use bevy::prelude::*;
use itertools::Itertools;
use serde::Deserialize;

use crate::utils::get_dat_path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Component)]
pub struct ItemId(usize);

#[derive(Debug, Clone, Resource)]
pub struct ItemList {
    names: HashMap<String, ItemId>,
    items: Vec<ItemData>,
}

impl Index<ItemId> for ItemList {
    type Output = ItemData;

    fn index(&self, index: ItemId) -> &Self::Output {
        &self.items[index.0]
    }
}

impl Index<&ItemId> for ItemList {
    type Output = ItemData;

    fn index(&self, index: &ItemId) -> &Self::Output {
        &self[*index]
    }
}

impl ItemList {
    pub fn from_raws() -> Self {
        let path = get_dat_path("items.yaml");
        let reader = BufReader::new(File::open(path).unwrap());

        let mut items = serde_yaml::Deserializer::from_reader(reader)
            .map(|document| ItemData::deserialize(document).unwrap())
            .collect_vec();
        // Ensure our item list is sorted, which makes our item IDs sortable in the same order
        items.sort_unstable();

        let names = items
            .iter()
            .enumerate()
            .map(|(i, item)| (item.name.to_lowercase(), ItemId(i)))
            .collect();

        Self { names, items }
    }

    pub fn get<S: AsRef<str>>(&self, item_name: S) -> ItemId {
        *self.names.get(&item_name.as_ref().to_lowercase()).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
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
