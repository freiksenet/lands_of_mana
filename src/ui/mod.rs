use bevy_egui::EguiContext;
use bevy_pixel_camera::PixelProjection;
use leafwing_input_manager::prelude::*;

use crate::{
    config::{EngineState, UpdateStageLabel},
    game::{
        map::Position,
        province::City,
        units::{Unit, UnitOrder, UnitOrders},
    },
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
                    .with_system(contextual)
                    .into(),
            )
            .add_plugin(camera::CameraPlugin {});
    }
}

#[derive(Bundle, Default)]
pub struct ViewerBundle {
    pub viewer: Viewer,
    pub selected: Selected,
    pub map: ViewerMap,
    #[bundle]
    pub cursor: CursorBundle,
}

#[derive(Bundle, Default)]
pub struct CursorBundle {
    pub cursor_pixel_position: CursorPixelPosition,
    pub cursor_type: CursorType,
    pub cursor_position: CursorPosition,
    pub target_time: CursorTargetTime,
    pub drag_select: CursorDragSelect,
    pub selection_target: CursorSelectionTarget,
    // pub tooltip_target - what tooltip to show for this
    pub debug_tooltip: CursorDebugTooltipTarget,
}
#[derive(Component, Debug, Default)]
pub struct CursorType {
    kind: CursorKind,
    color: CursorColor,
}

impl CursorType {
    pub fn texture_id(&self) -> usize {
        (match self.color {
            CursorColor::White => 0,
            CursorColor::Red => 8,
            CursorColor::Green => 16,
            CursorColor::Plain => 24,
        }) + (match self.kind {
            CursorKind::Arrow => 0,
            CursorKind::Triangle => 1,
            CursorKind::ShortTriangle => 2,
            CursorKind::ThinTriangle => 3,
            CursorKind::SmallArrow => 4,
            CursorKind::SmallTriangle => 5,
            CursorKind::SmallShortTriangle => 6,
            CursorKind::SmallThinTriangle => 7,
            CursorKind::Hand => 32,
            CursorKind::HandFinger => 33,
            CursorKind::SelectionBox => 34,
            CursorKind::Target => 35,
            CursorKind::WideTarget => 36,
            CursorKind::FlatTarget => 37,
            CursorKind::Talk => 38,
            CursorKind::Settings => 39,
        })
    }

    pub fn from_winit(cursor_icon: bevy::window::CursorIcon, is_clicked: bool) -> CursorType {
        let color = if is_clicked {
            CursorColor::White
        } else {
            CursorColor::Plain
        };
        CursorType {
            color,
            kind: match cursor_icon {
                CursorIcon::Default => CursorKind::default(),
                CursorIcon::Crosshair => CursorKind::Target,
                CursorIcon::Hand => CursorKind::Hand,
                CursorIcon::Arrow => CursorKind::Triangle,
                // CursorIcon::Move => todo!(),
                // CursorIcon::Text => todo!(),
                // CursorIcon::Wait => todo!(),
                // CursorIcon::Help => todo!(),
                // CursorIcon::Progress => todo!(),
                // CursorIcon::NotAllowed => todo!(),
                // CursorIcon::ContextMenu => todo!(),
                // CursorIcon::Cell => todo!(),
                // CursorIcon::VerticalText => todo!(),
                // CursorIcon::Alias => todo!(),
                // CursorIcon::Copy => todo!(),
                // CursorIcon::NoDrop => todo!(),
                // CursorIcon::Grab => todo!(),
                // CursorIcon::Grabbing => todo!(),
                // CursorIcon::AllScroll => todo!(),
                // CursorIcon::ZoomIn => todo!(),
                // CursorIcon::ZoomOut => todo!(),
                // CursorIcon::EResize => todo!(),
                // CursorIcon::NResize => todo!(),
                // CursorIcon::NeResize => todo!(),
                // CursorIcon::NwResize => todo!(),
                // CursorIcon::SResize => todo!(),
                // CursorIcon::SeResize => todo!(),
                // CursorIcon::SwResize => todo!(),
                // CursorIcon::WResize => todo!(),
                // CursorIcon::EwResize => todo!(),
                // CursorIcon::NsResize => todo!(),
                // CursorIcon::NeswResize => todo!(),
                // CursorIcon::NwseResize => todo!(),
                // CursorIcon::ColResize => todo!(),
                // CursorIcon::RowResize => todo!(),
                _ => CursorKind::default(),
            },
        }
    }
}

