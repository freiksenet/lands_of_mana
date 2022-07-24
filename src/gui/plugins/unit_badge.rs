use bevy_egui::{egui, EguiContext, EguiSettings};
use bevy_pixel_camera::PixelProjection;

use crate::{
    config::{EngineState, UiSyncLabel},
    game::{
        map::{Map, Position},
        units::{Unit, UnitOrder, UnitOrders, UnitType},
    },
    gui::{GuiContext, TextureType},
    prelude::*,
    ui::{
        camera::{self},
        Selected, Viewer,
    },
};

pub struct UnitBadgePlugin {}

impl Plugin for UnitBadgePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            config::Stage::UiSync,
            ConditionSet::new()
                .run_in_state(EngineState::InGame)
                .label_and_after(UiSyncLabel::Update)
                .with_system(unit_badges)
                .into(),
        );
    }
}

fn unit_badges(
    windows: Res<Windows>,
    mut egui_context: ResMut<EguiContext>,
    egui_settings: Res<EguiSettings>,
    gui_context: Res<GuiContext>,
    camera_transform_query: Query<(&Camera, &GlobalTransform), With<PixelProjection>>,
    map_query: Query<&Map>,
    // TODO: Visible only
    unit_query: Query<(Entity, &UnitType, &Position, &GlobalTransform, &UnitOrders), With<Unit>>,
    selected_query: Query<&Selected, With<Viewer>>,
) {
    let window = windows.get_primary().unwrap();
    let (camera, camera_transform) = camera_transform_query.single();
    let map = map_query.single();
    let Selected(selection) = selected_query.single();
    for (entity, _unit, position, transform, unit_orders) in unit_query.iter() {
        let scale = egui_settings.scale_factor;
        let egui_size = gui_context.egui_window_size().unwrap();
        let pixel_position = transform.translation.truncate();
        let corner_pixel_position = map.position_to_pixel_position(position);
        let position_next = &mut position.clone();
        position_next.move_to_direction(&Direction::NorthEast);
        let other_corner_pixel_position = map.position_to_pixel_position(position_next);
        if let (Some(viewport_position), Some(corner_position), Some(other_corner_position)) = (
            camera::world_to_viewport(window, camera, camera_transform, pixel_position),
            camera::world_to_viewport(window, camera, camera_transform, corner_pixel_position),
            camera::world_to_viewport(
                window,
                camera,
                camera_transform,
                other_corner_pixel_position,
            ),
        ) {
            let egui_position = egui::pos2(
                viewport_position.x / scale as f32,
                egui_size.y - viewport_position.y / scale as f32,
            );
            let egui_corner_position = egui::pos2(
                corner_position.x / scale as f32,
                egui_size.y - corner_position.y / scale as f32,
            );
            let egui_other_corner_position = egui::pos2(
                other_corner_position.x / scale as f32,
                egui_size.y - other_corner_position.y / scale as f32,
            );

            let tile_size = egui::vec2(
                egui_other_corner_position.x - egui_corner_position.x,
                egui_corner_position.y - egui_other_corner_position.y,
            );

            let position = egui_position + egui::vec2(tile_size.x * 0.125, tile_size.y * -1.9);
            egui::Area::new(format!("unit badge: {:?}", entity.id()))
                .fixed_pos(position)
                .order(egui::Order::Tooltip)
                .drag_bounds(egui::Rect::EVERYTHING)
                .show(egui_context.ctx_mut(), |ui| {
                    let size = egui::vec2(tile_size.x * 0.75, tile_size.y * 1.125);
                    let atlas = gui_context.get_texture_atlas(TextureType::Other, "unit_badges");
                    let mut texture_id = 0;
                    if selection.is_selected(entity) {
                        texture_id += 1;
                    }
                    ui.add(
                        egui::Image::new(atlas.texture_id, size)
                            .uv(atlas.get_uv_for_texture_id(texture_id)),
                    );
                    let base_rect =
                        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(12., 18.));
                    let current_rect = egui::Rect::from_min_size(position, size);
                    let resize_transform =
                        egui::emath::RectTransform::from_to(base_rect, current_rect);

                    let unit_badge_icons =
                        gui_context.get_texture_atlas(TextureType::Other, "unit_badge_icons");
                    let order_base_rect =
                        egui::Rect::from_min_size(egui::pos2(4., 12.), egui::vec2(4., 4.));

                    let order_rect = resize_transform.transform_rect(order_base_rect);
                    let image = egui::Image::new(unit_badge_icons.texture_id, order_rect.size())
                        .uv(unit_badge_icons.get_uv_for_texture_id(
                            match unit_orders.peek_order() {
                                Some(UnitOrder::Move { .. })
                                | Some(UnitOrder::MoveToPosition { .. }) => 6,
                                _ => 0,
                            },
                        ));
                    image.paint_at(ui, order_rect);

                    let nation_unit_icon_base_rect =
                        egui::Rect::from_min_size(egui::pos2(2., 4.), egui::vec2(6., 6.));
                    let nation_unit_icon_rect =
                        resize_transform.transform_rect(nation_unit_icon_base_rect);
                    let image =
                        egui::Image::new(unit_badge_icons.texture_id, nation_unit_icon_rect.size())
                            .uv(unit_badge_icons.get_uv_for_texture_id(15));
                    image.paint_at(ui, nation_unit_icon_rect);

                    let weapon_unit_icon_base_rect =
                        egui::Rect::from_min_size(egui::pos2(4., 4.), egui::vec2(6., 6.));
                    let weapon_unit_icon_rect =
                        resize_transform.transform_rect(weapon_unit_icon_base_rect);
                    let image =
                        egui::Image::new(unit_badge_icons.texture_id, weapon_unit_icon_rect.size())
                            .uv(unit_badge_icons.get_uv_for_texture_id(1));
                    image.paint_at(ui, weapon_unit_icon_rect);
                });
        }
    }
}
