use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_ecs_tilemap::Tilemap2dPlugin;
use bevy_framepace::{FramepacePlugin, FramerateLimit};
use iyes_loopless::prelude::*;

use crate::prelude::*;

pub mod animations;
pub mod selection;
pub mod tilemap;
pub mod units;
pub mod z_level;

pub struct RenderPlugin {}

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FramepacePlugin {
            framerate_limit: FramerateLimit::Auto,
            ..Default::default()
        })
        .add_stage(config::Stage::UiSync, SystemStage::parallel())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(Tilemap2dPlugin)
        .add_plugin(selection::RenderSelectionPlugin {})
        .add_plugin(units::RenderUnitsPlugin {})
        .add_plugin(animations::AnimationsRenderPlugin {})
        .add_enter_system(config::EngineState::LoadingGraphics, tilemap::setup)
        .add_system(proceed_to_ready_state.run_in_state(config::EngineState::LoadingGraphics));
    }
}

fn proceed_to_ready_state(mut commands: Commands) {
    commands.insert_resource(NextState(config::EngineState::LoadingGraphics.next()));
}
