use bevy::prelude::*;
use serde::Deserialize;

use crate::{combat::HP, ui::GameUi};

use super::{apply_effect, Effect};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct Spell {
    pub target: SpellTarget,
    pub range: u8,
    pub effect: Effect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SpellTarget {
    Caster,
    Single,
    Area(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Event)]
pub struct CastSpell {
    pub caster: Entity,
    pub spell: Spell,
}

impl CastSpell {
    pub fn on(&self, target: Entity) -> CastSpellOn {
        let Self { caster, spell } = *self;

        CastSpellOn {
            caster,
            target,
            spell,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Event)]
pub struct CastSpellOn {
    pub caster: Entity,
    pub target: Entity,
    pub spell: Spell,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Resource)]
pub struct SpellToCast(pub Option<CastSpell>);

impl SpellToCast {
    pub fn set(&mut self, spell: CastSpell) {
        self.0 = Some(spell);
    }

    pub fn on(&self, target: Entity) -> CastSpellOn {
        self.0.unwrap().on(target)
    }

    pub fn clear(&mut self) {
        self.0 = None
    }
}

pub(super) fn cast_spell(
    mut cast_spell_evt: EventReader<CastSpell>,
    mut cast_spell_on_evt: EventWriter<CastSpellOn>,
    mut ui_state: ResMut<NextState<GameUi>>,
    mut spell_to_cast: ResMut<SpellToCast>,
) {
    for cast in cast_spell_evt.iter() {
        match cast.spell.target {
            SpellTarget::Caster => {
                cast_spell_on_evt.send(cast.on(cast.caster));
            }
            SpellTarget::Single | SpellTarget::Area(_) => {
                spell_to_cast.set(*cast);
                ui_state.set(GameUi::TargetSpell);
            }
        }
    }
}

pub(super) fn cast_spell_on(
    mut cast_spell_on_evt: EventReader<CastSpellOn>,
    mut health_qry: Query<&mut HP>,
    mut spell_to_cast: ResMut<SpellToCast>,
) {
    for cast in cast_spell_on_evt.iter() {
        if let Ok(mut hp) = health_qry.get_mut(cast.target) {
            apply_effect(cast.spell.effect, &mut hp);
        }

        spell_to_cast.clear();
    }
}
