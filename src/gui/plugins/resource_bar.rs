use assoc::AssocExt;
use bevy_egui::{egui, EguiContext};

use crate::{
    config::{EngineState, Stage, UiSyncLabel},
    game::world::*,
    gui::{
        gui_context::{GuiContext, TextureType},
        widgets::*,
    },
    prelude::*,
};

pub struct ResourceBarPlugin {}

impl Plugin for ResourceBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(config::EngineState::LoadingGraphics, setup_resource_bar)
            .add_system_set_to_stage(
                Stage::UiSync,
                ConditionSet::new()
                    .run_in_state(EngineState::InGame)
                    .label_and_after(UiSyncLabel::Sync)
                    .with_system(bind_current_player_resources)
                    .into(),
            )
            .add_system_set_to_stage(
                config::Stage::UiSync,
                ConditionSet::new()
                    .run_in_state(EngineState::InGame)
                    .label_and_after(UiSyncLabel::Update)
                    .with_system(resource_bar)
                    .into(),
            );
    }
}

fn setup_resource_bar(mut commands: Commands) {
    commands.init_resource::<PlayerResources>();
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
struct PlayerStockpileResource {
    pub amount: f32,
    pub income: f32,
    // Tooltip stuff here maybe? could be separate tyfpe
}

impl Eq for PlayerStockpileResource {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
struct PlayerCapacityResource {
    pub free: i32,
    pub total: i32,
    // Tooltip stuff here maybe? could be separate type
}

#[derive(Debug, PartialEq, Eq)]

struct PlayerResources {
    pub stockpile_resources: Vec<(StockpileResourceType, PlayerStockpileResource)>,
    pub capacity_resources: Vec<(CapacityResourceType, PlayerCapacityResource)>,
}

impl Default for PlayerResources {
    fn default() -> Self {
        PlayerResources {
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
        }
    }
}

fn bind_current_player_resources(
    mut player_resources: ResMut<PlayerResources>,
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
}

fn resource_bar(
    mut egui_context: ResMut<EguiContext>,
    gui_context: Res<GuiContext>,
    resources: Res<PlayerResources>,
) {
    NinePatchWindow::new("Resource Bar")
        .title_bar(false)
        .auto_sized()
        .anchor(egui::Align2::CENTER_TOP, egui::Vec2::new(0., 4.))
        .body_nine_patch(
            *gui_context
                .get_texture_id(TextureType::Window, "bright")
                .unwrap(),
            egui::vec2(32., 32.),
        )
        .frame(
            egui::Frame::window(&egui_context.ctx_mut().style())
                .inner_margin(egui::style::Margin::symmetric(8., 0.)),
        )
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                for (resource_type, resource) in resources.stockpile_resources.iter() {
                    ui.image(
                        gui_context.icon_texture_id_for_stockpile_resource(resource_type),
                        egui::vec2(16., 16.),
                    );
                    let income_text = if resource.income >= 0. {
                        format!("+{:}", resource.income)
                    } else {
                        format!("-{:}", resource.income.abs())
                    };
                    ui.label(format!("{:}{:}", resource.amount, income_text));
                }
                for (resource_type, resource) in resources.capacity_resources.iter() {
                    ui.image(
                        gui_context.icon_texture_id_for_capacity_resource(resource_type),
                        egui::vec2(16., 16.),
                    );
                    ui.label(format!("{:}/{:}", resource.free, resource.total));
                }
            });
        });
}
