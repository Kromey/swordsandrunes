use bevy::prelude::*;

use crate::{
    combat::HP,
    dungeon::{Map, TilePos},
    fieldofview::FieldOfView,
    magic::{SpellTarget, SpellToCast},
};

use super::GameUi;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub(super) struct SpellTargetUi;

pub(super) fn init_spell_targeting(
    commands: Commands,
    targets_qry: Query<&Transform, With<HP>>,
    tile_fov_qry: Query<&FieldOfView>,
    map: Res<Map>,
    mut ui_state: ResMut<NextState<GameUi>>,
    spell: Res<SpellToCast>,
) {
    if let Some(spell) = spell.0 {
        match spell.spell.target {
            SpellTarget::Caster => ui_state.set(GameUi::Main),
            SpellTarget::Single => {
                init_single_target_select(commands, targets_qry, tile_fov_qry, map)
            }
            SpellTarget::Area(_) => todo!(),
        }
    } else {
        ui_state.set(GameUi::Main);
    }
}

fn init_single_target_select(
    mut commands: Commands,
    targets_qry: Query<&Transform, With<HP>>,
    tile_fov_qry: Query<&FieldOfView>,
    map: Res<Map>,
) {
    for target in targets_qry.iter() {
        let tile = TilePos::from(target);
        if map
            .get(tile)
            .and_then(|e| tile_fov_qry.get(e).ok())
            .is_some_and(|fov| *fov == FieldOfView::Visible)
        {
            info!("Init target at {tile:?}");
        }
    }
}
