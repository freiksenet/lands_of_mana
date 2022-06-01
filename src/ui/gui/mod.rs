use crate::prelude::*;

mod app;
mod bindings;
mod topbar;

pub struct GuiPlugin {}

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_kayak_ui::BevyKayakUIPlugin)
            .add_enter_system(
                config::EngineState::InGame,
                bindings::setup_binding_resources.exclusive_system(),
            )
            .add_enter_system(config::EngineState::InGame, app::setup_ui)
            .add_system_set(bindings::bindings_system_set());
    }
}
