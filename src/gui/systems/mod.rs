use bevy_egui::{egui, EguiContext};

use crate::{
    game::{GameDay, GameTick, InGameState},
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

pub fn resource_bar(mut egui_context: ResMut<EguiContext>, ui_assets: Res<assets::UiAssets>) {}

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
