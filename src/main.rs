use bevy::asset::AssetServerSettings;
use lands_of_mana::prelude::*;

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
        // .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
        .add_plugin(assets::AssetLoadingPlugin {})
        .add_plugin(game::GamePlugin {})
        .add_plugin(render::RenderPlugin {})
        .add_plugin(ui::InputPlugin {})
        .add_plugin(gui::GuiPlugin {})
        .run();
}
