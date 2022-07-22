use bevy_egui::{egui, EguiContext, EguiSettings};
use bevy_pixel_camera::PixelProjection;

use crate::{
    config::{EngineState, UiSyncLabel},
    game::{
        map::{Map, Position},
        units::{Unit, UnitType},
    },
    gui::{GuiContext, TextureType},
    prelude::*,
    ui::{camera, Selected, Viewer},
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
    camera_transform_query: Query<(&Camera, &Transform), With<PixelProjection>>,
    map_query: Query<&Map>,
    // TODO: Visible only
    unit_query: Query<(Entity, &UnitType, &Position), With<Unit>>,
    selected_query: Query<&Selected, With<Viewer>>,
) {
    let window = windows.get_primary().unwrap();
    let (camera, camera_transform) = camera_transform_query.single();
    let map = map_query.single();
    let Selected(selection) = selected_query.single();
    for (entity, _unit, position) in unit_query.iter() {
        let scale = egui_settings.scale_factor;
        let egui_size = gui_context.egui_window_size().unwrap();
        let pixel_position = map.position_to_pixel_position(position);
        let position_next = &mut position.clone();
        position_next.move_to_direction(&Direction::NorthEast);
        let pixel_position_next = map.position_to_pixel_position(position_next);
        if let (Some(viewport_position), Some(viewport_position_next)) = (
            camera::world_to_viewport(window, camera, camera_transform, pixel_position),
            camera::world_to_viewport(window, camera, camera_transform, pixel_position_next),
        ) {
            let egui_position = egui::pos2(
                viewport_position.x / scale as f32,
                egui_size.y - viewport_position.y / scale as f32,
            );
            let egui_position_next = egui::pos2(
                viewport_position_next.x / scale as f32,
                egui_size.y - viewport_position_next.y / scale as f32,
            );

            let tile_size = egui::vec2(
                egui_position_next.x - egui_position.x,
                egui_position.y - egui_position_next.y,
            );

            egui::Area::new(format!("unit badge: {:?}", entity.id()))
                .fixed_pos(egui_position + egui::vec2(tile_size.x * 0.125, tile_size.y * -1.8))
                .order(egui::Order::Tooltip)
                .drag_bounds(egui::Rect::EVERYTHING)
                .show(egui_context.ctx_mut(), |ui| {
                    let size = egui::vec2(tile_size.x * 0.75, tile_size.y);
                    let atlas = gui_context.get_texture_atlas(TextureType::Other, "badges");
                    let mut texture_id = 5;
                    if selection.is_selected(entity) {
                        texture_id += 8;
                    }
                    ui.add(
                        egui::Image::new(atlas.texture_id, size)
                            .uv(atlas.get_uv_for_texture_id(texture_id)),
                    );
                });
        }
    }
}
