use assoc::AssocExt;
use bevy_egui::{egui, EguiContext};

use crate::{
    game::{world::*, GameDay, GameTick, InGameState},
    gui::{widgets::*, GuiContext, TextureType},
    prelude::*,
};

pub fn title_bar(mut egui_context: ResMut<EguiContext>, gui_context: Res<GuiContext>) {
    NinePatchWindow::new("Title Bar")
        .title_bar(false)
        .auto_sized()
        .anchor(egui::Align2::LEFT_TOP, egui::Vec2::new(4., 4.))
        .frame(
            egui::Frame::window(&egui_context.ctx_mut().style())
                .inner_margin(egui::style::Margin::symmetric(32., 8.)),
        )
        .body_nine_patch(
            *gui_context
                .get_texture_id(TextureType::Window, "scroll_horizontal_wrapped")
                .unwrap(),
            egui::vec2(32., 16.),
        )
        .show(egui_context.ctx_mut(), |ui| {
            ui.label(
                egui::RichText::new("Lands of Mana")
                    .text_style(egui::TextStyle::Name("Heading2".into())),
            );
        });
}

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

#[derive(Clone, Debug, PartialEq, Eq)]

pub struct PlayerResources {
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

pub fn bind_current_player_resources(
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

pub fn resource_bar(
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
            egui::vec2(32., 16.),
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

pub fn time_bar(
    mut egui_context: ResMut<EguiContext>,
    gui_context: Res<GuiContext>,
    game_state: Res<CurrentState<InGameState>>,
    game_time_query: Query<(&GameDay, &GameTick)>,
) {
    let (GameDay(game_day), GameTick(game_tick)) = game_time_query.single();
    NinePatchWindow::new("Time Bar")
        .title_bar(false)
        .auto_sized()
        .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::new(-4., 4.))
        .frame(
            egui::Frame::window(&egui_context.ctx_mut().style())
                .inner_margin(egui::style::Margin::symmetric(8., 0.)),
        )
        .body_nine_patch(
            *gui_context
                .get_texture_id(TextureType::Window, "bright")
                .unwrap(),
            egui::vec2(32., 16.),
        )
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!("Day\u{00A0}{:04}", game_day + 1))
                        .text_style(egui::TextStyle::Body),
                );

                ui.label(
                    egui::RichText::new(format!("Tick\u{00A0}{:02}", game_tick + 1))
                        .text_style(egui::TextStyle::Body),
                );
                ui.add_enabled(
                    game_state.0 == InGameState::Running,
                    icon_button(
                        *gui_context
                            .get_texture_id(TextureType::Button, "shallow")
                            .unwrap(),
                        *gui_context
                            .get_texture_id(TextureType::IconOutline, "pause")
                            .unwrap(),
                        egui::vec2(16., 16.),
                    ),
                );
                ui.add_enabled(
                    game_state.0 == InGameState::Paused,
                    icon_button(
                        *gui_context
                            .get_texture_id(TextureType::Button, "shallow")
                            .unwrap(),
                        *gui_context
                            .get_texture_id(TextureType::IconOutline, "resume")
                            .unwrap(),
                        egui::vec2(16., 16.),
                    ),
                );
            });
        });
}
