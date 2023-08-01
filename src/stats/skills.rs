use crate::rand::Random;
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(transparent)]
pub struct Skill(i32);

impl Skill {
    pub fn new(level: i32) -> Self {
        Self(level)
    }

    pub fn level(&self) -> i32 {
        self.0
    }

    pub fn check(&self, modifiers: i32, rng: &mut Random) -> (bool, i32) {
        let roll = rng.roll_3d6();
        let effective_level = (self.0 + modifiers).clamp(0, 20);

        (roll <= effective_level, effective_level - roll)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Component)]
pub struct SkillSheet(HashMap<String, Skill>);

impl SkillSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<S: AsRef<str>>(&self, skill: S) -> Skill {
        self.0.get(skill.as_ref()).copied().unwrap_or_default()
    }

    pub fn set<S: Into<String>>(&mut self, skill: S, level: Skill) {
        self.0.insert(skill.into(), level);
    }
}
