use bevy::{input::mouse::MouseWheel, prelude::*};
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{config, game};

mod camera;
pub struct InputPlugin {
    pub config: config::EngineConfig,
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bevy::input::system::exit_on_esc_system)
            .add_system(bevy::window::exit_on_window_close_system)
            .add_plugin(InputManagerPlugin::<InputActions>::default())
            .add_enter_system(self.config.run_game, setup)
            .add_system_set(
                ConditionSet::new()
                    .label("input")
                    .run_in_state(self.config.run_game)
                    .with_system(input_to_game_actions)
                    .with_system(interact)
                    .into(),
            )
            .add_plugin(camera::CameraPlugin {
                config: self.config,
            });
    }
}

fn setup(mut commands: Commands, world_query: Query<Entity, With<game::map::GameWorld>>) {
    let world_entity = world_query.single();
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
    input_map.insert(InputActions::Interact, MouseButton::Left);
    commands
        .entity(world_entity)
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
        world_action_state.press(game::actions::WorldActions::Pause.clone())
    }

    if input_action_state.just_pressed(InputActions::Resume)
        || (input_action_state.just_pressed(InputActions::TogglePause)
            && game_state.0 == game::InGameState::Paused)
    {
        world_action_state.press(game::actions::WorldActions::Resume.clone())
    }
}

fn interact(
    windows: Res<Windows>,
    camera_transform_query: Query<(&Camera, &Transform), With<Camera>>,
    world_query: Query<&game::map::GameWorld>,
    input_action_query: Query<&ActionState<InputActions>>,
    mut selectable_query: Query<(&game::map::Position, &mut Selectable)>,
) {
    let input_action_state = input_action_query.single();

    if (input_action_state.just_pressed(InputActions::Interact)) {
        let window = windows.get_primary().unwrap();

        if let Some(window_cursor_position) = window.cursor_position() {
            let (camera, camera_transform) = camera_transform_query.single();
            let window_size = Vec2::new(window.width() as f32, window.height() as f32);
            let ndc = (window_cursor_position / window_size) * 2.0 - Vec2::ONE;
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix.inverse();
            // use it to convert ndc to world-space coordinates
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
            let world = world_query.single();
            let cursor_position = world.pixel_position_to_position(world_pos);

            for (position, mut selectable) in selectable_query.iter_mut() {
                if (&cursor_position == position) {
                    selectable.is_selected = true;
                } else if (selectable.is_selected) {
                    selectable.is_selected = false;
                }
            }
        }
    }
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

    // Generic left click interact
    Interact,
}

#[derive(Component, Debug, Clone, Default)]
#[component(storage = "SparseSet")]
pub struct Selectable {
    pub is_selected: bool,
}
