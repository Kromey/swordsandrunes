use std::{cmp::{min, max}, ops::RangeInclusive};

use itertools::Itertools;
use rand::{SeedableRng, Rng};
use rand_xoshiro::Xoshiro512StarStar;

use crate::tiles::TilePos;


pub fn simple_tunnel(start: TilePos, end: TilePos) -> impl Iterator<Item = TilePos> {
    let x1 = start.x;
    let x2 = end.x;
    let y1 = start.y;
    let y2 = end.y;

    let (corner_x, corner_y) = if Xoshiro512StarStar::from_entropy().gen_bool(0.5) {
        (x2, y1)
    } else {
        (x1, y2)
    };

    make_range(x1, corner_x).cartesian_product(make_range(y1, corner_y))
        .chain(make_range(corner_x, x2).cartesian_product(make_range(corner_y, y2)))
        .map(TilePos::from)
}

fn make_range(from: u32, to: u32) -> RangeInclusive<u32> {
    let a = min(from, to);
    let b = max(from, to);

    a..=b
}
