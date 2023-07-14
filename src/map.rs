use bevy::prelude::*;

use crate::{tiles::{TilePos, TileBundle, Tile}, dungeon::RectangularRoom};

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
    pub fn new(width: u32, height: u32, commands: &mut Commands, asset_server: &AssetServer) -> Self {
        let size = MapSize::new(width, height);
        let tiles = (0..size.len()).map(|i| {
            let pos = TilePos::from_index(i as usize, size);
            commands.spawn((
                TileBundle::wall(),
                Tile::sprite_bundle(pos, asset_server.load("catacombs2.png")),
            )).id()
        }).collect();
    
        Self { tiles, size, }
    }

    pub fn get(&self, pos: TilePos) -> Option<Entity> {
        if self.size.in_bounds(pos) {
            let idx = pos.as_index(self.size);
            Some(self.tiles[idx])
        } else {
            None
        }
    }

    pub fn add_room(&self, room: RectangularRoom, commands: &mut Commands, asset_server: &AssetServer) {
        let floor = TileBundle::floor();
        let floor_texture: Handle<Image> = asset_server.load("tomb0.png");

        for pos in room.iter() {
            if let Some(tile) = self.get(pos) {
                commands.entity(tile).insert((floor.clone(), floor_texture.clone()));
            }
        }
    }

    pub fn add_tunnel(&self, tunnel: impl Iterator<Item = TilePos>, commands: &mut Commands, asset_server: &AssetServer) {
        let floor = TileBundle::floor();
        let floor_texture: Handle<Image> = asset_server.load("tomb0.png");

        for pos in tunnel {
            if let Some(tile) = self.get(pos) {
                commands.entity(tile).insert((floor.clone(), floor_texture.clone()));
            }
        }
    }
}
