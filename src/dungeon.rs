use bevy::prelude::*;

pub mod room;
pub use room::*;

use crate::{map::Map, tiles::TilePos};

pub fn generate_dungeon(width: u32, height: u32, commands: &mut Commands, asset_server: &AssetServer) -> Map {
    let map = Map::new(width, height, commands, asset_server);

    let room1 = RectangularRoom::new(TilePos::new(20, 15), 10, 15);
    let room2 = RectangularRoom::new(TilePos::new(35, 15), 10, 15);

    map.add_room(room1, commands, asset_server);
    map.add_room(room2, commands, asset_server);

    map
}
