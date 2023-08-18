use bevy::prelude::*;

mod dungeon_ui;
mod inventory_ui;
pub mod messages;
mod spell_target_ui;

pub use inventory_ui::RedrawInventoryUi;
pub use messages::Messages;

use crate::{
    input_manager::{Action, Actions},
    GameState, TurnState,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum GameUi {
    #[default]
    Main,
    Inventory,
    TargetSpell,
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

fn reset_ui(mut ui_state: ResMut<NextState<GameUi>>) {
    ui_state.set(GameUi::Main);
}

fn destroy_ui<C: Component>(mut commands: Commands, inventory_ui_qry: Query<Entity, With<C>>) {
    for ui in inventory_ui_qry.iter() {
        commands.entity(ui).despawn_recursive();
    }
}

#[derive(Debug)]
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Messages>()
            .add_state::<GameUi>()
            .add_event::<RedrawInventoryUi>()
            // === Main Game UI ===
            .add_systems(OnExit(TurnState::WaitingForPlayer), reset_ui)
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
                    dungeon_ui::update_looking_at.run_if(in_state(GameUi::Main)),
                )
                    .run_if(in_state(GameState::Running)),
            )
            // == Inventory UI ==
            .add_systems(OnEnter(GameUi::Inventory), inventory_ui::spawn_inventory_ui)
            .add_systems(
                OnExit(GameUi::Inventory),
                destroy_ui::<inventory_ui::InventoryUi>,
            )
            .add_systems(
                Update,
                (
                    inventory_ui::inventory_interaction,
                    inventory_ui::build_inventory_ui,
                )
                    .run_if(in_state(GameUi::Inventory)),
            )
            // == Spell Target UI ==
            .add_systems(
                OnEnter(GameUi::TargetSpell),
                spell_target_ui::init_spell_targeting,
            )
            .add_systems(
                OnExit(GameUi::TargetSpell),
                destroy_ui::<spell_target_ui::SpellTargetUi>,
            )
            .add_systems(
                Update,
                (
                    spell_target_ui::update_single_target_select,
                    spell_target_ui::update_area_target_select,
                    spell_target_ui::fire_area_target_spell,
                )
                    .run_if(in_state(GameUi::TargetSpell)),
            );
    }
}
