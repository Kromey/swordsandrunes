use bevy::prelude::*;
use serde::Deserialize;
use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

use crate::{dungeon::BlocksMovement, utils::get_dat_path};

/// Marker component for mobs
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct Mob;

type MobMap = HashMap<String, MobData>;

#[derive(Debug, Deserialize, Resource)]
#[serde(from = "MobMap")]
pub struct MobList {
    names: HashMap<String, usize>,
    mobs: Vec<MobData>,
}

impl MobList {
    pub fn from_raws() -> Self {
        let path = get_dat_path("mobs.toml");
        let data = read_to_string(path).unwrap();
        toml::from_str(&data).unwrap()
    }

    pub fn spawn<S: AsRef<str>>(
        &self,
        mob_name: S,
        commands: &mut Commands,
        asset_server: &AssetServer,
    ) -> Entity {
        let id = self.names.get(mob_name.as_ref()).unwrap();
        let mob = self.mobs[*id].spawn(commands, asset_server);
        commands
            .entity(mob)
            .insert(Name::new(mob_name.as_ref().to_owned()));
        mob
    }
}

impl From<MobMap> for MobList {
    fn from(value: MobMap) -> Self {
        let (name_list, mobs) = value.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();
        let names = name_list
            .into_iter()
            .enumerate()
            .map(|(i, name)| (name, i))
            .collect();

        Self { names, mobs }
    }
}

#[derive(Debug, Deserialize)]
pub struct MobData {
    sprite: String,
    #[serde(default = "default_blocks_movement")]
    blocks_movement: bool,
}

impl MobData {
    pub fn spawn(&self, commands: &mut Commands, asset_server: &AssetServer) -> Entity {
        let mut ec = commands.spawn((
            SpriteBundle {
                texture: asset_server.load(self.sprite()),
                ..Default::default()
            },
            Mob,
        ));

        if self.blocks_movement {
            ec.insert(BlocksMovement);
        }

        ec.id()
    }

    pub fn sprite(&self) -> PathBuf {
        PathBuf::from("sprites").join(&self.sprite)
    }
}

fn default_blocks_movement() -> bool {
    true
}
