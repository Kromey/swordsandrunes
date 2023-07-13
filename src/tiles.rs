use bevy::prelude::*;

mod pos;
pub use pos::TilePos;

/// Dimensions of a tile edge; tiles are assumed to be square
pub const TILE_SIZE: u32 = 32;
/// Dimensions of a tile edge as a f32 value
pub const TILE_SIZE_F32: f32 = TILE_SIZE as f32;

/// Marker component to identify tiles
#[derive(Debug, Default, Clone, Copy, Component)]
pub struct Tile;

/// Is a tile walkable?
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deref, DerefMut, Component)]
pub struct Walkable(bool);

/// Is a tile transparent?
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deref, DerefMut, Component)]
pub struct Transparent(bool);

#[derive(Debug, Default, Bundle)]
pub struct TileBundle {
    /// Marker component indicating that this tile is, indeed, a tile
    pub tile: Tile,
    /// Whether or not a tile can be walked across
    pub walkable: Walkable,
    /// Whether or not a tile is transparent to light/vision
    pub transparent: Transparent,
    /// The tile's name
    pub name: Name,
    
    // These components are from SpriteBundle; copied here since Bevy has done away with bundle flattening

    pub sprite: Sprite,
    /// A handle to the sprite's image
    pub texture: Handle<Image>,
    /// Data pertaining to how the sprite is drawn on the screen
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}

impl TileBundle {
    pub fn floor(x: u32, y: u32, texture: Handle<Image>) -> Self {
        let pos = TilePos { x, y };
        Self {
            walkable: Walkable(true),
            transparent: Transparent(true),
            name: Name::new("Stone Floor"),

            texture,
            transform: pos.as_transform(0.0),

            ..Default::default()
        }
    }

    pub fn wall(x: u32, y: u32, texture: Handle<Image>) -> Self {
        let pos = TilePos { x, y };
        Self {
            walkable: Walkable(false),
            transparent: Transparent(false),
            name: Name::new("Stone Wall"),

            texture,
            transform: pos.as_transform(0.0),

            ..Default::default()
        }
    }
}
