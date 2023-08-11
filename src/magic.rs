use serde::Deserialize;

use crate::combat::HP;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Effect {
    Heal(u16),
    Harm(u16),
}

pub fn apply_effect(effect: Effect, hp: &mut HP) {
    match effect {
        Effect::Heal(heal) => hp.add(heal),
        Effect::Harm(dmg) => hp.sub(dmg),
    }
}
