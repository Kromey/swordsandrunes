use crate::combat::HP;
use bevy::prelude::*;
use serde::Deserialize;

mod spells;
pub use spells::{CastSpell, CastSpellOn, Spell, SpellTarget, SpellToCast};

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

pub struct MagicPlugin;

impl Plugin for MagicPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CastSpell>()
            .add_event::<CastSpellOn>()
            .init_resource::<SpellToCast>()
            .add_systems(Update, (spells::cast_spell, spells::cast_spell_on));
    }
}
