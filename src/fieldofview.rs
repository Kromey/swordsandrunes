//! Field of view calculated via symmetric shadowcasting
//! Ported and adapted from https://www.albertford.com/shadowcasting/

use crate::{
    dungeon::{BlocksSight, Tile, TilePos},
    setup::Player,
};
use bevy::prelude::*;
use std::collections::HashSet;

mod shadowcasting;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub enum FieldOfView {
    #[default]
    Unexplored,
    NotVisible,
    Visible,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub enum HideOutOfSight {
    #[default]
    Hide,
    Darken,
}

#[allow(clippy::type_complexity)]
pub fn update_fov(
    player_qry: Query<&Transform, (With<Player>, Changed<Transform>)>,
    blocks_sight_qry: Query<&Transform, With<BlocksSight>>,
    mut fov_set: ParamSet<(
        Query<(&mut FieldOfView, &mut Sprite, &mut Visibility, &Transform), With<Tile>>,
        Query<(&mut Visibility, &mut Sprite, &Transform, &HideOutOfSight)>,
    )>,
) {
    if let Ok(player_transform) = player_qry.get_single() {
        let player_pos = TilePos::from(*player_transform);

        let blockers: HashSet<_> = blocks_sight_qry
            .iter()
            .map(|transform| TilePos::from(*transform))
            .collect();

        let fov = shadowcasting::compute_fov(player_pos, |tile| blockers.contains(&tile));

        for (mut tile_fov, mut sprite, mut visibility, transform) in fov_set.p0().iter_mut() {
            let pos = TilePos::from(transform);
            if fov.contains(&pos) {
                *tile_fov = FieldOfView::Visible;
                *visibility = Visibility::Visible;
                sprite.color = Color::default();
            } else if *tile_fov == FieldOfView::Visible {
                *tile_fov = FieldOfView::NotVisible;
                sprite.color = Color::GRAY;
            }
        }

        for (mut visibility, mut sprite, transform, &hide) in fov_set.p1().iter_mut() {
            if fov.contains(&TilePos::from(transform)) {
                *visibility = Visibility::Visible;
                sprite.color = Color::default();
            } else {
                match hide {
                    HideOutOfSight::Darken => sprite.color = Color::GRAY,
                    HideOutOfSight::Hide => *visibility = Visibility::Hidden,
                }
            }
        }
    }
}
