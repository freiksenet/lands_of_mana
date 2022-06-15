use bevy::render::camera::CameraTypePlugin;
use bevy_kayak_ui::CameraUiKayak;

use crate::prelude::*;

mod app;
mod bindings;
mod camera;
mod topbar;

pub struct GuiPlugin {}

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_kayak_ui::BevyKayakUIPlugin)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                bevy::render::camera::camera_system::<camera::UIOrthographicProjection>,
            )
            .add_plugin(CameraTypePlugin::<CameraUiKayak>::default())
            .add_enter_system(
                config::EngineState::InGame,
                bindings::setup_binding_resources.exclusive_system(),
            )
            .add_enter_system(config::EngineState::InGame, app::setup_ui)
            .add_system_set(bindings::bindings_system_set());
    }
}
