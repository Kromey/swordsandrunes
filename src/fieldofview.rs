//! Field of view calculated via symmetric shadowcasting
//! Ported and adapted from https://www.albertford.com/shadowcasting/

use crate::{
    dungeon::{BlocksSight, Tile, TilePos},
    mobs::Mob,
    setup::Player,
};
use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub enum FieldOfView {
    #[default]
    Unexplored,
    NotVisible,
    Visible,
}

pub fn update_fov(
    player_qry: Query<&Transform, (With<Player>, Changed<Transform>)>,
    blocks_sight_qry: Query<&Transform, With<BlocksSight>>,
    mut fov_tiles_qry: Query<(&mut FieldOfView, &Transform), With<Tile>>,
    mut mob_qry: Query<(&mut Visibility, &Transform), With<Mob>>,
) {
    if let Ok(player_transform) = player_qry.get_single() {
        let player_pos = TilePos::from(*player_transform);

        let blockers: HashSet<_> = blocks_sight_qry
            .iter()
            .map(|transform| TilePos::from(*transform))
            .collect();

        let fov = compute_fov(player_pos, |tile| blockers.contains(&tile));

        for (mut tile_fov, transform) in fov_tiles_qry.iter_mut() {
            let pos = TilePos::from(transform);
            if fov.contains(&pos) {
                *tile_fov = FieldOfView::Visible;
            } else if *tile_fov == FieldOfView::Visible {
                *tile_fov = FieldOfView::NotVisible;
            }
        }

        for (mut mob_visible, transform) in mob_qry.iter_mut() {
            if fov.contains(&TilePos::from(transform)) {
                *mob_visible = Visibility::Visible;
            } else {
                *mob_visible = Visibility::Hidden;
            }
        }
    }
}

pub fn compute_fov<F>(origin: TilePos, mut is_blocking: F) -> Vec<TilePos>
where
    F: FnMut(TilePos) -> bool,
{
    let mut visible = vec![origin];

    for i in 0..4 {
        let quadrant = Quadrant::from_index(i, origin);
        scan(Row::first(), quadrant, &mut is_blocking, &mut |pos| {
            visible.push(pos)
        });
    }

    visible
}

fn scan<F, G>(row: Row, quadrant: Quadrant, is_blocking: &mut F, mark_visible: &mut G)
where
    F: FnMut(TilePos) -> bool,
    G: FnMut(TilePos),
{
    let mut prev_tile = None;
    let mut next_row = row.next();

    for tile in row.tiles() {
        let tile_is_wall = is_blocking(quadrant.transform(tile));
        let tile_is_floor = !tile_is_wall;

        let (prev_tile_is_wall, prev_tile_is_floor) = prev_tile.unwrap_or_default();

        if tile_is_wall || is_symmetric(row, tile) {
            mark_visible(quadrant.transform(tile));
        }
        if prev_tile_is_wall && tile_is_floor {
            next_row.start_slope = slope(tile);
        }
        if prev_tile_is_floor && tile_is_wall {
            let mut next_row = next_row;
            next_row.end_slope = slope(tile);
            scan(next_row, quadrant, is_blocking, mark_visible);
        }

        prev_tile = Some((tile_is_wall, tile_is_floor));
    }

    if prev_tile.unwrap_or_default().1 {
        scan(next_row, quadrant, is_blocking, mark_visible);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Pos {
    row: i32,
    col: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Quadrant {
    North(TilePos),
    East(TilePos),
    South(TilePos),
    West(TilePos),
}

impl Quadrant {
    fn from_index(index: u32, origin: TilePos) -> Self {
        match index {
            0 => Self::North(origin),
            1 => Self::East(origin),
            2 => Self::South(origin),
            3 => Self::West(origin),
            _ => panic!(),
        }
    }

    fn transform(&self, tile: Pos) -> TilePos {
        let (x, y) = match self {
            Quadrant::North(origin) => (origin.x as i32 + tile.col, origin.y as i32 - tile.row),
            Quadrant::South(origin) => (origin.x as i32 + tile.col, origin.y as i32 + tile.row),
            Quadrant::East(origin) => (origin.x as i32 + tile.row, origin.y as i32 + tile.col),
            Quadrant::West(origin) => (origin.x as i32 - tile.row, origin.y as i32 + tile.col),
        };

        TilePos::new(x as u32, y as u32)
    }
}

#[derive(Debug, Clone, Copy)]
struct Row {
    depth: i32,
    start_slope: f64,
    end_slope: f64,
}

impl Row {
    fn first() -> Self {
        Self {
            depth: 1,
            start_slope: -1.0,
            end_slope: 1.0,
        }
    }

    fn tiles(&self) -> impl Iterator<Item = Pos> + '_ {
        let min_col = round_ties_down(self.depth as f64 * self.start_slope);
        let max_col = round_ties_up(self.depth as f64 * self.end_slope);

        (min_col..=max_col).map(|col| Pos {
            row: self.depth,
            col,
        })
    }

    fn next(&self) -> Row {
        Row {
            depth: self.depth + 1,
            ..*self
        }
    }
}

fn slope(tile: Pos) -> f64 {
    (2 * tile.col - 1) as f64 / (2 * tile.row) as f64
}

fn is_symmetric(row: Row, tile: Pos) -> bool {
    tile.col as f64 >= (row.depth as f64 * row.start_slope)
        && tile.col as f64 <= (row.depth as f64 * row.end_slope)
}

fn round_ties_up(n: f64) -> i32 {
    (n + 0.5) as i32
}

fn round_ties_down(n: f64) -> i32 {
    (n - 0.5).ceil() as i32
}
