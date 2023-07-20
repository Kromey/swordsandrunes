use bevy::prelude::*;

use crate::{
    input_manager::{Actions, InputManager},
    map::Map,
    setup::Player,
    tiles::{BlocksMovement, Tile, TilePos, TILE_SIZE_F32},
    GameState,
};

pub fn movement_system(
    actions: Res<Actions>,
    mut player: Query<&mut Transform, With<Player>>,
    tile_qry: Query<&BlocksMovement, With<Tile>>,
    map: Res<Map>,
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

            if let Some(tile) = map.get(dest) {
                // If it doesn't block movement, allow the move
                if tile_qry.get(tile).is_err() {
                    transform.translation = dest.as_vec().extend(transform.translation.z);
                }
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
                .run_if(in_state(GameState::Running)),
        );
    }
}
