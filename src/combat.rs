use crate::bump::{bump_system, BumpEvent};
use bevy::prelude::*;
use std::cmp::min;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Event)]
pub struct AttackEvent {
    attacker: Entity,
    target: Entity,
}

impl From<BumpEvent> for AttackEvent {
    fn from(bump: BumpEvent) -> Self {
        Self {
            attacker: bump.entity,
            target: bump.target,
        }
    }
}

impl From<&BumpEvent> for AttackEvent {
    fn from(bump: &BumpEvent) -> Self {
        Self::from(*bump)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Component)]
pub struct HP {
    current: u8,
    max: u8,
}

impl HP {
    pub fn new(hp: u8) -> Self {
        Self {
            current: hp,
            max: hp,
        }
    }

    pub fn current(&self) -> u8 {
        self.current
    }

    pub fn max(&self) -> u8 {
        self.max
    }

    pub fn add(&mut self, value: u8) {
        self.current = min(self.max, self.current + value);
    }

    pub fn sub(&mut self, value: u8) {
        self.current = self.current.saturating_sub(value);
    }
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Component, Deref, DerefMut,
)]
pub struct Power(pub u8);

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Component, Deref, DerefMut,
)]
pub struct Defense(pub u8);

impl std::ops::Sub<Defense> for Power {
    type Output = u8;

    fn sub(self, rhs: Defense) -> Self::Output {
        self.0.saturating_sub(rhs.0)
    }
}

impl std::ops::Sub<&Defense> for &Power {
    type Output = u8;

    fn sub(self, rhs: &Defense) -> Self::Output {
        self.0.saturating_sub(rhs.0)
    }
}

fn attack_system(
    attacker_qry: Query<&Power>,
    mut defender_qry: Query<(&mut HP, &Defense)>,
    mut attack_events: EventReader<AttackEvent>,
) {
    for attack in attack_events.iter() {
        if let Ok(power) = attacker_qry.get(attack.attacker) {
            if let Ok((mut hp, defense)) = defender_qry.get_mut(attack.target) {
                let damage = power - defense;
                hp.sub(damage);
            }
        }
    }
}

fn death_system(
    dead_qry: Query<(Entity, &HP, Option<&Name>), Changed<HP>>,
    mut commands: Commands,
) {
    for (entity, hp, name) in dead_qry.iter() {
        if hp.current() == 0 {
            if let Some(name) = name {
                info!("Killed {name}");
            } else {
                info!("Killed {entity:?}");
            }
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Debug, Default)]
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttackEvent>().add_systems(
            Update,
            (attack_system, death_system).chain().after(bump_system),
        );
    }
}
