use bevy::prelude::*;

use crate::{
    camera::PrimaryCamera,
    combat::HP,
    dungeon::{Map, Tile, TilePos},
    fieldofview::FieldOfView,
    setup::Player,
    GameState,
};

pub mod messages;
pub use messages::Messages;

#[derive(Debug, Default, Component)]
struct DungeonUI;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
struct HPBar;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
struct MessageLog;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
struct LookingAt;

fn spawn_dungeon_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    messages: Res<Messages>,
) {
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
                    background_color: Color::rgb(0.1, 0.1, 0.1).into(),
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
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                height: Val::Percent(100.0),
                                flex_grow: 1.0,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_sections(
                                    messages
                                        .text_sections_rev(
                                            asset_server.load("fonts/FiraMono-Medium.ttf"),
                                        )
                                        .take(5),
                                ),
                                MessageLog,
                            ));
                        });

                    // === Right panel ===
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                height: Val::Percent(100.0),
                                width: Val::Px(320.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn((TextBundle::default(), LookingAt));
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

fn update_message_log(
    messages: Res<Messages>,
    mut message_log_qry: Query<&mut Text, With<MessageLog>>,
    asset_server: Res<AssetServer>,
) {
    if messages.is_changed() {
        if let Ok(mut message_log) = message_log_qry.get_single_mut() {
            message_log.sections = messages
                .text_sections_rev(asset_server.load("fonts/FiraMono-Medium.ttf"))
                .take(5)
                .collect();
        }
    }
}

fn update_looking_at(
    mut cursor_evt: EventReader<CursorMoved>,
    mut ui_text_qry: Query<&mut Text, With<LookingAt>>,
    camera_qry: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    tile_qry: Query<(&Name, &FieldOfView), With<Tile>>,
    names_qry: Query<(&Name, &Transform), Without<Tile>>,
    map: Res<Map>,
    asset_server: Res<AssetServer>,
) {
    if let Some(cursor) = cursor_evt.iter().last() {
        let (camera, camera_transform) = camera_qry.single();
        if let Some(world_position) = camera.viewport_to_world_2d(camera_transform, cursor.position)
        {
            let tile = TilePos::from(world_position);
            let mut names = Vec::new();

            if let Some((tile_name, fov)) = map
                .get(tile)
                .and_then(|tile_entity| tile_qry.get(tile_entity).ok())
            {
                if *fov != FieldOfView::Unexplored {
                    names.push(tile_name);
                }

                if *fov == FieldOfView::Visible {
                    names.extend(names_qry.iter().filter_map(|(name, transform)| {
                        let pos = TilePos::from(transform);
                        if pos == tile {
                            Some(name)
                        } else {
                            None
                        }
                    }));
                }
            }

            let mut text = ui_text_qry.single_mut();
            let font_handle = asset_server.load("fonts/FiraMono-Medium.ttf");
            text.sections = names
                .into_iter()
                .map(|name| {
                    let mut name = name.as_str().to_owned();
                    name.push('\n');
                    TextSection {
                        value: name,
                        style: TextStyle {
                            font: font_handle.clone(),
                            font_size: 32.0,
                            color: Color::WHITE,
                        },
                    }
                })
                .collect();
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
        app.init_resource::<Messages>()
            .add_systems(OnEnter(GameState::Running), spawn_dungeon_ui)
            .add_systems(OnExit(GameState::Running), despawn_dungeon_ui)
            .add_systems(
                Update,
                (update_hp, update_message_log, update_looking_at)
                    .run_if(in_state(GameState::Running)),
            );
    }
}
