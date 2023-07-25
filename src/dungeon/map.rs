use bevy::prelude::*;

use crate::{
    dungeon::{BlocksMovement, BlocksSight, RectangularRoom, RoomGraph, Tile, TileBundle, TilePos},
    fieldofview::FieldOfView,
    input_manager::{Action, ActionModifier, Actions},
};

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
        TilePos {
            x: self.width,
            y: self.height,
        }
        .as_vec()
            / 2.0
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
    pub rooms: RoomGraph,
}

impl Map {
    pub fn new(
        width: u32,
        height: u32,
        commands: &mut Commands,
        asset_server: &AssetServer,
    ) -> Self {
        let size = MapSize::new(width, height);
        let tiles = (0..size.len())
            .map(|i| {
                let pos = TilePos::from_index(i as usize, size);
                commands
                    .spawn((
                        TileBundle::wall(),
                        Tile::sprite_bundle(pos, asset_server.load("sprites/catacombs2.png")),
                    ))
                    .id()
            })
            .collect();

        Self {
            tiles,
            size,
            rooms: RoomGraph::default(),
        }
    }

    pub fn get(&self, pos: TilePos) -> Option<Entity> {
        if self.size.in_bounds(pos) {
            let idx = pos.as_index(self.size);
            Some(self.tiles[idx])
        } else {
            None
        }
    }

    pub fn neighbors_of(&self, pos: TilePos) -> Vec<TilePos> {
        let mut neighbors = Vec::with_capacity(8);

        let north = pos + TilePos::new(0, 1);
        if self.size.in_bounds(north) {
            neighbors.push(north);
        }
        let east = pos + TilePos::new(1, 0);
        if self.size.in_bounds(east) {
            neighbors.push(east);
        }
        // South
        if pos.y > 0 {
            neighbors.push(pos - TilePos::new(0, 1));
        }
        // West
        if pos.x > 0 {
            neighbors.push(pos - TilePos::new(1, 0));
        }

        let northeast = pos + TilePos::new(1, 1);
        if self.size.in_bounds(northeast) {
            neighbors.push(northeast);
        }

        // Southeast
        if pos.y > 0 {
            let southeast = pos - TilePos::new(0, 1) + TilePos::new(1, 0);
            if self.size.in_bounds(southeast) {
                neighbors.push(southeast);
            }
        }
        // Southwest
        if pos.y > 0 && pos.x > 0 {
            neighbors.push(pos - TilePos::new(1, 1));
        }
        // Northwest
        if pos.x > 0 {
            let northwest = pos - TilePos::new(1, 0) + TilePos::new(0, 1);
            if self.size.in_bounds(northwest) {
                neighbors.push(northwest);
            }
        }

        neighbors
    }

    pub fn iter_rooms(&self) -> impl Iterator<Item = &RectangularRoom> {
        self.rooms.rooms()
    }

    pub fn add_room(
        &self,
        room: RectangularRoom,
        commands: &mut Commands,
        asset_server: &AssetServer,
    ) {
        let floor_texture: Handle<Image> = asset_server.load("sprites/tomb0.png");

        for pos in room.iter() {
            if let Some(tile) = self.get(pos) {
                commands
                    .entity(tile)
                    .remove::<(BlocksMovement, BlocksSight)>()
                    .insert((TileBundle::floor(), floor_texture.clone()));
            }
        }
    }

    pub fn add_tunnel(
        &self,
        tunnel: impl Iterator<Item = TilePos>,
        commands: &mut Commands,
        asset_server: &AssetServer,
    ) {
        let floor_texture: Handle<Image> = asset_server.load("sprites/tomb0.png");

        for pos in tunnel {
            if let Some(tile) = self.get(pos) {
                commands
                    .entity(tile)
                    .remove::<(BlocksMovement, BlocksSight)>()
                    .insert((TileBundle::floor(), floor_texture.clone()));
            }
        }
    }
}

pub fn reveal_map(actions: Res<Actions>, mut tiles: Query<&mut FieldOfView, With<Tile>>) {
    if actions.perform(Action::RevealMap) && actions.modifier(ActionModifier::Alt) {
        for mut fov in tiles.iter_mut() {
            if *fov == FieldOfView::Unexplored {
                *fov = FieldOfView::NotVisible;
            }
        }
    }
}
