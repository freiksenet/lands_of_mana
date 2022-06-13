use assoc::AssocExt;
use kayak_core::{bind, Binding, Bound, MutableBound};

use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct PlayerStockpileResource {
    pub amount: f32,
    pub income: f32,
    // Tooltip stuff here maybe? could be separate tyfpe
}

impl Eq for PlayerStockpileResource {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PlayerCapacityResource {
    pub free: i32,
    pub total: i32,
    // Tooltip stuff here maybe? could be separate type
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PixelWindow(pub f32, pub f32);

impl Default for PixelWindow {
    fn default() -> PixelWindow {
        PixelWindow(200., 200.)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]

pub struct PlayerResources {
    pub stockpile_resources: Vec<(game::world::StockpileResourceType, PlayerStockpileResource)>,
    pub capacity_resources: Vec<(game::world::CapacityResourceType, PlayerCapacityResource)>,
}

pub fn setup_binding_resources(
    mut commands: Commands,
    windows: Res<Windows>,
    game_time_query: Query<(&game::GameTick, &game::GameDay)>,
    game_state: Res<CurrentState<game::InGameState>>,
) {
    let (game_tick, game_day) = game_time_query.single();
    let window = windows.get_primary().unwrap();
    commands.insert_resource(bind(PixelWindow(window.width(), window.height())));
    commands.insert_resource(bind(*game_tick));
    commands.insert_resource(bind(*game_day));
    commands.insert_resource(bind(game_state.0));
    commands.insert_resource(bind(PlayerResources {
        stockpile_resources: vec![
            (
                game::world::StockpileResourceType::Gold,
                PlayerStockpileResource {
                    amount: 0.0,
                    income: 0.0,
                },
            ),
            (
                game::world::StockpileResourceType::Wood,
                PlayerStockpileResource {
                    amount: 0.0,
                    income: 0.0,
                },
            ),
        ],
        capacity_resources: vec![
            (
                game::world::CapacityResourceType::Sun,
                PlayerCapacityResource { free: 0, total: 0 },
            ),
            (
                game::world::CapacityResourceType::Arcana,
                PlayerCapacityResource { free: 0, total: 0 },
            ),
            (
                game::world::CapacityResourceType::Death,
                PlayerCapacityResource { free: 0, total: 0 },
            ),
            (
                game::world::CapacityResourceType::Chaos,
                PlayerCapacityResource { free: 0, total: 0 },
            ),
            (
                game::world::CapacityResourceType::Nature,
                PlayerCapacityResource { free: 0, total: 0 },
            ),
        ],
    }))
}

pub fn bindings_system_set() -> SystemSet {
    ConditionSet::new()
        .run_in_state(config::EngineState::InGame)
        .label_and_after(config::UiSyncLabel::Sync)
        .with_system(bind_pixel_window)
        .with_system(bind_game_tick)
        .with_system(bind_game_day)
        .with_system(bind_game_state)
        .with_system(bind_current_player_resources)
        .into()
}

fn bind_pixel_window(
    window_binding: ResMut<Binding<PixelWindow>>,
    projection_query: Query<
        &ui::gui::camera::UIOrthographicProjection,
        Changed<ui::gui::camera::UIOrthographicProjection>,
    >,
) {
    if let Ok(projection) = projection_query.get_single() {
        window_binding.set(PixelWindow(projection.right, projection.bottom));
    }
}
fn bind_game_day(
    game_day_binding: ResMut<Binding<game::GameDay>>,
    game_day_query: Query<&game::GameDay, Changed<game::GameDay>>,
) {
    let game_day_result = game_day_query.get_single();
    if let Ok(game_day) = game_day_result {
        game_day_binding.set(*game_day);
    }
}

fn bind_game_tick(
    game_tick_binding: ResMut<Binding<game::GameTick>>,
    game_tick_query: Query<&game::GameTick, Changed<game::GameTick>>,
) {
    let game_tick_result = game_tick_query.get_single();
    if let Ok(game_tick) = game_tick_result {
        game_tick_binding.set(*game_tick);
    }
}

fn bind_game_state(
    game_state_binding: ResMut<Binding<game::InGameState>>,
    game_state: Res<CurrentState<game::InGameState>>,
) {
    if game_state.is_changed() {
        game_state_binding.set(game_state.0);
    }
}

// Assumes one player exists
fn bind_current_player_resources(
    _commands: Commands,
    player_resources_binding: ResMut<Binding<PlayerResources>>,
    stockpile_resources_query: Query<(
        &game::world::StockpileResourceType,
        &game::world::StockpileResourceAmount,
    )>,
    stockile_resources_prosumer_query: Query<(
        &game::world::StockpileResourceType,
        &game::world::StockpileResourceProsumer,
    )>,
    capacity_resources_query: Query<&game::world::CapacityResourceType>,
    capacity_resources_prosumer_query: Query<(
        &game::world::CapacityResourceType,
        &game::world::CapacityResourceProsumer,
    )>,
) {
    let mut player_resources = player_resources_binding.get();

    for (resource_type, game::world::StockpileResourceAmount(amount)) in
        stockpile_resources_query.iter()
    {
        player_resources
            .stockpile_resources
            .entry(*resource_type)
            .and_modify(|res| {
                res.amount = *amount;
                res.income = 0.0;
            });
    }

    for (resource, game::world::StockpileResourceProsumer(amount)) in
        stockile_resources_prosumer_query.iter()
    {
        player_resources
            .stockpile_resources
            .entry(*resource)
            .and_modify(|res| {
                res.income += amount;
            });
    }

    for resource_type in capacity_resources_query.iter() {
        player_resources
            .capacity_resources
            .entry(*resource_type)
            .and_modify(|res| {
                res.free = 0;
                res.total = 0;
            });
    }

    for (resource, game::world::CapacityResourceProsumer(amount)) in
        capacity_resources_prosumer_query.iter()
    {
        let amount_value = *amount;

        player_resources
            .capacity_resources
            .entry(*resource)
            .and_modify(|res| {
                if amount_value >= 0 {
                    res.total += amount_value;
                }
                res.free += amount_value;
            });
    }

    player_resources_binding.set(player_resources);
}
