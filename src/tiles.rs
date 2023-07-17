use bevy::prelude::*;

mod pos;
pub use pos::TilePos;

use crate::fieldofview::FieldOfView;

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
            transform: pos.as_transform(0.0),

            ..Default::default()
        }
    }
}

/// Is a tile walkable?
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deref, DerefMut, Component)]
pub struct Walkable(bool);

/// Is a tile transparent?
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deref, DerefMut, Component)]
pub struct Transparent(bool);

#[derive(Debug, Clone, Default, Bundle)]
pub struct TileBundle {
    /// Marker component indicating that this tile is, indeed, a tile
    pub tile: Tile,
    /// Whether or not a tile can be walked across
    pub walkable: Walkable,
    /// Whether or not a tile is transparent to light/vision
    pub transparent: Transparent,
    /// The tile's name
    pub name: Name,
    /// The tile's FoV status wrt the player
    pub fov: FieldOfView,
}

impl TileBundle {
    pub fn floor() -> Self {
        Self {
            walkable: Walkable(true),
            transparent: Transparent(true),
            name: Name::new("Stone Floor"),
            tile: Tile,
            fov: FieldOfView::Unexplored,
        }
    }

    pub fn wall() -> Self {
        Self {
            walkable: Walkable(false),
            transparent: Transparent(false),
            name: Name::new("Stone Wall"),
            tile: Tile,
            fov: FieldOfView::Unexplored,
        }
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
