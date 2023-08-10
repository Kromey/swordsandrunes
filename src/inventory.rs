use std::{collections::HashSet, ops::Index};

use crate::{
    dungeon::TilePos,
    items::{ItemId, ItemList},
    setup::Player,
    ui::Messages,
};
use bevy::{ecs::query::Has, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Component)]
pub struct InventoryIdx(usize);

#[derive(Debug, Default, Clone, Component)]
pub struct Inventory {
    capacity: usize,
    items: Vec<ItemId>,
}

impl Index<InventoryIdx> for Inventory {
    type Output = ItemId;

    fn index(&self, index: InventoryIdx) -> &Self::Output {
        &self.items[index.0]
    }
}

impl Inventory {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            items: Vec::with_capacity(capacity),
        }
    }

    pub fn get(&self, idx: InventoryIdx) -> Option<&ItemId> {
        self.items.get(idx.0)
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.size() >= self.capacity()
    }

    pub fn items(&self) -> &[ItemId] {
        self.items.as_ref()
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (InventoryIdx, &ItemId)> {
        self.items
            .iter()
            .enumerate()
            .map(|(i, item)| (InventoryIdx(i), item))
    }

    pub fn insert(&mut self, item: ItemId) -> bool {
        if self.items.len() >= self.capacity {
            false
        } else {
            self.items.push(item);
            self.items.sort_unstable();

            true
        }
    }
}

fn autopickup(
    mut picker_upper_qry: Query<(&Transform, &mut Inventory, Has<Player>), Changed<Transform>>,
    items_qry: Query<(Entity, &Transform, &ItemId)>,
    mut commands: Commands,
    item_list: Res<ItemList>,
    mut messages: ResMut<Messages>,
) {
    let mut picked_up = HashSet::new();
    for (pos, mut inventory, is_player) in picker_upper_qry.iter_mut() {
        if inventory.is_full() {
            continue;
        }

        let tile = TilePos::from(pos);

        for (item, item_pos, item_id) in items_qry.iter() {
            let item_tile = TilePos::from(item_pos);
            if tile == item_tile && !picked_up.contains(&item) {
                picked_up.insert(item);
                inventory.insert(*item_id);
                commands.entity(item).despawn();
                if is_player {
                    let item_name = &item_list[item_id].name;
                    messages.add_friendly(format!("Picked up {item_name}"));
                }
            }
        }
    }
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, autopickup);
    }
}
