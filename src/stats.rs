use crate::rand::prelude::*;
use bevy::prelude::*;
use rand_distr::Normal;
use serde::Deserialize;

pub mod skills;
pub use skills::{Skill, SkillSheet};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize, Component)]
#[serde(default)]
pub struct Attributes {
    pub strength: u8,
    pub dexterity: u8,
    pub intelligence: u8,
    pub perception: u8,
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            strength: 10,
            dexterity: 10,
            intelligence: 10,
            perception: 10,
        }
    }
}

impl Attributes {
    pub fn roll_damage(&self, rng: &mut Random) -> i32 {
        let mean = self.strength as f64 / 2.0;
        let std_dev = 0.25;
        let distr = Normal::<f64>::new(1.0, std_dev).unwrap();

        (distr.sample(rng).clamp(0.5, 1.5) * mean).round() as i32
    }
}
