use std::time::Duration;

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
        // Stage that will tick time
        let mut game_tick_stage = SystemStage::parallel();
        game_tick_stage.add_system(game_tick.run_in_state(InGameState::Running));

        app.add_enter_system(self.config.load_world, world_gen::build_world)
            .add_loopless_state(InGameState::Paused)
            .add_plugin(InputManagerPlugin::<actions::WorldActions>::default())
            .add_plugin(InputManagerPlugin::<actions::SelectActions>::default())
            .add_system(
                handle_world_actions
                    .run_in_state(self.config.run_game)
                    .label("game_actions")
                    .after("input"),
            )
            .add_stage_after(
                CoreStage::Update,
                "game_tick",
                FixedTimestepStage::new(Duration::from_millis(1000)).with_stage(game_tick_stage),
            );
    }
}

fn game_tick(mut game_time_query: Query<&mut GameTime>) {
    let mut game_time = game_time_query.single_mut();
    game_time.tick += 1;
    if game_time.tick >= 10 {
        game_time.tick = 0;
        game_time.day += 1;
    }
}

fn handle_world_actions(
    mut commands: Commands,
    action_state_query: Query<&ActionState<actions::WorldActions>>,
) {
    let action_state = action_state_query.single();
    if action_state.just_pressed(actions::WorldActions::Pause) {
        commands.insert_resource(NextState(InGameState::Paused));
    }

    if action_state.just_pressed(actions::WorldActions::Resume) {
        commands.insert_resource(NextState(InGameState::Running));
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameTime {
    pub tick: u32,
    pub day: u32,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum InGameState {
    #[default]
    Paused,
    Running,
}
