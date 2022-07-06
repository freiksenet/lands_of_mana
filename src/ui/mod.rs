use std::hash::Hash;

use bevy_egui::EguiContext;
use bevy_pixel_camera::PixelProjection;
use leafwing_input_manager::prelude::*;

use crate::{
    config::{EngineState, UpdateStageLabel},
    prelude::*,
};

pub mod camera;
pub mod selection;
pub mod viewer;

pub use selection::*;
pub use viewer::*;
pub struct InputPlugin {}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bevy::input::system::exit_on_esc_system)
            .add_system(bevy::window::exit_on_window_close_system)
            .add_plugin(InputManagerPlugin::<InputActions>::default())
            .add_enter_system(config::EngineState::InGame, setup)
            .add_system_set(
                ConditionSet::new()
                    .label_and_after(UpdateStageLabel::UpdateCache)
                    .run_in_state(EngineState::InGame)
                    .with_system(add_new_entitites_viewer_map)
                    .with_system(remove_entities_from_viewer_map)
                    .with_system(update_position_on_viewer_map)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .label_and_after(UpdateStageLabel::Input)
                    .run_in_state(EngineState::InGame)
                    .with_system(input_to_game_actions)
                    .with_system(interact)
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
    pub cursor_target: CursorTargetBundle,
}

fn setup(mut commands: Commands, viewer_query: Query<Entity, With<Viewer>>) {
    let viewer_entity = viewer_query.single();
    let mut input_map = InputMap::new([
        // pause / resume
        (KeyCode::Space, InputActions::TogglePause),
        (KeyCode::W, InputActions::CameraMoveNorth),
        (KeyCode::S, InputActions::CameraMoveSouth),
        (KeyCode::A, InputActions::CameraMoveWest),
        (KeyCode::D, InputActions::CameraMoveEast),
        (KeyCode::Z, InputActions::CameraZoomIn),
        (KeyCode::X, InputActions::CameraZoomOut),
    ]);
    input_map.insert(MouseButton::Left, InputActions::Select);
    input_map.insert(MouseButton::Right, InputActions::Contextual);

    commands
        .entity(viewer_entity)
        .insert_bundle(InputManagerBundle::<InputActions> {
            action_state: ActionState::default(),
            input_map,
        });
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum InputActions {
    Pause,
    Resume,
    TogglePause,

    CameraMoveNorth,
    CameraMoveSouth,
    CameraMoveWest,
    CameraMoveEast,
    CameraZoomIn,
    CameraZoomOut,

    // Stuff that is left click, usually "select something"
    Select,
    // Stuff that is right click, usually "do something with current context"
    Contextual,
}

fn input_to_game_actions(
    game_state: Res<CurrentState<game::InGameState>>,
    input_action_query: Query<&ActionState<InputActions>>,
    mut world_action_query: Query<&mut ActionState<game::actions::WorldActions>>,
) {
    let input_action_state = input_action_query.single();
    let mut world_action_state = world_action_query.single_mut();

    if input_action_state.just_pressed(InputActions::Pause)
        || (input_action_state.just_pressed(InputActions::TogglePause)
            && game_state.0 == game::InGameState::Running)
    {
        world_action_state.press(game::actions::WorldActions::Pause)
    }

    if input_action_state.just_pressed(InputActions::Resume)
        || (input_action_state.just_pressed(InputActions::TogglePause)
            && game_state.0 == game::InGameState::Paused)
    {
        world_action_state.press(game::actions::WorldActions::Resume)
    }
}

fn interact(
    windows: Res<Windows>,
    egui_context_option: Option<ResMut<EguiContext>>,
    camera_transform_query: Query<(&Camera, &Transform), With<PixelProjection>>,
    map_query: Query<&game::map::Map>,
    input_action_query: Query<&ActionState<InputActions>>,
    mut viewer_query: Query<
        (
            &ViewerMap,
            &mut Selected,
            &mut CursorDragSelect,
            &mut CursorTargetTime,
            &mut CursorSelectionTarget,
            &mut CursorDebugTooltipTarget,
        ),
        With<Viewer>,
    >,
    selectable_query: Query<Entity, With<Selectable>>,
) {
    let input_action_state = input_action_query.single();
    let ui_contains_cursor = match egui_context_option {
        Some(mut egui_context) => egui_context.ctx_mut().wants_pointer_input(),
        None => false,
    };

    if !ui_contains_cursor {
        let window = windows.get_primary().unwrap();

        let (camera, camera_transform) = camera_transform_query.single();
        if let Some(pixel_position) =
            camera::camera_position_to_pixel_position(window, camera, camera_transform)
        {
            let map = map_query.single();
            let cursor_position_option = map.pixel_position_to_position(pixel_position);
            let bound_cursor_position = map.pixel_position_to_position_or_map_bound(pixel_position);
            let (
                viewer_map,
                mut selected,
                mut cursor_drag_select,
                mut cursor_target_time,
                mut cursor_selection_target,
                mut cursor_debug_tooltip_target,
            ) = viewer_query.single_mut();

            let pressed = input_action_state.pressed(InputActions::Select);
            let just_pressed = input_action_state.just_pressed(InputActions::Select);
            let just_released = input_action_state.just_released(InputActions::Select);

            if let CursorDragSelectType::Dragging(anchor) = cursor_drag_select.0 {
                let bounding_box = (
                    map.pixel_position_to_position_or_map_bound(anchor),
                    bound_cursor_position,
                );
                let mut selections = Vec::new();
                for entity in viewer_map.entities_in_bounding_box(&bounding_box) {
                    if let Ok(selectable_entity) = selectable_query.get(entity.entity()) {
                        selections.push(selectable_entity);
                    }
                }
                if just_released {
                    cursor_drag_select.0 = CursorDragSelectType::None;
                    selected.0.select_units(selections)
                } else {
                    cursor_selection_target.0.select_units(selections);
                }
            } else if pressed && !just_pressed {
                cursor_drag_select.0 = CursorDragSelectType::Dragging(pixel_position);
            } else {
                let mut trying_to_select: Option<bool> =
                    if input_action_state.just_released(InputActions::Select) {
                        Some(false)
                    } else {
                        None
                    };
                let mut not_hovering = true;

                if let Some(cursor_position) = cursor_position_option {
                    if let Some(possible_entities) =
                        viewer_map.entities_at_position(&cursor_position)
                    {
                        cursor_debug_tooltip_target.entities =
                            Some(possible_entities.iter().copied().collect());
                        for entity in possible_entities {
                            if let Ok(selectable_entity) = selectable_query.get(entity.entity()) {
                                not_hovering = false;
                                if !cursor_selection_target.0.is_selected(selectable_entity) {
                                    cursor_selection_target.0.select_unit(selectable_entity);
                                    cursor_target_time.0.reset();
                                    cursor_target_time.0.unpause();
                                }

                                if trying_to_select.is_some()
                                    && !selected.0.is_selected_alone(selectable_entity)
                                {
                                    selected.0.select_unit(selectable_entity);
                                    trying_to_select = Some(true);
                                }
                            }
                        }
                    } else {
                        cursor_debug_tooltip_target.entities = None;
                    }
                } else {
                    cursor_debug_tooltip_target.entities = None;
                }

                match trying_to_select {
                    Some(have_selected) if !have_selected => {
                        selected.0.clear();
                    }
                    _ => {}
                }
                if not_hovering && !cursor_selection_target.0.is_empty() {
                    cursor_selection_target.0.clear();
                    cursor_target_time.0.reset();
                    cursor_target_time.0.pause();
                }
            }
        }
    }
}
