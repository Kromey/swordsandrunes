use bevy::prelude::*;

use crate::{
    input_manager::{Action, Actions},
    setup::Player,
    GameState,
};

#[derive(Debug, Default, Clone, Copy, Component)]
pub struct PrimaryCamera;

fn setup_camera(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    commands.spawn((Camera2dBundle::default(), PrimaryCamera));

    next_state.set(GameState::MainMenu);
}

/// Update the camera's position when the player moves
///
/// FIXME: This is really janky as there may be a 1-frame delay, resulting in flickering
#[allow(clippy::type_complexity)]
fn camera_follow_player(
    mut camera_qry: Query<&mut Transform, With<PrimaryCamera>>,
    player_qry: Query<&Transform, (With<Player>, Without<PrimaryCamera>, Changed<Transform>)>,
) {
    if let Ok(mut camera_transform) = camera_qry.get_single_mut() {
        if let Ok(player_transform) = player_qry.get_single() {
            camera_transform.translation = player_transform
                .translation
                .truncate()
                .extend(camera_transform.translation.z);
        }
    }
}

/// Update camera zoom
fn camera_zoom(
    actions: Res<Actions>,
    mut camera_qry: Query<&mut OrthographicProjection, With<PrimaryCamera>>,
) {
    if let Ok(mut projection) = camera_qry.get_single_mut() {
        if actions.perform(Action::ZoomIn) {
            projection.scale -= 0.2;
        }
        if actions.perform(Action::ZoomOut) {
            projection.scale += 0.2;
        }

        projection.scale = projection.scale.clamp(0.2, 2.0);
    }
}

#[derive(Debug, Default)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup_camera.run_if(in_state(GameState::Starting)))
            .add_systems(Update, camera_follow_player)
            .add_systems(Update, camera_zoom);
    }
}
