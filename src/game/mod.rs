use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::config;

pub mod map;
pub mod units;
pub mod world_gen;

pub struct GamePlugin {
    pub config: config::EngineConfig,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(self.config.load_world, world_gen::build_world);
    }
}
