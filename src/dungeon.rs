use bevy::prelude::*;

pub mod room;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro512StarStar;
pub use room::*;
pub mod tunnel;
pub use tunnel::*;

use crate::{map::Map, tiles::TilePos};

pub fn generate_dungeon(
    width: u32,
    height: u32,
    commands: &mut Commands,
    asset_server: &AssetServer,
) -> (Map, TilePos) {
    let max_room_size = 10;
    let min_room_size = 6;
    let max_rooms = 30;

    let mut rooms: Vec<RectangularRoom> = Vec::new();

    let mut rng = Xoshiro512StarStar::from_entropy();

    let map = Map::new(width, height, commands, asset_server);
    let mut player_start = map.size.center_tile();

    for _ in 0..max_rooms {
        let room_width = rng.gen_range(min_room_size..=max_room_size);
        let room_height = rng.gen_range(min_room_size..=max_room_size);

        let x = rng.gen_range(0..(width - room_width));
        let y = rng.gen_range(0..(height - room_height));

        let new_room = RectangularRoom::new(TilePos::new(x, y), room_width, room_height);

        if rooms.iter().any(|room| room.intersects(new_room)) {
            continue;
        }

        map.add_room(new_room, commands, asset_server);
        if rooms.is_empty() {
            // First room, good place to start the player? Sure! Why not?
            player_start = new_room.center();
        }

        rooms.push(new_room);
    }

    for i in 0..(rooms.len() - 1) {
        let nearest = rooms
            .iter()
            .skip(i + 1)
            .min_by_key(|room| rooms[i].center().distance(room.center()))
            .unwrap();
        let tunnel = simple_tunnel(rooms[i].center(), nearest.center());
        map.add_tunnel(tunnel, commands, asset_server);
    }

    (map, player_start)
}
