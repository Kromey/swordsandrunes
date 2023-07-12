use bevy::prelude::*;

use crate::map::MapSize;

use super::TILE_SIZE;

/// Position in terms of tiles
#[derive(Debug, Default, Clone, Copy, Component)]
pub struct TilePos {
    pub x: u32,
    pub y: u32,
}

impl TilePos {
    pub fn from_index(idx: usize, size: MapSize) -> Self {
        Self {
            x: idx as u32 % size.width,
            y: idx as u32 / size.width,
        }
    }

    pub fn as_vec(&self) -> Vec2 {
        Vec2 {
            x: (self.x * TILE_SIZE) as f32,
            y: (self.y * TILE_SIZE) as f32,
        }
    }

    pub fn as_transform(&self, z: f32) -> Transform {
        Transform::from_translation(self.as_vec().extend(z))
    }
}

impl From<Transform> for TilePos {
    fn from(value: Transform) -> Self {
        Self::from(value.translation)
    }
}

impl From<Vec3> for TilePos {
    fn from(value: Vec3) -> Self {
        Self::from(value.truncate())
    }
}

impl From<Vec2> for TilePos {
    fn from(value: Vec2) -> Self {
        Self {
            x: (value.x / TILE_SIZE as f32).round() as u32,
            y: (value.y / TILE_SIZE as f32).round() as u32,
        }
    }
}

impl std::ops::Div<u32> for TilePos {
    type Output = Self;

    fn div(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::ops::DivAssign<u32> for TilePos {
    fn div_assign(&mut self, rhs: u32) {
        *self = *self / rhs;
    }
}

impl std::ops::Mul<u32> for TilePos {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::MulAssign<u32> for TilePos {
    fn mul_assign(&mut self, rhs: u32) {
        *self = *self * rhs;
    }
}
