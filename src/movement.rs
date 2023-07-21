use bevy::prelude::*;

use crate::{
    dungeon::{BlocksMovement, TilePos, TILE_SIZE_F32},
    input_manager::{Actions, InputManager},
    setup::Player,
    TurnState,
};

pub fn movement_system(
    actions: Res<Actions>,
    mut player: Query<&mut Transform, With<Player>>,
    blocks_movement_qry: Query<&Transform, (With<BlocksMovement>, Without<Player>)>,
    mut next_state: ResMut<NextState<TurnState>>,
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

        if let Ok(mut transform) = player.get_single_mut() {
            let dest = TilePos::from(transform.translation.truncate() + delta);

            // If there's nothing in the destination blocking movement, allow the move
            if !blocks_movement_qry
                .iter()
                .any(|transform| TilePos::from(*transform) == dest)
            {
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
