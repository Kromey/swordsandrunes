use bevy::prelude::*;

use crate::{
    camera::PrimaryCamera,
    combat::HP,
    dungeon::{Map, TilePos, TILE_SIZE_F32},
    fieldofview::FieldOfView,
    magic::{CastSpellOn, SpellTarget, SpellToCast},
};

use super::GameUi;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub(super) struct SpellTargetUi;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub(super) struct SingleTarget(Entity);

#[allow(clippy::type_complexity)]
pub(super) fn init_spell_targeting(
    commands: Commands,
    camera_qry: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    targets_qry: Query<(Entity, &Transform), With<HP>>,
    tile_fov_qry: Query<&FieldOfView>,
    map: Res<Map>,
    mut ui_state: ResMut<NextState<GameUi>>,
    spell: Res<SpellToCast>,
) {
    if let Some(spell) = spell.0 {
        match spell.spell.target {
            SpellTarget::Caster => ui_state.set(GameUi::Main),
            SpellTarget::Single => init_single_target_select(
                spell.caster,
                commands,
                camera_qry,
                targets_qry,
                tile_fov_qry,
                spell.spell.range,
                map,
            ),
            SpellTarget::Area(_) => todo!(),
        }
    } else {
        ui_state.set(GameUi::Main);
    }
}

#[allow(clippy::type_complexity)]
fn init_single_target_select(
    caster: Entity,
    mut commands: Commands,
    camera_qry: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    targets_qry: Query<(Entity, &Transform), With<HP>>,
    tile_fov_qry: Query<&FieldOfView>,
    range: u8,
    map: Res<Map>,
) {
    let (camera, camera_transform) = camera_qry.get_single().unwrap();

    let (_, from) = targets_qry.get(caster).unwrap();
    let from_tile = TilePos::from(from);
    let range = u32::from(range);

    for (target, target_pos) in targets_qry.iter() {
        if target == caster {
            // Don't target yourself
            continue;
        }

        let tile = TilePos::from(target_pos);
        if from_tile.distance(tile) > range {
            // Out of range, don't target this one
            continue;
        }

        if map
            .get(tile)
            .and_then(|e| tile_fov_qry.get(e).ok())
            .is_some_and(|fov| *fov == FieldOfView::Visible)
        {
            let world_pos = tile.as_vec() + Vec2::new(-TILE_SIZE_F32 / 2.0, TILE_SIZE_F32 / 2.0);
            if let Some(screen_pos) =
                camera.world_to_viewport(camera_transform, world_pos.extend(0.0))
            {
                commands.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(TILE_SIZE_F32),
                            height: Val::Px(TILE_SIZE_F32),
                            position_type: PositionType::Absolute,
                            top: Val::Px(screen_pos.y),
                            left: Val::Px(screen_pos.x),
                            border: UiRect::all(Val::Px(1.0)),
                            ..Default::default()
                        },
                        border_color: Color::ALICE_BLUE.into(),
                        ..Default::default()
                    },
                    Interaction::default(),
                    SpellTargetUi,
                    SingleTarget(target),
                ));
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub(super) fn update_single_target_select(
    mut targets_qry: Query<
        (&Interaction, &mut BorderColor, Option<&SingleTarget>),
        (With<SpellTargetUi>, Changed<Interaction>),
    >,
    spell: Res<SpellToCast>,
    mut spell_evt: EventWriter<CastSpellOn>,
    mut ui_state: ResMut<NextState<GameUi>>,
) {
    for (interaction, mut border, single_target) in targets_qry.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Cast the spell on the target
                spell_evt.send(spell.on(single_target.unwrap().0));
                // Close the spell target UI
                ui_state.set(GameUi::Main);
            }
            Interaction::Hovered => *border = Color::GREEN.into(),
            Interaction::None => *border = Color::ALICE_BLUE.into(),
        }
    }
}
