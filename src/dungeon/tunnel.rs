use std::cmp::{min, max};

use itertools::Itertools;
use rand::{SeedableRng, Rng};
use rand_xoshiro::Xoshiro512StarStar;

use crate::tiles::TilePos;


pub fn simple_tunnel(start: TilePos, end: TilePos) -> impl Iterator<Item = TilePos> {
    let x1 = min(start.x, end.x);
    let x2 = max(start.x, end.x);
    let y1 = min(start.y, end.y);
    let y2 = max(start.y, end.y);

    let (corner_x, corner_y) = if Xoshiro512StarStar::from_entropy().gen_bool(0.5) {
        (x2, y1)
    } else {
        (x1, y2)
    };

    (x1..=corner_x).cartesian_product(y1..=corner_y)
        .chain((corner_x..=x2).cartesian_product(corner_y..=y2))
        .map(TilePos::from)
}
