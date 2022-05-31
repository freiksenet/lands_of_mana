use bevy::prelude::*;
use bevy_pixel_camera::{PixelBorderPlugin, PixelCameraBundle, PixelCameraPlugin, PixelProjection};
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{config, game, ui};

pub struct CameraPlugin {
    pub config: config::EngineConfig,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PixelCameraPlugin)
            .add_plugin(PixelBorderPlugin {
                color: Color::rgb(0.1, 0.1, 0.1),
            })
            .add_enter_system(self.config.run_game, setup)
            .add_system_set(
                ConditionSet::new()
                    .label("input")
                    .run_in_state(self.config.run_game)
                    .with_system(camera_control)
                    .into(),
            );
    }
}

pub fn camera_position_to_pixel_position(
    window: &Window,
    camera: &Camera,
    camera_transform: &Transform,
) -> Option<Vec2> {
    if let Some(window_cursor_position) = window.cursor_position() {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let ndc = (window_cursor_position / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
        // use it to convert ndc to world-space coordinates
        Some(ndc_to_world.project_point3(ndc.extend(-1.0)).truncate())
    } else {
        None
    }
}

fn setup(mut commands: Commands, world_query: Query<Entity, With<game::map::GameWorld>>) {
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
    world_query: Query<&game::map::GameWorld>,
    mut query: Query<(&mut Transform, &mut PixelProjection), With<Camera>>,
) {
    let game_world = world_query.single();
    let border_size_pixels = 6 * 16;
    let pixels_width = (game_world.width * 16 + border_size_pixels * 2) as f32;
    let pixels_height = (game_world.height * 16 + border_size_pixels * 2) as f32;

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
