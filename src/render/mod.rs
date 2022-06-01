use benimator::AnimationPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_ecs_tilemap::Tilemap2dPlugin;
use bevy_framepace::{FramepacePlugin, FramerateLimit};
use iyes_loopless::prelude::*;

use crate::prelude::*;

pub mod tilemap;
pub mod units;

pub struct RenderPlugin {}

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FramepacePlugin {
            framerate_limit: FramerateLimit::Manual(30),
            ..Default::default()
        })
        .add_stage(config::Stage::UiSync, SystemStage::parallel())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(Tilemap2dPlugin)
        .add_plugin(AnimationPlugin::default())
        .add_enter_system(config::EngineState::LoadingGraphics, tilemap::setup)
        .add_enter_system(config::EngineState::LoadingGraphics, units::setup)
        .add_system(proceed_to_ready_state.run_in_state(config::EngineState::LoadingGraphics))
        .add_system_set_to_stage(
            config::Stage::UiSync,
            ConditionSet::new()
                .label_and_after(config::UiSyncLabel::Bindings)
                .run_in_state(config::EngineState::InGame)
                .with_system(units::animations)
                .with_system(units::selected)
                .into(),
        );
    }
}

fn proceed_to_ready_state(mut commands: Commands) {
    commands.insert_resource(NextState(config::EngineState::LoadingGraphics.next()));
}
