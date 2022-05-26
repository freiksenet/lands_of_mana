use benimator::AnimationPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::exit_on_window_close_system;
use bevy_ecs_tilemap::Tilemap2dPlugin;
use bevy_framepace::{FramepacePlugin, FramerateLimit};
use iyes_loopless::prelude::*;

mod assets;
mod camera;
mod config;
mod game;
mod render;

fn main() {
    let config = config::EngineConfig {
        load_assets: config::EngineState::LoadingAssets,
        after_load_assets: config::EngineState::LoadingWorld,
        load_world: config::EngineState::LoadingWorld,
        after_load_world: config::EngineState::LoadingGraphics,
        load_graphics: config::EngineState::LoadingGraphics,
        after_load_graphics: config::EngineState::InGame,
        run_game: config::EngineState::InGame,
    };

    let window = WindowDescriptor {
        // mode: bevy::window::WindowMode::BorderlessFullscreen,
        title: String::from("mom4x"),
        ..Default::default()
    };

    let mut app = App::new();

    app.insert_resource(window)
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(config)
        .add_loopless_state(config::EngineState::LoadingAssets);

    app.add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system(exit_on_window_close_system)
        .add_plugin(assets::AssetLoadingPlugin { config })
        .add_plugin(game::GamePlugin { config })
        .add_plugin(render::RenderPlugin { config })
        .add_plugin(camera::CameraPlugin { config })
        .run();
}
