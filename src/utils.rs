use std::path::{Path, PathBuf};

pub fn get_dat_path<P>(file: P) -> PathBuf
where
    P: AsRef<Path>,
{
    bevy::asset::FileAssetIo::get_base_path()
        .join("assets/dat")
        .join(file.as_ref())
}
