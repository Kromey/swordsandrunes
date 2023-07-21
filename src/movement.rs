use bevy::prelude::*;

use crate::{
    bump::BumpEvent,
    dungeon::{BlocksMovement, TilePos, TILE_SIZE_F32},
    input_manager::{Actions, InputManager},
    setup::Player,
    TurnState,
};

#[allow(clippy::type_complexity)]
pub fn movement_system(
    actions: Res<Actions>,
    mut player_qry: Query<(Entity, &mut Transform), With<Player>>,
    blockers_qry: Query<(Entity, &Transform), (With<BlocksMovement>, Without<Player>)>,
    mut next_state: ResMut<NextState<TurnState>>,
    mut bump_events: EventWriter<BumpEvent>,
) {
    let mut delta = Vec2::ZERO;

    if actions.perform(crate::input_manager::Action::WalkNorth) {
        delta += Vec2::Y;
    }
    if actions.perform(crate::input_manager::Action::WalkEast) {
        delta += Vec2::X;
    }
    if actions.perform(crate::input_manager::Action::WalkSouth) {
        delta += Vec2::NEG_Y;
    }
    if actions.perform(crate::input_manager::Action::WalkWest) {
        delta += Vec2::NEG_X;
    }

    if delta.length_squared() > 0.1 {
        delta = delta.round() * TILE_SIZE_F32;

        if let Ok((player, mut transform)) = player_qry.get_single_mut() {
            let dest = TilePos::from(transform.translation.truncate() + delta);

            // If there's nothing in the destination blocking movement, allow the move
            if let Some(blocker) = blockers_qry
                .iter()
                .find(|&(_, transform)| TilePos::from(*transform) == dest)
                .map(|(entity, _)| entity)
            {
                bump_events.send(BumpEvent::new(player, blocker));
            } else {
                transform.translation = dest.as_vec().extend(transform.translation.z);

                // We did our move, end our turn
                next_state.0 = Some(TurnState::MonsterTurn);
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            movement_system
                .after(InputManager)
                .run_if(in_state(TurnState::PlayerTurn)),
        );
    }
}
