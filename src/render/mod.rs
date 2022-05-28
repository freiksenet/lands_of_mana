use benimator::AnimationPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use bevy::prelude::*;

use bevy_ecs_tilemap::Tilemap2dPlugin;
use bevy_framepace::{FramepacePlugin, FramerateLimit};
use iyes_loopless::prelude::*;

use crate::config;

pub mod tilemap;
pub mod units;
pub struct RenderPlugin {
    pub config: config::EngineConfig,
}

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FramepacePlugin {
            framerate_limit: FramerateLimit::Manual(30),
            ..Default::default()
        })
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(Tilemap2dPlugin)
        .add_plugin(AnimationPlugin::default())
        .add_enter_system(self.config.load_graphics, tilemap::setup)
        .add_enter_system(self.config.load_graphics, units::setup)
        .add_system(proceed_to_ready_state.run_in_state(self.config.load_graphics))
        .add_system_set(
            ConditionSet::new()
                .label("render")
                .after("input")
                .run_in_state(self.config.run_game)
                .with_system(units::animations)
                .with_system(units::selected)
                .into(),
        );
    }
}

fn proceed_to_ready_state(mut commands: Commands, config: Res<config::EngineConfig>) {
    commands.insert_resource(NextState(config.after_load_graphics));
}
