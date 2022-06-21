use crate::prelude::*;

fn icon_button_ui(
    ui: &mut egui::Ui,
    button_texture_id: egui::TextureId,
    icon_texture_id: egui::TextureId,
    size: egui::Vec2,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    if ui.is_rect_visible(rect) {
        let uv_rect = {
            if response.clicked() || !ui.is_enabled() {
                egui::Rect::from_min_max(egui::pos2(0.66, 0.), egui::pos2(1., 1.))
            } else if response.hovered() {
                egui::Rect::from_min_max(egui::pos2(0.33, 0.), egui::pos2(0.66, 1.))
            } else {
                egui::Rect::from_min_max(egui::pos2(0., 0.), egui::pos2(0.33, 1.))
            }
        };
        let image = egui::Image::new(button_texture_id, size).uv(uv_rect);
        image.paint_at(ui, rect);
        let icon = egui::Image::new(icon_texture_id, size);
        icon.paint_at(ui, rect.shrink(4.))
    }

    response
}

pub fn icon_button(
    button_texture_id: egui::TextureId,
    icon_texture_id: egui::TextureId,
    size: egui::Vec2,
) -> impl egui::Widget {
    move |ui: &mut egui::Ui| icon_button_ui(ui, button_texture_id, icon_texture_id, size)
}
