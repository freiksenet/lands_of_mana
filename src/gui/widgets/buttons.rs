use crate::prelude::{egui::emath::RectTransform, gui::widgets::*, *};

fn icon_button_ui(
    ui: &mut egui::Ui,
    button_texture_id: egui::TextureId,
    icon_texture_id: egui::TextureId,
    size: egui::Vec2,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    if ui.is_rect_visible(rect) {
        let uv_rect = uv_for_button_state(ui, &response);
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

fn button_ui(
    ui: &mut egui::Ui,
    (button_texture_size, button_texture_id): (egui::Vec2, egui::TextureId),
    text: egui::WidgetText,
    min_size: egui::Vec2,
    icon: Option<(egui::Vec2, egui::TextureId)>,
) -> egui::Response {
    let mut button_padding = ui.spacing().button_padding;
    button_padding.x *= button_texture_size.x / 8.;
    let icon_spacing = ui.spacing().icon_spacing * (button_texture_size.x / 8.);

    let total_extra = button_padding + button_padding;

    let mut offset_vec = button_texture_size / 8.;
    offset_vec.x = 0.;

    let wrap_width = ui.available_width() - total_extra.x;

    let mut text_job = text.into_text_job(
        ui.style(),
        egui::FontSelection::Style(egui::TextStyle::Button),
        egui::Align::Center,
    );
    text_job.job.wrap.max_width = wrap_width;
    text_job.job.halign = if icon.is_some() {
        egui::Align::LEFT
    } else {
        egui::Align::Center
    };
    text_job.job.first_row_min_height = min_size.y - offset_vec.y;

    let text = text_job.into_galley(&ui.fonts());

    let mut desired_size = text.size();
    desired_size.x += button_padding.x * 2.;

    if let Some((icon_size, _)) = icon {
        desired_size.x += icon_size.x + icon_spacing;
        desired_size.y = desired_size.y.max(icon_size.y);
    }
    desired_size = egui::NumExt::at_least(desired_size, min_size);

    let (rect, response) = ui.allocate_at_least(desired_size, egui::Sense::click());
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, text.text()));

    if ui.is_rect_visible(rect) {
        let uv_rect = uv_for_button_state(ui, &response);
        let nine_patch = NinePatch::new(button_texture_id, button_texture_size)
            .rect(rect)
            .uv(uv_rect)
            .begin(ui);

        let text_pos = if let Some((icon_size, icon_texture_id)) = icon {
            egui::Image::new(icon_texture_id, icon_size).paint_at(
                ui,
                egui::Rect::from_min_size(
                    rect.left_center() + egui::vec2(button_padding.x, -icon_size.y / 2.)
                        - offset_vec,
                    icon_size,
                ),
            );
            let mut text_pos = rect.left_top();
            text_pos.x += button_padding.x + icon_size.x + icon_spacing;
            text_pos
        } else {
            rect.center_top()
        };

        text.paint_with_visuals(ui.painter(), text_pos, ui.style().interact(&response));

        nine_patch.end(ui);
    }

    response
}

pub fn button(
    button_texture: (egui::Vec2, egui::TextureId),
    text: egui::WidgetText,
    min_size: egui::Vec2,
    icon: Option<(egui::Vec2, egui::TextureId)>,
) -> impl egui::Widget {
    move |ui: &mut egui::Ui| button_ui(ui, button_texture, text, min_size, icon)
}

pub fn uv_for_button_state(ui: &egui::Ui, response: &egui::Response) -> egui::Rect {
    let transform = RectTransform::from_to(
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(72., 24.)),
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1., 1.)),
    );
    transform.transform_rect(if response.clicked() || !ui.is_enabled() {
        egui::Rect::from_min_max(egui::pos2(48., 0.), egui::pos2(72., 24.))
    } else if response.hovered() {
        egui::Rect::from_min_max(egui::pos2(24., 0.), egui::pos2(48., 24.))
    } else {
        egui::Rect::from_min_max(egui::pos2(0., 0.), egui::pos2(24., 24.))
    })
}
