use std::hash::Hash;

use bevy::ecs::query::WorldQuery;
use bevy_egui::EguiContext;
use bevy_inspector_egui::egui::epaint::text::cursor;
use bevy_pixel_camera::PixelProjection;
use leafwing_input_manager::prelude::*;

use crate::{
    config::{EngineState, UpdateStageLabel},
    game::map::Position,
    prelude::*,
};

pub mod camera;
pub mod input;
pub mod selection;
pub mod viewer;

pub use input::*;
pub use selection::*;
pub use viewer::*;
pub struct InputPlugin {}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bevy::input::system::exit_on_esc_system)
            .add_system(bevy::window::exit_on_window_close_system)
            .add_plugin(InputManagerPlugin::<InputActions>::default())
            .add_enter_system(config::EngineState::InGame, setup_input_map)
            .add_system_set(
                ConditionSet::new()
                    .label_and_after(UpdateStageLabel::UpdateCache)
                    .run_in_state(EngineState::InGame)
                    .with_system(add_new_entitites_viewer_map)
                    .with_system(remove_entities_from_viewer_map)
                    .with_system(update_position_on_viewer_map)
                    .with_system(cursor_position)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .label_and_after(UpdateStageLabel::Input)
                    .run_in_state(EngineState::InGame)
                    .with_system(input_to_game_actions)
                    .with_system(drag_selection)
                    .with_system(hover)
                    .with_system(select)
                    .into(),
            )
            .add_plugin(camera::CameraPlugin {});
    }
}

#[derive(Bundle, Debug, Default)]
pub struct ViewerBundle {
    pub viewer: Viewer,
    pub selected: Selected,
    pub map: ViewerMap,
    #[bundle]
    pub cursor: CursorBundle,
}

#[derive(Bundle, Debug, Default)]
pub struct CursorBundle {
    pub cursor_pixel_position: CursorPixelPosition,
    pub cursor_position: CursorPosition,
    pub target_time: CursorTargetTime,
    pub drag_select: CursorDragSelect,
    pub selection_target: CursorSelectionTarget,
    // pub interaction_target - what will happen if you right click
    // pub tooltip_target - what tooltip to show for this
    pub debug_tooltip: CursorDebugTooltipTarget,
}

#[derive(Component, Debug, Default)]
pub struct CursorPixelPosition(Vec2);

#[derive(Component, Debug, Default)]
pub struct CursorPosition {
    // Exact point on map where we are pointing. None indicates we are out of map bounds
    pub exact_position_option: Option<Position>,
    // Position bound by the edges of map, so being outside of map will give closest point on the map
    pub bound_position: Position,

    pub in_gui: bool,
}

pub fn cursor_position(
    windows: Res<Windows>,
    egui_context_option: Option<ResMut<EguiContext>>,
    camera_transform_query: Query<(&Camera, &Transform), With<PixelProjection>>,
    map_query: Query<&game::map::Map>,
    mut cursor_query: Query<(&mut CursorPixelPosition, &mut CursorPosition), With<Viewer>>,
) {
    let window = windows.get_primary().unwrap();
    let (camera, camera_transform) = camera_transform_query.single();
    if let Some(pixel_position) =
        camera::camera_position_to_pixel_position(window, camera, camera_transform)
    {
        let map = map_query.single();
        let (exact_position_option, bound_position) =
            map.pixel_position_to_cursor_position(pixel_position);
        let in_gui = match egui_context_option {
            Some(mut egui_context) => egui_context.ctx_mut().wants_pointer_input(),
            None => false,
        };
        let (mut cursor_pixel_position, mut cursor_position) = cursor_query.single_mut();
        cursor_pixel_position.0 = pixel_position;

        if cursor_position.in_gui != in_gui {
            cursor_position.in_gui = in_gui;
        }

        if cursor_position.exact_position_option != exact_position_option {
            cursor_position.exact_position_option = exact_position_option
        }

        if cursor_position.bound_position != bound_position {
            cursor_position.bound_position = bound_position;
        }
    }
}

