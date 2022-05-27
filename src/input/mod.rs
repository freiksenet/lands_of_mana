use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;
use leafwing_input_manager::prelude::*;

use crate::{config, game};

pub struct InputPlugin {
    pub config: config::EngineConfig,
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(bevy::input::system::exit_on_esc_system)
            .add_system(bevy::window::exit_on_window_close_system)
            .add_plugin(InputManagerPlugin::<game::Actions>::default())
            .add_enter_system(self.config.run_game, setup);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(InputManagerBundle::<game::Actions> {
            // Stores "which actions are currently pressed"
            action_state: ActionState::default(),
            // Describes how to convert from player inputs into those actions
            input_map: InputMap::new([(game::Actions::TogglePause, KeyCode::Space)]),
        });
}
