use bevy_egui::{EguiContext, EguiPlugin, EguiSettings};

use crate::{config::UiSyncLabel, prelude::*};

pub mod gui_context;
pub mod systems;
pub mod widgets;

pub use gui_context::*;
use systems::*;
pub struct GuiPlugin {}

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_enter_system(config::EngineState::LoadingGraphics, egui_setup)
            .add_system_set_to_stage(
                config::Stage::UiSync,
                ConditionSet::new()
                    .run_in_state(config::EngineState::InGame)
                    .label_and_after(UiSyncLabel::Sync)
                    .with_system(bind_current_player_resources)
                    .into(),
            )
            .add_system_set_to_stage(
                config::Stage::UiSync,
                ConditionSet::new()
                    .run_in_state(config::EngineState::InGame)
                    .label_and_after(UiSyncLabel::Update)
                    .with_system(title_bar)
                    .with_system(resource_bar)
                    .with_system(time_bar)
                    .into(),
            );
    }
}

pub fn egui_setup(
    mut commands: Commands,
    windows: Res<Windows>,
    mut egui_context: ResMut<EguiContext>,
    asset_server: Res<AssetServer>,
    ui_assets: Res<assets::UiAssets>,
    icon_assets: Res<assets::IconAssets>,
    mut egui_settings: ResMut<EguiSettings>,
) {
    let window = windows.get_primary().unwrap();
    let window_width = window.physical_width();
    let desired_width = 1280;
    let scale = window_width as f64 / desired_width as f64;
    commands.init_resource::<PlayerResources>();
    egui_settings.scale_factor = scale;
    let mut gui_context = GuiContext::default();
    gui_context.setup(&mut egui_context, &asset_server, &ui_assets, &icon_assets);
    commands.insert_resource(gui_context);
}
