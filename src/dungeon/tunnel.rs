use crate::{dungeon::TilePos, rand::prelude::*};
use itertools::Itertools;
use std::cmp::{max, min};

pub fn simple_tunnel(
    start: TilePos,
    end: TilePos,
    rng: &mut Random,
) -> impl Iterator<Item = TilePos> {
    let x1 = start.x;
    let x2 = end.x;
    let y1 = start.y;
    let y2 = end.y;

    let (corner_x, corner_y) = if rng.gen_bool(0.5) {
        (x2, y1)
    } else {
        (x1, y2)
    };

    make_range(x1, corner_x)
        .cartesian_product(make_range(y1, corner_y))
        .chain(make_range(corner_x, x2).cartesian_product(make_range(corner_y, y2)))
        .map(TilePos::from)
}

fn make_range(from: u32, to: u32) -> impl Iterator<Item = u32> + Clone {
    let a = min(from, to);
    let b = max(from, to);

    a..=b
}
