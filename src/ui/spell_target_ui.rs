use bevy::prelude::*;

use crate::{
    camera::PrimaryCamera,
    combat::HP,
    dungeon::{Map, TilePos, TILE_SIZE_F32},
    fieldofview::FieldOfView,
    magic::{CastSpell, CastSpellOn, SpellTarget, SpellToCast},
};

use super::GameUi;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub(super) struct SpellTargetUi;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub(super) struct SingleTarget(Entity);

pub(super) fn init_spell_targeting(world: &mut World) {
    if let Some(casting) = world.resource::<SpellToCast>().0 {
        match casting.spell.target {
            SpellTarget::Caster => world.resource_mut::<NextState<GameUi>>().set(GameUi::Main),
            SpellTarget::Single => init_single_target_select(casting, world),
            SpellTarget::Area(_) => todo!(),
        }
    } else {
        world.resource_mut::<NextState<GameUi>>().set(GameUi::Main);
    }
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
            let world_pos = tile.as_vec() + Vec2::new(-TILE_SIZE_F32 / 2.0, TILE_SIZE_F32 / 2.0);
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
