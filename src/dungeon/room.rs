use itertools::Itertools;

use crate::tiles::TilePos;


pub struct RectangularRoom {
    start: TilePos,
    end: TilePos,
}

impl RectangularRoom {
    pub fn new(from: TilePos, width: u32, height: u32) -> Self {
        Self {
            start: from,
            end: TilePos {
                x: from.x + width - 1, // End is inclusive, but adding width gets an exclusive point
                y: from.y + height - 1, // Ditto
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = TilePos> {
        // Iterate across the _floor_ tiles within this room
        ((self.start.x + 1)..self.end.x)
            .cartesian_product((self.start.y + 1)..self.end.y)
            .map(TilePos::from)
    }
}
