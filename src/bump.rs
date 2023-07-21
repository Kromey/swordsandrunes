use bevy::prelude::*;

use crate::{dungeon::Tile, movement::movement_system, TurnState};

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

fn bump_system(
    mut bumps: EventReader<BumpEvent>,
    mut next_state: ResMut<NextState<TurnState>>,
    mob_qry: Query<Entity, Without<Tile>>,
    name_qry: Query<&Name>,
) {
    for bump in bumps.iter() {
        if mob_qry.contains(bump.target) {
            // Bumped a non-tile entity
            // TODO: Do something
            if let Ok(name) = name_qry.get(bump.target) {
                info!("Player bumped into {name}!");
            } else {
                info!("Player bumped {:?}", bump.target);
            }
        }
        // For now we just go back to waiting for player input
        next_state.0 = Some(TurnState::WaitingForPlayer);
    }
}

pub struct BumpPlugin;

impl Plugin for BumpPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BumpEvent>()
            .add_systems(Update, bump_system.after(movement_system));
    }
}
