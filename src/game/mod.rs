use bevy::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::config;

pub mod actions;
pub mod map;
pub mod units;
pub mod world_gen;

pub struct GamePlugin {
    pub config: config::EngineConfig,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(self.config.load_world, world_gen::build_world)
            .add_loopless_state(InGameState::Paused)
            .add_plugin(InputManagerPlugin::<actions::WorldActions>::default())
            .add_plugin(InputManagerPlugin::<actions::SelectActions>::default())
            .add_system(
                handle_world_actions
                    .run_in_state(self.config.run_game)
                    .label("game_actions")
                    .after("input"),
            );
    }
}

fn handle_world_actions(
    mut commands: Commands,
    action_state_query: Query<&ActionState<actions::WorldActions>>,
) {
    let action_state = action_state_query.single();
    if (action_state.just_pressed(actions::WorldActions::Pause)) {
        commands.insert_resource(NextState(InGameState::Paused));
    }

    if (action_state.just_pressed(actions::WorldActions::Resume)) {
        commands.insert_resource(NextState(InGameState::Running));
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum InGameState {
    Paused,
    Running,
}
