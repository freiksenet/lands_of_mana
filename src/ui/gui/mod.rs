use bevy::prelude::*;
use iyes_loopless::prelude::*;
use kayak_core::{bind, Binding, MutableBound};
use kayak_ui::{
    bevy::{BevyContext, BevyKayakUIPlugin, FontMapping, UICameraBundle},
    core::{render, Index},
};

use crate::{assets, config, game};

mod topbar;

pub struct GuiPlugin {
    pub config: config::EngineConfig,
}

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BevyKayakUIPlugin)
            .add_enter_system(
                self.config.run_game,
                setup_binding_resources.exclusive_system(),
            )
            .add_enter_system(self.config.run_game, setup_ui)
            .add_system_set(
                ConditionSet::new()
                    .label("gui")
                    .after("input")
                    .run_in_state(self.config.run_game)
                    .with_system(bind_game_time)
                    .with_system(bind_game_state)
                    .with_system(bind_current_player_resources)
                    .into(),
            );
    }
}

pub fn setup_ui(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    font_assets: Res<assets::FontAssets>,
) {
    commands.spawn_bundle(UICameraBundle::new());
    font_mapping.set_default(font_assets.compass.clone());

    let context = BevyContext::new(|context| {
        render! {
            <kayak_ui::widgets::App>
              <topbar::TopBar />
            </kayak_ui::widgets::App>
        }
    });

    commands.insert_resource(context);
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]

pub struct PlayerStockpileResource {
    pub resource_type: game::world::StockpileResourceType,
    pub amount: f32,
    pub income: f32,
    // Tooltip stuff here maybe? could be separate tyfpe
}

impl Eq for PlayerStockpileResource {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]

pub struct PlayerCapacityResource {
    pub resource_type: game::world::CapacityResourceType,
    pub free: u32,
    pub total: u32,
    // Tooltip stuff here maybe? could be separate type
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]

pub struct PlayerResources {
    pub stockpile_resources: Vec<PlayerStockpileResource>,
    pub capacity_resources: Vec<PlayerCapacityResource>,
}

pub fn setup_binding_resources(
    mut commands: Commands,
    game_time_query: Query<&game::GameTime>,
    game_state: Res<CurrentState<game::InGameState>>,
) {
    let game_time = game_time_query.single();
    commands.insert_resource(bind(*game_time));
    commands.insert_resource(bind(game_state.0));
    commands.insert_resource(bind(PlayerResources {
        stockpile_resources: vec![
            PlayerStockpileResource {
                resource_type: game::world::StockpileResourceType::Gold,
                amount: 0.0,
                income: 0.0,
            },
            PlayerStockpileResource {
                resource_type: game::world::StockpileResourceType::Wood,
                amount: 0.0,
                income: 0.0,
            },
        ],
        capacity_resources: vec![
            PlayerCapacityResource {
                resource_type: game::world::CapacityResourceType::Sun,
                free: 0,
                total: 0,
            },
            PlayerCapacityResource {
                resource_type: game::world::CapacityResourceType::Arcana,
                free: 0,
                total: 0,
            },
            PlayerCapacityResource {
                resource_type: game::world::CapacityResourceType::Death,
                free: 0,
                total: 0,
            },
            PlayerCapacityResource {
                resource_type: game::world::CapacityResourceType::Chaos,
                free: 0,
                total: 0,
            },
            PlayerCapacityResource {
                resource_type: game::world::CapacityResourceType::Nature,
                free: 0,
                total: 0,
            },
        ],
    }))
}

pub fn bind_game_time(
    game_time_binding: ResMut<Binding<game::GameTime>>,
    game_time_query: Query<&game::GameTime, Changed<game::GameTime>>,
) {
    let game_time_result = game_time_query.get_single();
    if let Ok(game_time) = game_time_result {
        game_time_binding.set(*game_time);
    }
}

pub fn bind_game_state(
    game_state_binding: ResMut<Binding<game::InGameState>>,
    game_state: Res<CurrentState<game::InGameState>>,
) {
    if game_state.is_changed() {
        game_state_binding.set(game_state.0);
    }
}

pub fn bind_current_player_resources(
    player_resources_binding: ResMut<Binding<PlayerResources>>,
    player_query: Query<&Children, With<game::world::Viewer>>,
    stockpile_resources_query: Query<(
        &game::world::StockpileResourceType,
        &game::world::StockpileResourceAmount,
    )>,
    capacity_resources_query: Query<&game::world::CapacityResourceType>,
) {
    let player_children = player_query.single();
    let mut capacity_resources = Vec::new();
    let mut stockpile_resources = Vec::new();
    for child_entity in player_children.iter() {
        if let Ok((resource_type, game::world::StockpileResourceAmount(amount))) =
            stockpile_resources_query.get(*child_entity)
        {
            stockpile_resources.push(PlayerStockpileResource {
                resource_type: *resource_type,
                amount: *amount,
                income: 0.0,
            });
        }
        if let Ok(resource_type) = capacity_resources_query.get(*child_entity) {
            capacity_resources.push(PlayerCapacityResource {
                resource_type: *resource_type,
                free: 0,
                total: 0,
            });
        }
    }
    player_resources_binding.set(PlayerResources {
        stockpile_resources,
        capacity_resources,
    })
}
