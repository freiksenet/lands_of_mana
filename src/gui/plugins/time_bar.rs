use bevy_egui::{egui, EguiContext};

use crate::{
    config::{EngineState, UiSyncLabel},
    game::{GameDay, GameTick, InGameState},
    gui::{
        gui_context::{GuiContext, TextureType},
        widgets::*,
    },
    prelude::*,
};

pub struct TimeBarPlugin {}

impl Plugin for TimeBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            config::Stage::UiSync,
            ConditionSet::new()
                .run_in_state(EngineState::InGame)
                .label_and_after(UiSyncLabel::Update)
                .with_system(time_bar)
                .into(),
        );
    }
}

fn time_bar(
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
            egui::vec2(32., 32.),
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
                    gui_context.icon_button(
                        &gui::ButtonType::Shallow,
                        &gui::ButtonSize::Medium,
                        "pause",
                    ),
                );
                ui.add_enabled(
                    game_state.0 == InGameState::Paused,
                    gui_context.icon_button(
                        &gui::ButtonType::Shallow,
                        &gui::ButtonSize::Medium,
                        "resume",
                    ),
                );
            });
        });
}
