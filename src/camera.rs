use bevy::prelude::*;
use bevy_pixel_camera::{PixelBorderPlugin, PixelCameraBundle, PixelCameraPlugin, PixelProjection};
use iyes_loopless::prelude::*;

use super::state;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PixelCameraPlugin)
            .add_plugin(PixelBorderPlugin {
                color: Color::rgb(0.1, 0.1, 0.1),
            })
            .add_enter_system(state::GameState::InGame, setup)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(state::GameState::InGame)
                    .with_system(movement)
                    .into(),
            );
    }
}

fn setup(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let ratio: f32 = window.physical_width() as f32 / window.physical_height() as f32;
    println!("{:?}", ratio);
    commands.spawn_bundle(PixelCameraBundle::new(PixelProjection {
        centered: true,
        desired_height: Some(160),
        desired_width: Some((160. * ratio).round() as i32),
        ..Default::default()
    }));
}

// A simple camera system for moving and zooming the camera.
fn movement(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Camera>>) {
    for mut transform in query.iter_mut() {
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

        // if keyboard_input.pressed(KeyCode::Z) {
        //     ortho.scale += 0.02;
        // }

        // if keyboard_input.pressed(KeyCode::X) {
        //     ortho.scale -= 0.02;
        // }

        // if ortho.scale < 0.1 {
        //     ortho.scale = 0.1;
        // }

        let z = transform.translation.z;
        transform.translation += direction * 5.;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}
