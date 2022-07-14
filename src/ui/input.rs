use std::hash::Hash;

use leafwing_input_manager::prelude::*;

use crate::{prelude::*, ui::Viewer};

pub fn setup_input_map(mut commands: Commands, viewer_query: Query<Entity, With<Viewer>>) {
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

pub fn input_to_game_actions(
    game_state: Res<CurrentState<game::InGameState>>,
    input_action_query: Query<&ActionState<InputActions>>,
    mut world_action_query: Query<&mut ActionState<game::actions::WorldActions>>,
) {
    let input_action_state = input_action_query.single();
    let mut world_action_state = world_action_query.single_mut();

    if input_action_state.just_released(InputActions::Pause)
        || (input_action_state.just_released(InputActions::TogglePause)
            && game_state.0 == game::InGameState::Running)
    {
        world_action_state.press(game::actions::WorldActions::Pause)
    }

    if input_action_state.just_released(InputActions::Resume)
        || (input_action_state.just_released(InputActions::TogglePause)
            && game_state.0 == game::InGameState::Paused)
    {
        world_action_state.press(game::actions::WorldActions::Resume)
    }
}