fn drag_selection(
    input_action_query: Query<&ActionState<InputActions>>,
    mut viewer_query: Query<
        (
            &ViewerMap,
            &CursorPixelPosition,
            &CursorPosition,
            ChangeTrackers<CursorPosition>,
            &CursorSelectionTarget,
            &mut CursorDragSelect,
        ),
        With<Viewer>,
    >,
    selectable_query: Query<Entity, With<Selectable>>,
) {
    let input_action_state = input_action_query.single();
    let pressed = input_action_state.pressed(InputActions::Select);
    let just_pressed = input_action_state.just_pressed(InputActions::Select);

    let (
        viewer_map,
        pixel_position,
        cursor_position,
        cursor_position_tracker,
        cursor_selection_target,
        mut cursor_drag_select,
    ) = viewer_query.single_mut();

    if let CursorDragSelectType::Dragging(_, anchor_position, selection) = &mut cursor_drag_select.0
    {
        if cursor_position_tracker.is_changed() {
            let bounding_box = (*anchor_position, cursor_position.bound_position);
            let mut selections = Vec::new();
            for entity in viewer_map.entities_in_bounding_box(&bounding_box) {
                if let Ok(selectable_entity) = selectable_query.get(entity.entity()) {
                    selections.push(selectable_entity);
                }
            }
            selection.select_units(selections);
        }
    } else if pressed && !just_pressed {
        let mut selection = Selection::default();
        selection.select_units(cursor_selection_target.0.entities());
        cursor_drag_select.0 = CursorDragSelectType::Dragging(
            pixel_position.0,
            cursor_position.bound_position,
            selection,
        );
    }
}

fn hover(
    mut viewer_query: Query<
        (
            &ViewerMap,
            &CursorPosition,
            &mut CursorSelectionTarget,
            &mut CursorTargetTime,
            &mut CursorDebugTooltipTarget,
        ),
        (With<Viewer>, Changed<CursorPosition>),
    >,
    selectable_query: Query<Entity, With<Selectable>>,
) {
    if let Ok((
        viewer_map,
        cursor_position,
        mut cursor_selection_target,
        mut cursor_target_time,
        mut cursor_debug_tooltip_target,
    )) = viewer_query.get_single_mut()
    {
        let mut nothing_to_select = true;
        if !cursor_position.in_gui {
            if let Some(cursor_position) = cursor_position.exact_position_option {
                if let Some(possible_entities) = viewer_map.entities_at_position(&cursor_position) {
                    cursor_debug_tooltip_target.entities =
                        Some(possible_entities.iter().copied().collect());
                    for entity in possible_entities {
                        if let Ok(selectable_entity) = selectable_query.get(entity.entity()) {
                            nothing_to_select = false;
                            if !cursor_selection_target.0.is_selected(selectable_entity) {
                                cursor_selection_target.0.select_unit(selectable_entity);
                                cursor_target_time.0.reset();
                                cursor_target_time.0.unpause();
                            }
                            break;
                        }
                    }
                }
            }
        }

        if nothing_to_select {
            cursor_selection_target.0.clear();
            cursor_target_time.0.reset();
            cursor_target_time.0.pause();
        }
    }
}

fn select(
    input_action_query: Query<&ActionState<InputActions>>,
    mut viewer_query: Query<
        (
            &CursorPosition,
            &mut Selected,
            &CursorSelectionTarget,
            &mut CursorDragSelect,
        ),
        With<Viewer>,
    >,
) {
    let input_action_state = input_action_query.single();
    let just_released = input_action_state.just_released(InputActions::Select);
    if just_released {
        let (cursor_position, mut selected, cursor_selection_target, mut cursor_drag_select) =
            viewer_query.single_mut();
        if let CursorDragSelectType::Dragging(_, _, selection_target) = &mut cursor_drag_select.0 {
            selected.0.select_units(selection_target.entities());
            cursor_drag_select.0 = CursorDragSelectType::None;
        } else if !cursor_position.in_gui {
            selected
                .0
                .select_units(cursor_selection_target.0.entities());
        }
    }
}
