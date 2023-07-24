use crate::{
    bump::{handle_bumps, BumpEvent},
    dungeon::TILE_SIZE_F32,
};
use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro512StarStar;
use std::{cmp::min, f32::consts::TAU};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Event)]
pub struct DamageEvent {
    entity: Entity,
    damage: u16,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Component)]
pub struct HP {
    current: u16,
    max: u16,
}

impl HP {
    pub fn new(hp: u16) -> Self {
        Self {
            current: hp,
            max: hp,
        }
    }

    pub fn current(&self) -> u16 {
        self.current
    }

    pub fn max(&self) -> u16 {
        self.max
    }

    pub fn add(&mut self, value: u16) {
        self.current = min(self.max, self.current.saturating_add(value));
    }

    pub fn sub(&mut self, value: u16) {
        self.current = self.current.saturating_sub(value);
    }
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Component, Deref, DerefMut,
)]
pub struct Power(pub u16);

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Component, Deref, DerefMut,
)]
pub struct Defense(pub u16);

impl std::ops::Sub<Defense> for Power {
    type Output = u16;

    fn sub(self, rhs: Defense) -> Self::Output {
        self.0.saturating_sub(rhs.0)
    }
}

impl std::ops::Sub<&Defense> for &Power {
    type Output = u16;

    fn sub(self, rhs: &Defense) -> Self::Output {
        self.0.saturating_sub(rhs.0)
    }
}

fn attack(
    attacker_qry: Query<&Power>,
    mut defender_qry: Query<(&mut HP, &Defense)>,
    mut attack_events: EventReader<AttackEvent>,
    mut damage_event: EventWriter<DamageEvent>,
) {
    for attack in attack_events.iter() {
        if let Ok(power) = attacker_qry.get(attack.attacker) {
            if let Ok((mut hp, defense)) = defender_qry.get_mut(attack.target) {
                let damage = power - defense;

                if damage > 0 {
                    hp.sub(damage);
                    damage_event.send(DamageEvent {
                        entity: attack.target,
                        damage,
                    });
                }
            }
        }
    }
}

fn splatter_blood(
    mut damage_event: EventReader<DamageEvent>,
    transform_qry: Query<&Transform>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut rng = Xoshiro512StarStar::from_entropy();
    for event in damage_event.iter() {
        if let Ok(transform) = transform_qry.get(event.entity) {
            let scale = rng.gen_range(0.3..1.0);
            let rot = rng.gen_range(0.0..TAU);
            let displace = rng.gen_range(0.0..(TILE_SIZE_F32 / 2.0));
            let displace_rot = rng.gen_range(0.0..TAU);
            let blood = format!("sprites/blood/blood_red{:02}.png", rng.gen_range(0..30));

            let mut transform = *transform;
            transform.translation += Vec2::from_angle(displace_rot).extend(0.0) * displace;
            transform.translation.z = 0.5; // Above tiles, below mobs
            transform.rotate_z(rot);
            transform.scale = Vec3::splat(scale);

            commands.spawn((SpriteBundle {
                texture: asset_server.load(blood),
                transform,
                ..Default::default()
            },));
        }
    }
}

fn remove_dead(dead_qry: Query<(Entity, &HP, Option<&Name>), Changed<HP>>, mut commands: Commands) {
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
        app.add_event::<AttackEvent>()
            .add_event::<DamageEvent>()
            .add_systems(
                Update,
                (
                    (attack, remove_dead).chain().after(handle_bumps),
                    splatter_blood,
                ),
            );
    }
}