#![allow(clippy::forget_non_drop)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::type_complexity)]
#![feature(allocator_api)]
#![feature(trait_alias)]
#![feature(map_try_insert)]

pub mod assets;
pub mod config;
pub mod game;
pub mod gui;
pub mod prelude;
pub mod render;
pub mod ui;

use bevy::asset::AssetServerSettings;

use crate::prelude::*;

fn main() {
    let window = WindowDescriptor {
        mode: bevy::window::WindowMode::BorderlessFullscreen,
        title: String::from("mom4x"),
        // width: 1600.,
        // height: 1000.,
        ..Default::default()
    };

    let mut app = App::new();

    app.insert_resource(window)
        .insert_resource(AssetServerSettings {
            asset_folder: "assets/export".to_string(),
            watch_for_changes: true,
        })
        .insert_resource(Msaa { samples: 1 })
        .add_loopless_state(config::EngineState::LoadingAssets);

    app.add_plugins(DefaultPlugins)
        .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
        .add_plugin(assets::AssetLoadingPlugin {})
        .add_plugin(game::GamePlugin {})
        .add_plugin(render::RenderPlugin {})
        .add_plugin(ui::InputPlugin {})
        .add_plugin(gui::GuiPlugin {})
        .run();
}
