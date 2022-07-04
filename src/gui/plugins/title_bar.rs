use bevy_egui::{egui, EguiContext};

use crate::{
    config::{EngineState, UiSyncLabel},
    gui::{
        gui_context::{GuiContext, TextureType},
        widgets::*,
    },
    prelude::*,
};

pub struct TitleBarPlugin {}

impl Plugin for TitleBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            config::Stage::UiSync,
            ConditionSet::new()
                .run_in_state(EngineState::InGame)
                .label_and_after(UiSyncLabel::Update)
                .with_system(title_bar)
                .into(),
        );
    }
}

fn title_bar(mut egui_context: ResMut<EguiContext>, gui_context: Res<GuiContext>) {
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
