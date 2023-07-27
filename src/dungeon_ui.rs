use bevy::prelude::*;

use crate::GameState;

#[derive(Debug, Default, Component)]
pub struct DungeonUI;

fn spawn_dungeon_ui(mut commands: Commands) {
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
                        column_gap: Val::Px(5.0),
                        ..Default::default()
                    },
                    background_color: Color::ALICE_BLUE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // === Left panel ===
                    parent.spawn(NodeBundle {
                        style: Style {
                            height: Val::Percent(100.0),
                            width: Val::Px(320.0),
                            ..Default::default()
                        },
                        background_color: Color::DARK_GREEN.into(),
                        ..Default::default()
                    });

                    // === Center panel ===
                    parent.spawn(NodeBundle {
                        style: Style {
                            height: Val::Percent(100.0),
                            flex_grow: 1.0,
                            ..Default::default()
                        },
                        background_color: Color::BLUE.into(),
                        ..Default::default()
                    });

                    // === Right panel ===
                    parent.spawn(NodeBundle {
                        style: Style {
                            height: Val::Percent(100.0),
                            width: Val::Px(320.0),
                            ..Default::default()
                        },
                        background_color: Color::GOLD.into(),
                        ..Default::default()
                    });
                });
        });
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
            .add_systems(OnExit(GameState::Running), despawn_dungeon_ui);
    }
}
