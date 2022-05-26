use bevy::{prelude::*};
use bevy_pixel_camera::{PixelBorderPlugin, PixelCameraBundle, PixelCameraPlugin, PixelProjection};
use iyes_loopless::prelude::*;

use crate::config;

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
                    .run_in_state(self.config.run_game)
                    .with_system(movement)
                    .into(),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(PixelCameraBundle::new(PixelProjection {
        centered: true,
        zoom: 5,
        ..Default::default()
    }));
}

// A simple camera system for moving and zooming the camera.
fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    world_query: Query<&crate::game::map::GameWorld>,
    mut query: Query<(&mut Transform, &mut PixelProjection), With<Camera>>,
) {
    let game_world = world_query.single();
    let border_size_pixels = 6 * 16;
    let pixels_width = (game_world.width * 16 + border_size_pixels * 2) as f32;
    let pixels_height = (game_world.height * 16 + border_size_pixels * 2) as f32;

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

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Z) {
            let zoom = std::cmp::max(camera.zoom - 1, 3);
            camera.zoom = zoom;
        }

        if keyboard_input.pressed(KeyCode::X) {
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
