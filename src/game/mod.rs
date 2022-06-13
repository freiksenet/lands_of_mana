use std::{collections::HashMap, time::Duration};

use leafwing_input_manager::prelude::*;

pub mod actions;
pub mod load_map;
pub mod map;
pub mod province;
pub mod units;
pub mod world;

use self::map::Position;
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
        game_tick_stage.add_system_set(
            ConditionSet::new()
                .label_and_after(config::GameTickStageLabel::UpdateResources)
                .run_in_state(InGameState::Running)
                .with_system(update_stockpile_resources)
                .into(),
        );

        app.add_enter_system(config::EngineState::LoadingWorld, setup_game_world)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(config::EngineState::LoadingWorld)
                    .with_system(load_map::load_map)
                    .into(),
            )
            .add_exit_system(config::EngineState::LoadingWorld, setup_actions)
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

fn setup_game_world(mut commands: Commands) {
    commands
        .spawn_bundle(GameWorldBundle::empty())
        .with_children(|builder| {
            builder
                .spawn_bundle(world::PlayerBundle {
                    color: world::PlayerColor(Color::RED),
                    name: world::PlayerName("PLAYER".to_string()),
                    ..Default::default()
                })
                .with_children(|builder| {
                    let player_entity = builder.parent_entity();
                    builder.spawn_bundle(game::world::PlayerStockpileBundle {
                        player: game::world::OfPlayer(player_entity),
                        resource: game::world::StockpileResourceType::Gold,
                        amount: game::world::StockpileResourceAmount(100.),
                    });
                    builder.spawn_bundle(game::world::PlayerStockpileBundle {
                        player: game::world::OfPlayer(player_entity),

                        resource: game::world::StockpileResourceType::Wood,
                        amount: game::world::StockpileResourceAmount(50.),
                    });
                    builder.spawn_bundle(game::world::PlayerCapacityBundle {
                        player: game::world::OfPlayer(player_entity),

                        resource: game::world::CapacityResourceType::Sun,
                    });
                    builder.spawn_bundle(game::world::PlayerCapacityBundle {
                        player: game::world::OfPlayer(player_entity),

                        resource: game::world::CapacityResourceType::Arcana,
                    });
                    builder.spawn_bundle(game::world::PlayerCapacityBundle {
                        player: game::world::OfPlayer(player_entity),

                        resource: game::world::CapacityResourceType::Death,
                    });
                    builder.spawn_bundle(game::world::PlayerCapacityBundle {
                        player: game::world::OfPlayer(player_entity),

                        resource: game::world::CapacityResourceType::Chaos,
                    });
                    builder.spawn_bundle(game::world::PlayerCapacityBundle {
                        player: game::world::OfPlayer(player_entity),

                        resource: game::world::CapacityResourceType::Nature,
                    });
                    let mut unit = builder.spawn();
                    game::units::UnitBundle::insert_full(
                        &mut unit,
                        player_entity,
                        units::UnitType::Skeleton,
                        Position { x: 0, y: 0 },
                    );
                })
                .insert(world::Viewer {});
        });
}

fn setup_actions(mut commands: Commands, world_query: Query<Entity, With<GameWorld>>) {
    let world = world_query.single();
    commands
        .entity(world)
        .insert_bundle(InputManagerBundle::<game::actions::WorldActions> {
            action_state: ActionState::default(),
            input_map: InputMap::default(),
        })
        .insert_bundle(TransformBundle {
            local: Transform::identity(),
            global: GlobalTransform::identity(),
        });
}

fn game_tick(mut game_time_query: Query<(&mut GameTick, &mut GameDay)>) {
    let (mut game_tick, mut game_day) = game_time_query.single_mut();
    game_tick.0 += 1;
    if game_tick.0 >= 10 {
        game_tick.0 = 0;
        game_day.0 += 1;
    }
}

fn update_stockpile_resources(
    mut _commands: Commands,
    game_tick_query: Query<&GameTick, Changed<GameDay>>,
    mut stockpiles_query: Query<(
        &game::world::OfPlayer,
        &game::world::StockpileResourceType,
        &mut game::world::StockpileResourceAmount,
    )>,
    prosumers_query: Query<(
        &game::world::OfPlayer,
        &game::world::StockpileResourceType,
        &game::world::StockpileResourceProsumer,
    )>,
) {
    if let Ok(game_tick) = game_tick_query.get_single() {
        if game_tick.0 == 0 {
            let mut stockpiles_by_player: HashMap<
                u32,
                HashMap<
                    game::world::StockpileResourceType,
                    Mut<game::world::StockpileResourceAmount>,
                >,
            > = HashMap::new();
            for (player, stockpile_resource_type, stockpile_resource_amount) in
                stockpiles_query.iter_mut()
            {
                let stockpile_amount = stockpile_resource_amount;
                stockpiles_by_player.try_insert(player.0.id(), HashMap::new());
                stockpiles_by_player.entry(player.0.id()).and_modify(
                    |m: &mut HashMap<
                        game::world::StockpileResourceType,
                        Mut<game::world::StockpileResourceAmount>,
                    >| {
                        m.insert(*stockpile_resource_type, stockpile_amount);
                    },
                );
            }

            for (player, stockpile_resource_type, game::world::StockpileResourceProsumer(amount)) in
                prosumers_query.iter()
            {
                if let Some(by_type) = stockpiles_by_player.get_mut(&player.0.id()) {
                    if let Some(stockpile_amount) = by_type.get_mut(stockpile_resource_type) {
                        stockpile_amount.0 += amount;
                    }
                }
            }
        }
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

#[derive(Bundle, Clone, Debug)]
pub struct GameWorldBundle {
    pub world: GameWorld,
    pub game_day: GameDay,
    pub game_tick: GameTick,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl GameWorldBundle {
    pub fn empty() -> GameWorldBundle {
        GameWorldBundle {
            world: GameWorld {},
            game_day: GameDay(0),
            game_tick: GameTick(0),
            transform: Transform::identity(),
            global_transform: GlobalTransform::identity(),
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameTick(pub usize);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameDay(pub u32);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum InGameState {
    #[default]
    Paused,
    Running,
}
