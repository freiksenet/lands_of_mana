use bevy_egui::{egui, EguiContext};

use crate::{
    config::{EngineState, UiSyncLabel},
    gui::{GuiContext, TextureType},
    prelude::*,
    ui::{CursorType, Viewer},
};

pub struct CursorPlugin {}

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            config::Stage::UiSync,
            ConditionSet::new()
                .run_in_state(EngineState::InGame)
                .label_and_after(UiSyncLabel::Update)
                .with_system(cursor)
                .into(),
        );
    }
}

fn cursor(
    mut egui_context: ResMut<EguiContext>,
    gui_context: Res<GuiContext>,
    cursor_type_query: Query<&CursorType, With<Viewer>>,
) {
    let ctx = egui_context.ctx_mut();
    let position = ctx
        .input()
        .pointer
        .hover_pos()
        .map(|pos| pos + egui::vec2(-12., -12.));
    egui::Area::new("cursor")
        .fixed_pos(position.unwrap_or(egui::pos2(-16., -16.)))
        .order(egui::Order::Tooltip)
        .interactable(false)
        .drag_bounds(egui::Rect::EVERYTHING)
        .show(ctx, |ui| {
            let size = egui::vec2(24., 24.);
            let atlas = gui_context.get_texture_atlas(TextureType::Other, "cursors");
            let texture_id = cursor_type_query.single().texture_id();
            ui.add(
                egui::Image::new(atlas.texture_id, size)
                    .uv(atlas.get_uv_for_texture_id(texture_id)),
            )
        });
}
