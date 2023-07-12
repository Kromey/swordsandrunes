use bevy::prelude::*;

mod pos;
pub use pos::TilePos;

/// Dimensions of a tile edge; tiles are assumed to be square
pub const TILE_SIZE: u32 = 16;

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
    
    // These components are from SpriteSheetBundle; copied here since Bevy has done away with bundle flattening

    /// The specific sprite from the texture atlas to be drawn, defaulting to the sprite at index 0.
    pub sprite: TextureAtlasSprite,
    /// A handle to the texture atlas that holds the sprite images
    pub texture_atlas: Handle<TextureAtlas>,
    /// Data pertaining to how the sprite is drawn on the screen
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}

impl TileBundle {
    pub fn floor(x: u32, y: u32, texture_atlas: Handle<TextureAtlas>) -> Self {
        let pos = TilePos { x, y };
        Self {
            walkable: Walkable(true),
            transparent: Transparent(true),

            sprite: TextureAtlasSprite::new(7),
            texture_atlas,
            transform: pos.as_transform(0.0),

            ..Default::default()
        }
    }

    pub fn wall(x: u32, y: u32, texture_atlas: Handle<TextureAtlas>) -> Self {
        let pos = TilePos { x, y };
        Self {
            walkable: Walkable(false),
            transparent: Transparent(false),

            sprite: TextureAtlasSprite::new(880),
            texture_atlas,
            transform: pos.as_transform(0.0),

            ..Default::default()
        }
    }
}
