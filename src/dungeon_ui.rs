use bevy::prelude::*;

use crate::{combat::HP, setup::Player, GameState};

#[derive(Debug, Default, Component)]
pub struct DungeonUI;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct HPBar;

fn spawn_dungeon_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                },
                ..Default::default()
            },
            DungeonUI,
        ))
        .with_children(|parent| {
            // === Bottom UI ===
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(160.0),
                        flex_direction: FlexDirection::Row,
                        // column_gap: Val::Px(5.0),
                        ..Default::default()
                    },
                    background_color: Color::DARK_GRAY.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // === Left panel ===
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                height: Val::Percent(100.0),
                                width: Val::Px(320.0),
                                padding: UiRect::all(Val::Px(5.0)),
                                flex_direction: FlexDirection::Column,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // === HP bar ===
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(32.0),
                                        ..Default::default()
                                    },
                                    background_color: Color::rgb(0.8, 0.0, 0.0).into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn((
                                            NodeBundle {
                                                style: Style {
                                                    width: Val::Percent(100.0),
                                                    overflow: Overflow::visible(),
                                                    ..Default::default()
                                                },
                                                background_color: Color::rgb(0.0, 0.8, 0.0).into(),
                                                ..Default::default()
                                            },
                                            HPBar,
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                TextBundle::from_section(
                                                    "HP: 0 / 0",
                                                    TextStyle {
                                                        font: asset_server
                                                            .load("fonts/FiraMono-Medium.ttf"),
                                                        font_size: 32.0,
                                                        color: Color::rgb(0.0, 0.0, 0.5),
                                                    },
                                                )
                                                .with_no_wrap(),
                                                HPBar,
                                            ));
                                        });
                                });
                        });

                    // === Center panel ===
                    parent.spawn(NodeBundle {
                        style: Style {
                            height: Val::Percent(100.0),
                            flex_grow: 1.0,
                            ..Default::default()
                        },
                        ..Default::default()
                    });

                    // === Right panel ===
                    parent.spawn(NodeBundle {
                        style: Style {
                            height: Val::Percent(100.0),
                            width: Val::Px(320.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        });
}

fn update_hp(
    player_hp_qry: Query<&HP, (Changed<HP>, With<Player>)>,
    mut hp_bar_qry: Query<(Option<&mut Style>, Option<&mut Text>), With<HPBar>>,
) {
    if let Ok(player_hp) = player_hp_qry.get_single() {
        for (style, text) in hp_bar_qry.iter_mut() {
            if let Some(mut style) = style {
                style.width = Val::Percent(player_hp.percent());
            }

            if let Some(mut text) = text {
                text.sections[0].value = format!(
                    "HP: {:width$} / {}",
                    player_hp.current(),
                    player_hp.max(),
                    // number.ilog10() + 1 tells us how many digits in a number
                    // Here we're ensuring `current` always occupies the same width as `max`
                    width = (player_hp.max().checked_ilog10().unwrap_or(0) + 1) as usize,
                );
            }
        }
    }
}

fn despawn_dungeon_ui(mut commands: Commands, dungeon_ui_qry: Query<Entity, With<DungeonUI>>) {
    if let Ok(dungeon_ui) = dungeon_ui_qry.get_single() {
        commands.entity(dungeon_ui).despawn_recursive();
    }
}

#[derive(Debug)]
pub struct DungeonUIPlugin;

impl Plugin for DungeonUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Running), spawn_dungeon_ui)
            .add_systems(OnExit(GameState::Running), despawn_dungeon_ui)
            .add_systems(Update, update_hp.run_if(in_state(GameState::Running)));
    }
}
