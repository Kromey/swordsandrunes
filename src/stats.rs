use bevy::prelude::*;

pub mod skills;
pub use skills::{Skill, SkillSheet};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Component)]
pub struct Attributes {
    pub strength: u8,
    pub dexterity: u8,
    pub intelligence: u8,
    pub perception: u8,
}
