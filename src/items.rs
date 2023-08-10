use std::{collections::HashMap, fs::File, io::BufReader, ops::Index};

use bevy::prelude::*;
use itertools::Itertools;
use serde::Deserialize;

use crate::{combat::HP, utils::get_dat_path};

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

impl ItemData {
    pub fn is_consumable(&self) -> bool {
        self.data.is_consumable()
    }
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

impl Item {
    pub fn is_consumable(&self) -> bool {
        matches!(self, Item::Potion { .. } | Item::Scroll { .. })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Effect {
    Heal(u16),
    Harm(u16),
}

#[derive(Debug, Clone, Copy, Event)]
pub struct UseItem {
    pub item: ItemId,
    pub user: Entity,
}

fn use_item(
    item_list: Res<ItemList>,
    mut use_item_evt: EventReader<UseItem>,
    mut health_qry: Query<&mut HP>,
) {
    for event in use_item_evt.iter() {
        if let Ok(mut hp) = health_qry.get_mut(event.user) {
            let item = item_list[event.item].data;

            match item {
                Item::Potion { effect } => apply_effect(effect, &mut hp),
                Item::Scroll { effect: _ } => todo!(),
                Item::Weapon => todo!(),
                Item::Armor => todo!(),
            }
        }
    }
}

fn apply_effect(effect: Effect, hp: &mut HP) {
    match effect {
        Effect::Heal(heal) => hp.add(heal),
        Effect::Harm(dmg) => hp.sub(dmg),
    }
}

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UseItem>().add_systems(Update, use_item);
    }
}
