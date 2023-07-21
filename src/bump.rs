use bevy::prelude::*;

use crate::{combat::AttackEvent, dungeon::Tile, movement::movement_system, TurnState};

#[derive(Debug, Clone, Copy, Event)]
pub struct BumpEvent {
    pub entity: Entity,
    pub target: Entity,
}

impl BumpEvent {
    pub fn new(entity: Entity, target: Entity) -> Self {
        Self { entity, target }
    }
}

pub fn bump_system(
    mut bumps: EventReader<BumpEvent>,
    mut next_state: ResMut<NextState<TurnState>>,
    tile_qry: Query<Entity, With<Tile>>,
    mut attack_event: EventWriter<AttackEvent>,
) {
    for bump in bumps.iter() {
        if tile_qry.contains(bump.target) {
            // Bumped into a tile, do nothing
            // For now we just go back to waiting for player input
            next_state.0 = Some(TurnState::WaitingForPlayer);

            // NOTE: This might be a bug if somehow we can have multiple
            // bump events in a single frame
            return;
        }

        // TODO: Is every non-tile bump an attack?
        attack_event.send(bump.into());

        // if let Ok((mut hp, defense)) = defender_qry.get_mut(bump.target) {
        //     if let Ok(power) = attacker_qry.get(bump.entity) {
        //         let damage = power - defense;
        //         hp.sub(damage);
        //     }
        // }

        // We've handled the bump, move to the monster's turn
        next_state.0 = Some(TurnState::MonsterTurn);
    }
}

pub struct BumpPlugin;

impl Plugin for BumpPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BumpEvent>()
            .add_systems(Update, bump_system.after(movement_system));
    }
}
