use crate::rand::prelude::*;
use bevy::prelude::*;

mod map;
pub use map::{Map, MapSize};
mod room;
pub use room::{RectangularRoom, RoomGraph};
mod tiles;
pub use tiles::{BlocksMovement, BlocksSight, Tile, TileBundle, TilePos, TILE_SIZE, TILE_SIZE_F32};
mod tunnel;
pub use tunnel::simple_tunnel;

pub fn generate_dungeon(
    width: u32,
    height: u32,
    commands: &mut Commands,
    asset_server: &AssetServer,
    mut rng: Random,
) -> (Map, TilePos) {
    let max_room_size = 10;
    let min_room_size = 6;
    let max_rooms = 30;

    let mut room_list: Vec<RectangularRoom> = Vec::new();

    let mut map = Map::new(width, height, commands, asset_server);
    let mut player_start = map.size.center_tile();

    for _ in 0..max_rooms {
        let room_width = rng.gen_range(min_room_size..=max_room_size);
        let room_height = rng.gen_range(min_room_size..=max_room_size);

        let x = rng.gen_range(0..(width - room_width));
        let y = rng.gen_range(0..(height - room_height));

        let new_room = RectangularRoom::new(TilePos::new(x, y), room_width, room_height);

        if room_list.iter().any(|room| room.intersects(new_room)) {
            continue;
        }

        map.add_room(new_room, commands, asset_server);
        if room_list.is_empty() {
            // First room, good place to start the player? Sure! Why not?
            player_start = new_room.center();
        }

        room_list.push(new_room);
    }

    let mut rooms = RoomGraph::from_rooms(&room_list);
    rooms.triangulate();
    rooms.to_min_spanning_tree();

    for (a, b) in rooms.edges() {
        map.add_tunnel(
            simple_tunnel(a.center(), b.center(), &mut rng),
            commands,
            asset_server,
        );
    }

    map.rooms = rooms;

    (map, player_start)
}

#[derive(Debug, Default)]
pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, map::reveal_map);
    }
}
