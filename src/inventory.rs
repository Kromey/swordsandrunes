use std::collections::HashSet;

use crate::{
    dungeon::TilePos,
    dungeon_ui::Messages,
    items::{Item, ItemData, ItemList},
    setup::Player,
};
use bevy::{ecs::query::Has, prelude::*};

#[derive(Debug, Default, Clone, Component)]
pub struct Inventory {
    capacity: usize,
    items: Vec<ItemData>,
}

impl Inventory {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            items: Vec::with_capacity(capacity),
        }
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

    pub fn items(&self) -> &[ItemData] {
        self.items.as_ref()
    }

    pub fn insert(&mut self, item: ItemData) -> bool {
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
    items_qry: Query<(Entity, &Transform, &Name), With<Item>>,
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

        for (item, item_pos, item_name) in items_qry.iter() {
            let item_tile = TilePos::from(item_pos);
            if tile == item_tile && !picked_up.contains(&item) {
                picked_up.insert(item);
                inventory.insert(item_list.get(item_name.as_str()).clone());
                commands.entity(item).despawn();
                if is_player {
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
