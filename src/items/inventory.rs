use super::ItemData;
use bevy::prelude::*;

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
