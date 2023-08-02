use bevy::prelude::*;

use crate::{
    combat::AttackEvent, dungeon::Tile, dungeon_ui::Messages, movement::movement, TurnState,
};

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

pub fn handle_bumps(
    mut bumps: EventReader<BumpEvent>,
    mut next_state: ResMut<NextState<TurnState>>,
    tile_qry: Query<Entity, With<Tile>>,
    mut attack_event: EventWriter<AttackEvent>,
    mut messages: ResMut<Messages>,
) {
    for bump in bumps.iter() {
        if tile_qry.contains(bump.target) {
            // Bumped into a tile, do nothing
            // For now we just go back to waiting for player input
            next_state.set(TurnState::WaitingForPlayer);
            messages.add("You cannot go that way");

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
        next_state.set(TurnState::MonsterTurn);
    }
}

pub struct BumpPlugin;

impl Plugin for BumpPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BumpEvent>()
            .add_systems(Update, handle_bumps.after(movement));
    }
}