#[derive(Debug, Default)]
pub enum CursorColor {
    #[default]
    Plain,
    White,
    Green,
    Red,
}

#[derive(Debug, Default)]
pub enum CursorKind {
    #[default]
    Arrow,
    Triangle,
    ShortTriangle,
    ThinTriangle,
    SmallArrow,
    SmallTriangle,
    SmallShortTriangle,
    SmallThinTriangle,
    Hand,
    HandFinger,
    SelectionBox,
    Target,
    WideTarget,
    FlatTarget,
    Talk,
    Settings,
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
    if let Some(pixel_position) = camera::cursor_to_world(window, camera, camera_transform) {
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
            &mut CursorDragSelect,
        ),
        With<Viewer>,
    >,
    selectable_query: Query<Entity, (With<Selectable>, With<Unit>)>,
) {
    let input_action_state = input_action_query.single();
    let pressed = input_action_state.pressed(InputActions::Select);
    let just_pressed = input_action_state.just_pressed(InputActions::Select);

    let (
        viewer_map,
        pixel_position,
        cursor_position,
        cursor_position_tracker,
        mut cursor_drag_select,
    ) = viewer_query.single_mut();

    if cursor_position_tracker.is_changed() {
        if let CursorDragSelectType::Dragging(_, anchor_position, selection) =
            &mut cursor_drag_select.0
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
        } else if pressed && !just_pressed && !cursor_position.in_gui {
            let selection = Selection::default();
            cursor_drag_select.0 = CursorDragSelectType::Dragging(
                pixel_position.0,
                cursor_position.bound_position,
                selection,
            );
        }
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
    selectable_unit_query: Query<Entity, (With<Unit>, With<Selectable>)>,
    selectable_city_query: Query<Entity, (With<City>, With<Selectable>)>,
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
                        match entity {
                            EntityOnTile::City { city_entity, .. } => {
                                if let Ok(selectable_entity) =
                                    selectable_city_query.get(*city_entity)
                                {
                                    nothing_to_select = false;
                                    if !cursor_selection_target.0.is_selected(selectable_entity) {
                                        cursor_selection_target.0.select_city(selectable_entity);
                                        cursor_target_time.0.reset();
                                        cursor_target_time.0.unpause();
                                    }
                                    break;
                                }
                            }
                            EntityOnTile::Unit(unit_entity) => {
                                if let Ok(selectable_entity) =
                                    selectable_unit_query.get(*unit_entity)
                                {
                                    nothing_to_select = false;
                                    if !cursor_selection_target.0.is_selected(selectable_entity) {
                                        cursor_selection_target.0.select_unit(selectable_entity);
                                        cursor_target_time.0.reset();
                                        cursor_target_time.0.unpause();
                                    }
                                    break;
                                }
                            }
                            _ => {}
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
            selected.0.select_entities(selection_target.entities());
            cursor_drag_select.0 = CursorDragSelectType::None;
        } else if !cursor_position.in_gui {
            selected
                .0
                .select_entities(cursor_selection_target.0.entities());
        }
    }
}

fn contextual(
    input_action_query: Query<&ActionState<InputActions>>,
    viewer_query: Query<(&CursorPosition, &Selected), With<Viewer>>,
    mut unit_orders_query: Query<&mut UnitOrders, With<Unit>>,
) {
    let input_action_state = input_action_query.single();
    let just_released = input_action_state.just_released(InputActions::Contextual);
    let (cursor_position, selected) = viewer_query.single();
    if just_released && cursor_position.exact_position_option.is_some() && !selected.0.is_empty() {
        for selected_entity in selected.0.entities() {
            match selected_entity {
                SelectedEntity::Unit(entity) => {
                    if let Ok(mut unit_orders) = unit_orders_query.get_mut(*entity) {
                        unit_orders.new_order(UnitOrder::MoveToPosition {
                            target_position: cursor_position.exact_position_option.unwrap(),
                        })
                    }
                }
                SelectedEntity::City(_) => todo!(),
            }
        }
    }
}
