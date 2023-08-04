use std::path::{Path, PathBuf};

pub fn get_dat_path<P>(file: P) -> PathBuf
where
    P: AsRef<Path>,
{
    bevy::asset::FileAssetIo::get_base_path()
        .join("assets/dat")
        .join(file.as_ref())
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpriteLayer {
    #[default]
    Tile,
    Decoration,
    Item,
    Actor,
}

impl SpriteLayer {
    pub fn as_f32(&self) -> f32 {
        *self as u8 as f32
    }
}
