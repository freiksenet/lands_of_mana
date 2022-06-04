use std::time::Duration;

use leafwing_input_manager::prelude::*;

pub mod actions;
pub mod load_map;
pub mod map;
pub mod units;
pub mod world;

use crate::prelude::*;

pub struct GamePlugin {}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Stage that will tick time
        let mut game_tick_stage = SystemStage::parallel();
        game_tick_stage.add_system_set(
            ConditionSet::new()
                .label_and_after(config::GameTickStageLabel::Tick)
                .run_in_state(InGameState::Running)
                .with_system(game_tick)
                .into(),
        );

        app.add_enter_system(config::EngineState::LoadingWorld, load_map::load_map)
            .add_exit_system(config::EngineState::LoadingWorld, setup)
            .add_loopless_state(InGameState::Paused)
            .add_plugin(InputManagerPlugin::<actions::WorldActions>::default())
            .add_plugin(InputManagerPlugin::<actions::SelectActions>::default())
            .add_system_set(
                ConditionSet::new()
                    .label_and_after(config::UpdateStageLabel::GameActions)
                    .run_in_state(config::EngineState::InGame)
                    .with_system(handle_world_actions)
                    .into(),
            )
            .add_stage_after(
                CoreStage::Update,
                config::Stage::GameTick,
                FixedTimestepStage::new(Duration::from_millis(1000)).with_stage(game_tick_stage),
            );
    }
}

fn setup(mut commands: Commands, world_query: Query<Entity, With<GameWorld>>) {
    let world = world_query.single();
    commands
        .entity(world)
        .insert(GameTime { tick: 0, day: 0 })
        .insert_bundle(InputManagerBundle::<game::actions::WorldActions> {
            action_state: ActionState::default(),
            input_map: InputMap::default(),
        })
        .insert_bundle(TransformBundle {
            local: Transform::identity(),
            global: GlobalTransform::identity(),
        });
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

#[derive(Component, Debug, Clone)]
pub struct GameWorld {}

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
