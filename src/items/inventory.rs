use super::Item;
use bevy::prelude::*;

#[derive(Debug, Default, Clone, Component)]
pub struct Inventory {
    capacity: usize,
    items: Vec<Item>,
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

    pub fn items(&self) -> &[Item] {
        self.items.as_ref()
    }
}
