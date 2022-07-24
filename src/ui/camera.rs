use bevy_pixel_camera::{PixelBorderPlugin, PixelCameraBundle, PixelCameraPlugin, PixelProjection};
use leafwing_input_manager::prelude::*;

use crate::{
    config::{EngineState, Stage, UiSyncLabel, UpdateStageLabel},
    prelude::*,
};

pub struct CameraPlugin {}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PixelCameraPlugin)
            .add_plugin(PixelBorderPlugin {
                color: Color::rgb(0.1, 0.1, 0.1),
            })
            .add_enter_system(config::EngineState::InGame, setup)
            .add_system_set(
                ConditionSet::new()
                    .label_and_after(UpdateStageLabel::Input)
                    .run_in_state(EngineState::InGame)
                    .with_system(camera_control)
                    .into(),
            );
    }
}

/// Given a position in world space, use the camera to compute the viewport-space coordinates.
///
/// To get the coordinates in Normalized Device Coordinates, you should use
/// [`world_to_ndc`](Self::world_to_ndc).
pub fn world_to_viewport(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    world_position: Vec2,
) -> Option<Vec2> {
    let target_size = Vec2::new(window.width() as f32, window.height() as f32);
    let ndc_space_coords = world_to_ndc(camera, camera_transform, world_position)?;
    // NDC z-values outside of 0 < z < 1 are outside the camera frustum and are thus not in viewport-space
    if ndc_space_coords.z < 0.0 || ndc_space_coords.z > 1.0 {
        return None;
    }

    // Once in NDC space, we can discard the z element and rescale x/y to fit the screen
    Some((ndc_space_coords.truncate() + Vec2::ONE) / 2.0 * target_size)
}

/// Given a position in world space, use the camera's viewport to compute the Normalized Device Coordinates.
///
/// Values returned will be between -1.0 and 1.0 when the position is within the viewport.
/// To get the coordinates in the render target's viewport dimensions, you should use
/// [`world_to_viewport`](Self::world_to_viewport).
pub fn world_to_ndc(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    world_position: Vec2,
) -> Option<Vec3> {
    // Build a transform to convert from world to NDC using camera data
    let world_to_ndc: Mat4 = camera.projection_matrix * camera_transform.compute_matrix().inverse();
    let ndc_space_coords: Vec3 = world_to_ndc.project_point3(world_position.extend(0.));

    if !ndc_space_coords.is_nan() {
        Some(ndc_space_coords)
    } else {
        None
    }
}

pub fn cursor_to_world(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    if let Some(window_cursor_position) = window.cursor_position() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let ndc = (window_cursor_position / window_size) * 2.0 - Vec2::ONE;
        Some(ndc_to_world(camera, camera_transform, ndc))
    } else {
        None
    }
}

pub fn ndc_to_world(camera: &Camera, camera_transform: &GlobalTransform, ndc: Vec2) -> Vec2 {
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
    ndc_to_world.project_point3(ndc.extend(-1.0)).truncate()
}

fn setup(mut commands: Commands, world_query: Query<Entity, With<game::GameWorld>>) {
    commands
        .entity(world_query.single())
        .with_children(|builder| {
            builder.spawn_bundle(PixelCameraBundle::new(PixelProjection {
                centered: true,
                zoom: 5,
                ..Default::default()
            }));
        });
}

// A simple camera system for moving and zooming the camera.
fn camera_control(
    input_action_query: Query<&ActionState<ui::InputActions>>,
    map_query: Query<&game::map::Map>,
    mut query: Query<(&mut Transform, &mut PixelProjection), With<Camera>>,
) {
    let map = map_query.single();
    let border_size_pixels = 6 * 16;
    let pixels_width = (map.width * 16 + border_size_pixels * 2) as f32;
    let pixels_height = (map.height * 16 + border_size_pixels * 2) as f32;

    let input_action_state = input_action_query.single();

    for (mut transform, mut camera) in query.iter_mut() {
        let min = Vec3::new(
            0. - pixels_width / 2. - camera.left,
            0. - pixels_height / 2. - camera.bottom,
            0.,
        );
        let max = Vec3::new(
            pixels_width / 2. - camera.right,
            pixels_height / 2. - camera.top,
            0.,
        );
        let mut direction = Vec3::ZERO;

        if input_action_state.pressed(ui::InputActions::CameraMoveWest) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if input_action_state.pressed(ui::InputActions::CameraMoveEast) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if input_action_state.pressed(ui::InputActions::CameraMoveNorth) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if input_action_state.pressed(ui::InputActions::CameraMoveSouth) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if input_action_state.pressed(ui::InputActions::CameraZoomIn) {
            let zoom = std::cmp::max(camera.zoom - 1, 3);
            camera.zoom = zoom;
        }

        if input_action_state.pressed(ui::InputActions::CameraZoomOut) {
            let zoom = std::cmp::min(camera.zoom + 1, 10);
            camera.zoom = zoom;
        }

        let z = transform.translation.z;
        transform.translation = (transform.translation + direction * 5.).clamp(min, max);
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}
