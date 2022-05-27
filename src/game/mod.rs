use bevy::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::config;

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
            .add_system(handle_pausing.run_in_state(self.config.run_game));
    }
}

fn handle_pausing(
    mut commands: Commands,
    state: Res<CurrentState<InGameState>>,
    action_state_query: Query<&ActionState<Actions>>,
) {
    let action_state = action_state_query.single();
    if (action_state.just_pressed(Actions::TogglePause)) {
        commands.insert_resource(NextState(match state.0 {
            InGameState::Paused => InGameState::Running,
            InGameState::Running => InGameState::Paused,
        }));
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum InGameState {
    Paused,
    Running,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Actions {
    TogglePause,
}
