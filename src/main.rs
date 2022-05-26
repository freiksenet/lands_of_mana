use benimator::AnimationPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::exit_on_window_close_system;
use bevy_ecs_tilemap::Tilemap2dPlugin;
use bevy_framepace::{FramepacePlugin, FramerateLimit};
use iyes_loopless::prelude::*;

mod assets;
mod camera;
mod game;
mod render;
mod state;

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        // mode: bevy::window::WindowMode::BorderlessFullscreen,
        title: String::from("mom4x"),
        ..Default::default()
    })
    .insert_resource(Msaa { samples: 4 })
    .add_loopless_state(state::GameState::LoadingAssets);

    app.add_plugins(DefaultPlugins)
        .add_plugin(FramepacePlugin {
            framerate_limit: FramerateLimit::Manual(30),
            ..Default::default()
        })
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system(exit_on_window_close_system)
        .add_plugin(assets::AssetLoadingPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(Tilemap2dPlugin)
        .add_plugin(AnimationPlugin::default())
        .add_enter_system(state::GameState::LoadingWorld, game::setup)
        .add_enter_system(state::GameState::LoadingGraphics, render::tilemap::setup)
        .add_exit_system(state::GameState::LoadingAssets, render::units::setup)
        .run();
}
