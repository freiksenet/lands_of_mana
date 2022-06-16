use bevy::{core::Stopwatch, utils::HashSet};
use bevy_pixel_camera::PixelProjection;
use kayak_ui::bevy::BevyContext;
use leafwing_input_manager::prelude::*;

use crate::{game::map::Position, prelude::*};

pub mod camera;
pub struct InputPlugin {}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bevy::input::system::exit_on_esc_system)
            .add_system(bevy::window::exit_on_window_close_system)
            .add_plugin(InputManagerPlugin::<InputActions>::default())
            .add_enter_system(config::EngineState::InGame, setup)
            .add_system_set(
                ConditionSet::new()
                    .label_and_after(config::UpdateStageLabel::Input)
                    .run_in_state(config::EngineState::InGame)
                    .with_system(input_to_game_actions)
                    .with_system(interact)
                    .into(),
            )
            .add_plugin(camera::CameraPlugin {});
    }
}

fn setup(mut commands: Commands, viewer_query: Query<Entity, With<Viewer>>) {
    let viewer_entity = viewer_query.single();
    let mut input_map = InputMap::new([
        // pause / resume
        (InputActions::TogglePause, KeyCode::Space),
        (InputActions::CameraMoveNorth, KeyCode::W),
        (InputActions::CameraMoveSouth, KeyCode::S),
        (InputActions::CameraMoveWest, KeyCode::A),
        (InputActions::CameraMoveEast, KeyCode::D),
        (InputActions::CameraZoomIn, KeyCode::Z),
        (InputActions::CameraZoomOut, KeyCode::X),
    ]);
    input_map.insert(InputActions::Select, MouseButton::Left);
    input_map.insert(InputActions::Contextual, MouseButton::Right);

    commands
        .entity(viewer_entity)
        .insert_bundle(InputManagerBundle::<InputActions> {
            action_state: ActionState::default(),
            input_map,
        });
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
    kayak_context_option: Option<Res<BevyContext>>,
    camera_transform_query: Query<(&Camera, &Transform), With<PixelProjection>>,
    map_query: Query<&game::map::Map>,
    input_action_query: Query<&ActionState<InputActions>>,
    mut viewer_query: Query<
        (
            &mut Selection,
            &mut CursorDragSelect,
            &mut CursorTargetTime,
            &mut CursorSelectionTarget,
        ),
        With<Viewer>,
    >,
    selectable_query: Query<(Entity, &game::map::Position), With<Selectable>>,
) {
    let input_action_state = input_action_query.single();
    let ui_contains_cursor = match kayak_context_option {
        Some(kayak_context) => kayak_context.contains_cursor(),
        None => false,
    };

    if !ui_contains_cursor {
        let window = windows.get_primary().unwrap();

        let (camera, camera_transform) = camera_transform_query.single();
        if let Some(pixel_position) =
            camera::camera_position_to_pixel_position(window, camera, camera_transform)
        {
            let map = map_query.single();
            let cursor_position = map.pixel_position_to_position(pixel_position);
            let (
                mut selection,
                mut cursor_drag_select,
                mut cursor_target_time,
                mut cursor_selection_target,
            ) = viewer_query.single_mut();

            let pressed = input_action_state.pressed(InputActions::Select);
            let just_pressed = input_action_state.just_pressed(InputActions::Select);
            let just_released = input_action_state.just_released(InputActions::Select);

            if let CursorDragSelectType::Dragging(anchor) = cursor_drag_select.0 {
                let bounding_box = (map.pixel_position_to_position(anchor), cursor_position);
                let mut selections = Vec::new();
                for (entity, position) in selectable_query.iter() {
                    if in_bounding_box(&bounding_box, position) {
                        selections.push(entity);
                    }
                }
                if just_released {
                    cursor_drag_select.0 = CursorDragSelectType::None;
                    selection.select_units(selections)
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

                for (entity, position) in selectable_query.iter() {
                    if &cursor_position == position {
                        not_hovering = false;
                        if !cursor_selection_target.0.is_selected(entity) {
                            cursor_selection_target.0.select_unit(entity);
                            cursor_target_time.0.reset();
                            cursor_target_time.0.unpause();
                        }

                        if trying_to_select.is_some() && !selection.is_selected_alone(entity) {
                            selection.select_unit(entity);
                            trying_to_select = Some(true);
                        }
                    }
                }
                match trying_to_select {
                    Some(have_selected) if !have_selected => {
                        selection.clear();
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

fn in_bounding_box(bounding_box: &(Position, Position), position: &Position) -> bool {
    let min_x = std::cmp::min(bounding_box.0.x, bounding_box.1.x);
    let max_x = std::cmp::max(bounding_box.0.x, bounding_box.1.x);
    let min_y = std::cmp::min(bounding_box.0.y, bounding_box.1.y);
    let max_y = std::cmp::max(bounding_box.0.y, bounding_box.1.y);

    position.x >= min_x && position.x <= max_x && position.y >= min_y && position.y <= max_y
}

// we go through all entities that can be hovered in current context (how do we do that actually?)
// if they are hoverable, we add that to hover

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

#[derive(Component, Debug, Clone, Default)]
pub struct Selectable {}

#[derive(Component, Debug, Clone, Default)]
pub enum SelectionInteractionState {
    #[default]
    None,
    Selecting,
    Selected,
}

#[derive(Component, Debug, Clone, Default)]
pub struct Selection(SelectionType);

#[derive(Debug, Clone, Default)]
pub enum SelectionType {
    #[default]
    None,
    Unit(Entity),
    Units(HashSet<Entity>),
}

impl Selection {
    pub fn is_empty(&self) -> bool {
        matches!(self.0, SelectionType::None)
    }

    pub fn is_selected(&self, entity: Entity) -> bool {
        match &self.0 {
            SelectionType::Unit(selected_entity) if *selected_entity == entity => true,
            SelectionType::Units(selected_entities) if selected_entities.contains(&entity) => true,
            _ => false,
        }
    }

    pub fn is_selected_alone(&self, entity: Entity) -> bool {
        matches!(self.0, SelectionType::Unit(selected_entity) if selected_entity == entity)
    }

    pub fn clear(&mut self) {
        self.0 = SelectionType::None;
    }

    pub fn select_unit(&mut self, entity: Entity) {
        self.0 = SelectionType::Unit(entity);
    }

    pub fn select_units(&mut self, mut entities: Vec<Entity>) {
        self.0 = match entities.len() {
            0 => SelectionType::None,
            1 => SelectionType::Unit(entities[0]),
            _ => SelectionType::Units(HashSet::from_iter(entities.drain(..))),
        };
    }

    // add unit to a valid unit selection, ignores otherwise
    pub fn add_unit_to_selection(&mut self, entity: Entity) {
        match &self.0 {
            SelectionType::Units(selected_entities) => {
                let mut new_selected_entities = selected_entities.clone();
                new_selected_entities.insert(entity);
                self.0 = SelectionType::Units(new_selected_entities);
            }
            SelectionType::Unit(selected_entity) if *selected_entity != entity => {
                let mut selected_entities = HashSet::new();
                selected_entities.insert(*selected_entity);
                selected_entities.insert(entity);
                self.0 = SelectionType::Units(selected_entities);
            }
            SelectionType::None => {
                self.0 = SelectionType::Unit(entity);
            }
            _ => {}
        }
    }

    // remove unit from a valid unit selection, ignores otherwise
    pub fn remove_unit_from_selection(&mut self, entity: Entity) {
        match &self.0 {
            SelectionType::Units(selected_entities) => {
                let mut new_selected_entities = selected_entities.clone();
                new_selected_entities.remove(&entity);
                if new_selected_entities.len() <= 1 {
                    self.0 = SelectionType::Unit(*selected_entities.iter().next().unwrap());
                } else {
                    self.0 = SelectionType::Units(new_selected_entities);
                }
            }
            SelectionType::Unit(selected_entity) if *selected_entity == entity => {
                self.0 = SelectionType::None;
            }
            _ => {}
        }
    }
}

#[derive(Component, Debug, Clone, Default)]
pub struct CursorTargetTime(pub Stopwatch);

#[derive(Component, Debug, Clone, Default)]
pub struct CursorSelectionTarget(pub Selection);

#[derive(Component, Debug, Clone, Default)]
pub struct CursorDragSelect(pub CursorDragSelectType);

#[derive(Debug, Clone, Default)]
pub enum CursorDragSelectType {
    #[default]
    None,
    Dragging(Vec2),
}

#[derive(Bundle, Debug, Clone, Default)]
pub struct CursorTargetBundle {
    pub target_time: CursorTargetTime,
    pub drag_select: CursorDragSelect,
    pub selection_target: CursorSelectionTarget,
    // pub interaction_target - what will happen if you right click
    // pub tooltip_target - what tooltip to show for this
}

#[derive(Bundle, Debug, Clone, Default)]
pub struct SelectableBundle {
    selectable: Selectable,
    interaction_state: SelectionInteractionState,
}

#[derive(Component, Debug, Clone, Default)]
pub struct Viewer {}

#[derive(Bundle, Debug, Clone, Default)]
pub struct ViewerBundle {
    pub viewer: Viewer,
    pub selection: Selection,
    #[bundle]
    pub cursor_target: CursorTargetBundle,
}
