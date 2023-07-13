use bevy::prelude::*;

use crate::tiles::TilePos;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapSize {
    pub width: u32,
    pub height: u32,
}

impl MapSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn len(&self) -> u32 {
        self.width * self.height
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn center(&self) -> Vec2 {
        TilePos { x: self.width, y: self.height }.as_vec() / 2.0
    }

    pub fn center_tile(&self) -> TilePos {
        TilePos::from(self.center())
    }

    pub fn in_bounds(&self, pos: TilePos) -> bool {
        pos.x < self.width && pos.y < self.height
    }
}

impl Default for MapSize {
    fn default() -> Self {
        Self {
            width: 32,
            height: 32,
        }
    }
}

#[derive(Debug, Default, Resource)]
pub struct Map {
    pub tiles: Vec<Entity>,
    pub size: MapSize,
}

impl Map {
    pub fn get(&self, pos: TilePos) -> Option<Entity> {
        if self.size.in_bounds(pos) {
            let idx = pos.as_index(self.size);
            Some(self.tiles[idx])
        } else {
            None
        }
    }
}
