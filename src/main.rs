#![allow(clippy::forget_non_drop)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::type_complexity)]
#![feature(allocator_api)]

use bevy::prelude::*;
use iyes_loopless::prelude::*;

mod assets;
mod config;
mod game;
mod render;
mod ui;

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
        width: 1600.,
        height: 1000.,
        ..Default::default()
    };

    let mut app = App::new();

    app.insert_resource(window)
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(config)
        .add_loopless_state(config::EngineState::LoadingAssets);

    app.add_plugins(DefaultPlugins)
        .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
        .add_plugin(assets::AssetLoadingPlugin { config })
        .add_plugin(game::GamePlugin { config })
        .add_plugin(render::RenderPlugin { config })
        .add_plugin(ui::InputPlugin { config })
        .run();
}
