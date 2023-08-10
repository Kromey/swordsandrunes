use bevy::prelude::*;
use itertools::Itertools;

use crate::{
    inventory::{Inventory, InventoryIdx},
    items::{ItemId, ItemList, UseItem},
    setup::Player,
};

const INVENTORY_TILE_SIZE: f32 = 72.0;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub(super) struct InventoryUi;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub(super) struct InventoryCell;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub struct RedrawInventoryUi;

pub(super) fn spawn_inventory_ui(mut redraw_evt: EventWriter<RedrawInventoryUi>) {
    redraw_evt.send_default();
}

pub(super) fn build_inventory_ui(
    mut redraw_evt: EventReader<RedrawInventoryUi>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_inventory: Query<&Inventory, With<Player>>,
    item_list: Res<ItemList>,
    inventory_ui_qry: Query<Entity, With<InventoryUi>>,
) {
    // Check for our redraw event and short-circuit if there isn't one
    if redraw_evt.is_empty() {
        return;
    }
    redraw_evt.clear();

    // Make sure we start with a clean slate
    for ui in inventory_ui_qry.iter() {
        commands.entity(ui).despawn_recursive();
    }

    let font_handle: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            InventoryUi,
        ))
        .with_children(|container| {
            container
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        grid_template_columns: RepeatedGridTrack::px(5, INVENTORY_TILE_SIZE),
                        grid_template_rows: RepeatedGridTrack::px(6, INVENTORY_TILE_SIZE),
                        // row_gap: Val::Px(5.0),
                        // column_gap: Val::Px(5.0),
                        padding: UiRect::all(Val::Px(15.0)),
                        ..Default::default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.9).into(),
                    ..Default::default()
                })
                .with_children(|grid| {
                    // Header
                    grid.spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            grid_column: GridPlacement::span(5),
                            align_items: AlignItems::Center,
                            justify_items: JustifyItems::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|header| {
                        header.spawn(TextBundle::from_section(
                            "Your Inventory",
                            TextStyle {
                                font: font_handle.clone(),
                                font_size: 32.0,
                                color: Color::WHITE,
                            },
                        ));
                    });

                    // Inventory items
                    let inventory = player_inventory
                        .get_single()
                        .map(|inv| inv.enumerate().collect_vec())
                        .unwrap_or_default();
                    for item in inventory
                        .into_iter()
                        .map_into::<Option<_>>()
                        .pad_using(25, |_| None)
                    {
                        spawn_item_cell(grid, item, &item_list, font_handle.clone(), &asset_server);
                    }
                });
        });
}

fn spawn_item_cell(
    grid: &mut ChildBuilder,
    item: Option<(InventoryIdx, &ItemId)>,
    item_list: &ItemList,
    font_handle: Handle<Font>,
    asset_server: &AssetServer,
) {
    let background_color = if item.is_some() {
        Color::GRAY.into()
    } else {
        Color::DARK_GRAY.into()
    };

    let mut cell = grid.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(INVENTORY_TILE_SIZE),
                height: Val::Px(INVENTORY_TILE_SIZE),
                border: UiRect::all(Val::Px(3.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            border_color: Color::BLACK.into(),
            background_color,
            ..Default::default()
        },
        InventoryCell,
    ));
    // If this cell is occupied, make it interactable
    if let Some((idx, _)) = item {
        cell.insert((Interaction::default(), idx));
    }

    cell.with_children(|cell| {
        if let Some((_, item)) = item {
            // Place the image first so it lies underneath the text we'll spawn next
            cell.spawn(ImageBundle {
                image: asset_server
                    .load("sprites/items/potions/brilliant_blue.png")
                    .into(),
                ..Default::default()
            });

            cell.spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(2.0),
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|container| {
                container.spawn(TextBundle::from_section(
                    &item_list[item].name,
                    TextStyle {
                        font: font_handle,
                        font_size: 12.0,
                        color: Color::WHITE,
                    },
                ));
            });
        }
    });
}

#[allow(clippy::type_complexity)]
pub(super) fn inventory_interaction(
    player_qry: Query<(Entity, &Inventory), With<Player>>,
    mut cell_qry: Query<
        (&Interaction, &InventoryIdx, &mut BorderColor),
        (Changed<Interaction>, With<InventoryCell>),
    >,
    mut use_item_evt: EventWriter<UseItem>,
) {
    for (interaction, idx, mut border) in cell_qry.iter_mut() {
        match *interaction {
            Interaction::None => *border = Color::BLACK.into(),
            Interaction::Hovered => *border = Color::YELLOW.into(),
            Interaction::Pressed => {
                *border = Color::GREEN.into();
                let (player, inventory) = player_qry.get_single().unwrap();
                use_item_evt.send(UseItem {
                    item: inventory[idx],
                    user: player,
                });
            }
        }
    }
}

pub(super) fn destroy_inventory_ui(
    mut commands: Commands,
    inventory_ui_qry: Query<Entity, With<InventoryUi>>,
) {
    for ui in inventory_ui_qry.iter() {
        commands.entity(ui).despawn_recursive();
    }
}
