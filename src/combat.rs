use crate::{
    bump::{handle_bumps, BumpEvent},
    dungeon::TILE_SIZE_F32,
    dungeon_ui::Messages,
    fieldofview::HideOutOfSight,
    rand::prelude::*,
    stats::{Attributes, SkillSheet},
    utils::SpriteLayer,
};
use bevy::prelude::*;
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

impl AttackEvent {
    pub fn new(attacker: Entity, target: Entity) -> Self {
        Self { attacker, target }
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

    pub fn ratio(&self) -> f32 {
        self.current as f32 / self.max as f32
    }

    pub fn percent(&self) -> f32 {
        self.ratio() * 100.0
    }

    pub fn add(&mut self, value: u16) {
        self.current = min(self.max, self.current.saturating_add(value));
    }

    pub fn sub(&mut self, value: u16) {
        self.current = self.current.saturating_sub(value);
    }
}

fn attack(
    attacker_qry: Query<(&SkillSheet, &Attributes, Option<&Name>)>,
    mut defender_qry: Query<(&mut HP, &SkillSheet, Option<&Name>)>,
    mut attack_events: EventReader<AttackEvent>,
    mut damage_event: EventWriter<DamageEvent>,
    mut messages: ResMut<Messages>,
    rand: Res<Random>,
) {
    for event in attack_events.iter() {
        if let Ok((attacker_skills, attacker_attributes, attacker)) =
            attacker_qry.get(event.attacker)
        {
            if let Ok((mut hp, defender_skills, defender)) = defender_qry.get_mut(event.target) {
                let mut rng = rand.from_entropy();

                let attack = attacker_skills.get("Attack");
                let defense = defender_skills.get("Defense");

                let (attack_successful, degree_of_success) = attack.check(0, &mut rng);
                if !attack_successful {
                    if let (Some(attacker), Some(defender)) = (attacker, defender) {
                        let message = format!("{attacker} misses {defender}!");
                        if attacker.as_str() == "Player" {
                            messages.add_friendly(message);
                        } else if defender.as_str() == "Player" {
                            messages.add_hostile(message);
                        } else {
                            messages.add_notice(message);
                        }
                    }

                    continue;
                }

                let (defense_successful, _) = defense.check(-degree_of_success, &mut rng);
                if defense_successful {
                    if let (Some(attacker), Some(defender)) = (attacker, defender) {
                        let message = format!("{defender} dodges {attacker}'s swing!");
                        if attacker.as_str() == "Player" {
                            messages.add_friendly(message);
                        } else if defender.as_str() == "Player" {
                            messages.add_hostile(message);
                        } else {
                            messages.add_notice(message);
                        }
                    }

                    continue;
                }

                let damage = attacker_attributes.roll_damage(&mut rng) as u16;

                if damage > 0 {
                    if let (Some(attacker), Some(defender)) = (attacker, defender) {
                        let message =
                            format!("{attacker} attacks {defender} for {damage} hit points.");
                        if attacker.as_str() == "Player" {
                            messages.add_friendly(message);
                        } else if defender.as_str() == "Player" {
                            messages.add_hostile(message);
                        } else {
                            messages.add_notice(message);
                        }
                    }
                    hp.sub(damage);
                    damage_event.send(DamageEvent {
                        entity: event.target,
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
    rand: Res<Random>,
) {
    let mut rng = rand.from_entropy();
    for event in damage_event.iter() {
        if let Ok(transform) = transform_qry.get(event.entity) {
            let scale = rng.gen_range(0.3..1.0);
            let rot = rng.gen_range(0.0..TAU);
            let displace = rng.gen_range(0.0..(TILE_SIZE_F32 / 2.0));
            let displace_rot = rng.gen_range(0.0..TAU);
            let blood = format!("sprites/blood/blood_red{:02}.png", rng.gen_range(0..30));

            let mut transform = *transform;
            transform.translation += Vec2::from_angle(displace_rot).extend(0.0) * displace;
            transform.translation.z = SpriteLayer::Decoration.as_f32();
            transform.rotate_z(rot);
            transform.scale = Vec3::splat(scale);

            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load(blood),
                    transform,
                    ..Default::default()
                },
                HideOutOfSight::Darken,
            ));
        }
    }
}

fn remove_dead(
    dead_qry: Query<(Entity, &HP, Option<&Name>), Changed<HP>>,
    mut commands: Commands,
    mut messages: ResMut<Messages>,
) {
    for (entity, hp, name) in dead_qry.iter() {
        if hp.current() == 0 {
            if let Some(name) = name {
                if name.as_str() == "Player" {
                    messages.add_hostile("YOU DIED!");
                } else {
                    messages.add_notice(format!("{name} is dead!"));
                }
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
