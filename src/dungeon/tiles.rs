use bevy::prelude::*;

mod pos;
pub use pos::TilePos;

use crate::{fieldofview::FieldOfView, utils::SpriteLayer};

/// Dimensions of a tile edge; tiles are assumed to be square
pub const TILE_SIZE: u32 = 32;
/// Dimensions of a tile edge as a f32 value
pub const TILE_SIZE_F32: f32 = TILE_SIZE as f32;

/// Marker component to identify tiles
#[derive(Debug, Default, Clone, Copy, Component)]
pub struct Tile;

impl Tile {
    pub fn sprite_bundle(pos: TilePos, texture: Handle<Image>) -> SpriteBundle {
        SpriteBundle {
            texture,
            transform: pos.as_transform(SpriteLayer::Tile),

            ..Default::default()
        }
    }
}

/// Does an entity block movement?
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct BlocksMovement;

/// Does an entity block sight?
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct BlocksSight;

#[derive(Debug, Clone, Default, Bundle)]
pub struct TileBundle {
    /// Marker component indicating that this tile is, indeed, a tile
    pub tile: Tile,
    /// The tile's name
    pub name: Name,
    /// The tile's FoV status wrt the player
    pub fov: FieldOfView,
}

impl TileBundle {
    pub fn floor() -> impl Bundle {
        Self {
            name: Name::new("Stone Floor"),
            tile: Tile,
            fov: FieldOfView::Unexplored,
        }
    }

    pub fn wall() -> impl Bundle {
        (
            Self {
                name: Name::new("Stone Wall"),
                tile: Tile,
                fov: FieldOfView::Unexplored,
            },
            BlocksMovement,
            BlocksSight,
        )
    }
}

pub fn tile_fov(
    mut tile_qry: Query<(&FieldOfView, &mut Visibility, &mut Sprite), Changed<FieldOfView>>,
) {
    for (fov, mut visibility, mut sprite) in tile_qry.iter_mut() {
        match *fov {
            FieldOfView::Unexplored => *visibility = Visibility::Hidden,
            FieldOfView::NotVisible => {
                *visibility = Visibility::Visible;
                sprite.color = Color::GRAY;
            }
            FieldOfView::Visible => {
                *visibility = Visibility::Visible;
                sprite.color = Default::default();
            }
        }
    }
}
