use bevy::prelude::*;

mod dungeon_ui;
mod inventory_ui;
pub mod messages;

pub use messages::Messages;

use crate::{
    input_manager::{Action, Actions},
    GameState,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum GameUi {
    #[default]
    Main,
    Inventory,
}

fn set_initial_ui_state(mut ui_state: ResMut<NextState<GameUi>>) {
    ui_state.set(GameUi::Main);
}

fn ui_state_manager(
    actions: Res<Actions>,
    current_state: Res<State<GameUi>>,
    mut next_state: ResMut<NextState<GameUi>>,
) {
    if actions.perform(Action::OpenInventory) {
        if *current_state == GameUi::Main {
            next_state.set(GameUi::Inventory);
        } else {
            next_state.set(GameUi::Main);
        }
    }
}

#[derive(Debug)]
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Messages>()
            .add_state::<GameUi>()
            // === Main Game UI ===
            .add_systems(
                OnEnter(GameState::Running),
                (set_initial_ui_state, dungeon_ui::spawn_dungeon_ui),
            )
            .add_systems(OnExit(GameState::Running), dungeon_ui::despawn_dungeon_ui)
            .add_systems(
                Update,
                (
                    ui_state_manager,
                    dungeon_ui::update_hp,
                    dungeon_ui::update_message_log,
                    dungeon_ui::update_looking_at,
                )
                    .run_if(in_state(GameState::Running)),
            )
            // == Inventory UI ==
            .add_systems(OnEnter(GameUi::Inventory), inventory_ui::build_inventory_ui)
            .add_systems(
                OnExit(GameUi::Inventory),
                inventory_ui::destroy_inventory_ui,
            );
    }
}
