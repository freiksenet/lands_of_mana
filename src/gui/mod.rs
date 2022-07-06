use bevy_egui::{EguiContext, EguiPlugin, EguiSettings};

use crate::prelude::*;

pub mod gui_context;
pub mod plugins;
pub mod widgets;

pub use gui_context::*;
pub struct GuiPlugin {}

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_enter_system(config::EngineState::LoadingGraphics, setup_egui)
            .add_plugin(plugins::DebugTooltipPlugin {})
            .add_plugin(plugins::TitleBarPlugin {})
            .add_plugin(plugins::TimeBarPlugin {})
            .add_plugin(plugins::ResourceBarPlugin {})
            .add_plugin(plugins::SelectedWindowPlugin {});
    }
}

pub fn setup_egui(
    mut commands: Commands,
    windows: Res<Windows>,
    mut egui_context: ResMut<EguiContext>,
    asset_server: Res<AssetServer>,
    ui_assets: Res<assets::UiAssets>,
    icon_assets: Res<assets::IconAssets>,
    mut egui_settings: ResMut<EguiSettings>,
) {
    let mut gui_context = GuiContext::default();
    gui_context.setup(
        &mut egui_context,
        &windows,
        &mut egui_settings,
        &asset_server,
        &ui_assets,
        &icon_assets,
    );
    commands.insert_resource(gui_context);
}
