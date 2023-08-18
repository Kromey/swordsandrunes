use std::collections::HashSet;

use bevy::{prelude::*, window::PrimaryWindow};
use itertools::Itertools;

use crate::{
    camera::PrimaryCamera,
    combat::HP,
    dungeon::{BlocksMovement, BlocksSight, Map, TilePos, TILE_SIZE_F32},
    fieldofview::{compute_limited_fov, FieldOfView},
    magic::{CastSpell, CastSpellOn, SpellTarget, SpellToCast},
    utils::SpriteLayer,
};

use super::GameUi;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub(super) struct SpellTargetUi;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub(super) struct SingleTarget(Entity);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub(super) struct TargetArea(i32);

pub(super) fn init_spell_targeting(world: &mut World) {
    if let Some(casting) = world.resource::<SpellToCast>().0 {
        match casting.spell.target {
            SpellTarget::Caster => world.resource_mut::<NextState<GameUi>>().set(GameUi::Main),
            SpellTarget::Single => init_single_target_select(casting, world),
            SpellTarget::Area(radius) => init_area_target_select(casting, radius, world),
        }
    } else {
        world.resource_mut::<NextState<GameUi>>().set(GameUi::Main);
    }
}

fn init_area_target_select(casting: CastSpell, radius: u8, world: &mut World) {
    let from = world.get::<Transform>(casting.caster).unwrap();
    let from_tile = TilePos::from(from);

    world.spawn((
        SpatialBundle::from_transform(from_tile.as_transform(SpriteLayer::UI)),
        SpellTargetUi,
        TargetArea(radius as i32),
    ));
}

fn init_single_target_select(casting: CastSpell, world: &mut World) {
    let (camera, &camera_transform) = world
        .query_filtered::<(&Camera, &GlobalTransform), With<PrimaryCamera>>()
        .get_single(world)
        .unwrap();
    // Hacky way to get around borrow checker if we keep the reference instead
    // Consider rewriting this to use SystemState
    let camera = camera.clone();

    let from = world.get::<Transform>(casting.caster).unwrap();
    let from_tile = TilePos::from(from);
    let range = u32::from(casting.spell.range);

    let mut targets = Vec::new();

    for (target, target_pos) in world
        .query_filtered::<(Entity, &Transform), With<HP>>()
        .iter(world)
    {
        if target == casting.caster {
            // Don't target yourself
            continue;
        }

        let tile = TilePos::from(target_pos);
        if from_tile.distance(tile) > range {
            // Out of range, don't target this one
            continue;
        }

        if world
            .resource::<Map>()
            .get(tile)
            .and_then(|e| world.get::<FieldOfView>(e))
            .is_some_and(|fov| *fov == FieldOfView::Visible)
        {
            let world_pos = tile.corner();
            if let Some(screen_pos) =
                camera.world_to_viewport(&camera_transform, world_pos.extend(0.0))
            {
                targets.push((target, screen_pos));
            }
        }
    }

    for (target, screen_pos) in targets {
        world.spawn((
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

pub(super) fn update_area_target_select(
    camera_qry: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    mut commands: Commands,
    mut cursor_evt: EventReader<CursorMoved>,
    blocks_sight_qry: Query<&Transform, With<BlocksSight>>,
    mut target_src: Query<(Entity, &TargetArea, &mut Transform), Without<BlocksSight>>,
    map: Res<Map>,
    targetable_tile: Query<&FieldOfView, Without<BlocksMovement>>,
) {
    if let Ok((target, target_area, mut target_transform)) = target_src.get_single_mut() {
        if let Some(cursor) = cursor_evt.iter().last() {
            let (camera, camera_transform) = camera_qry.get_single().unwrap();
            if let Some(cursor_pos) = camera.viewport_to_world_2d(camera_transform, cursor.position)
            {
                let target_tile = TilePos::from(cursor_pos);

                if target_tile == TilePos::from(*target_transform) {
                    // Cursor hasn't left the currently targeted tile
                    return;
                }

                *target_transform = target_tile.as_transform(SpriteLayer::UI);
                commands.entity(target).despawn_descendants();

                if map
                    .get(target_tile)
                    .and_then(|tile_entity| targetable_tile.get(tile_entity).ok())
                    .map(|fov| *fov != FieldOfView::Visible)
                    .unwrap_or(true)
                {
                    // No FoV if the target itself isn't visible
                    return;
                }

                let blockers: HashSet<_> = blocks_sight_qry
                    .iter()
                    .map(|transform| TilePos::from(*transform))
                    .collect();

                let area = compute_limited_fov(target_tile, target_area.0, |tile| {
                    blockers.contains(&tile)
                });
                let target_origin = target_tile.as_vec();

                commands.entity(target).with_children(|parent| {
                    area.into_iter()
                        .sorted_by_cached_key(|tile| (tile.x, tile.y))
                        .dedup()
                        .filter(|tile| {
                            targetable_tile
                                .get(map.get(*tile).unwrap())
                                .is_ok_and(|fov| *fov == FieldOfView::Visible)
                        })
                        .for_each(|tile| {
                            let relative_translation = tile.as_vec() - target_origin;
                            parent.spawn(SpriteBundle {
                                transform: Transform::from_translation(
                                    relative_translation.extend(-0.1),
                                ),
                                sprite: Sprite {
                                    color: Color::GREEN.with_a(0.15),
                                    custom_size: Some(Vec2::splat(TILE_SIZE_F32)),
                                    ..Default::default()
                                },
                                ..Default::default()
                            });
                        });
                });
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn fire_area_target_spell(
    buttons: Res<Input<MouseButton>>,
    window_qry: Query<&Window, With<PrimaryWindow>>,
    camera_qry: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    blocks_sight_qry: Query<&Transform, With<BlocksSight>>,
    spell: Res<SpellToCast>,
    targetable_qry: Query<(Entity, &Transform), With<HP>>,
    mut spell_evt: EventWriter<CastSpellOn>,
    mut ui_state: ResMut<NextState<GameUi>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(spell) = spell.0 {
            let spell_area = match spell.spell.target {
                SpellTarget::Caster | SpellTarget::Single => return,
                SpellTarget::Area(area) => area,
            };
            if let Some(cursor_position) = window_qry.single().cursor_position() {
                let (camera, camera_transform) = camera_qry.get_single().unwrap();
                if let Some(cursor_pos) =
                    camera.viewport_to_world_2d(camera_transform, cursor_position)
                {
                    let spell_tile = TilePos::from(cursor_pos);
                    let blockers: HashSet<_> = blocks_sight_qry
                        .iter()
                        .map(|transform| TilePos::from(*transform))
                        .collect();
                    let area: HashSet<_> =
                        compute_limited_fov(spell_tile, spell_area as i32, |tile| {
                            blockers.contains(&tile)
                        })
                        .into_iter()
                        .collect();

                    for (target, target_transform) in targetable_qry.iter() {
                        let target_tile = TilePos::from(target_transform);
                        if area.contains(&target_tile) {
                            spell_evt.send(spell.on(target));
                        }
                    }

                    ui_state.set(GameUi::Main);
                }
            }
        }
    }
}
