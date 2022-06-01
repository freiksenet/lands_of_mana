use kayak_ui::{
    bevy::{BevyContext, FontMapping, UICameraBundle},
    core::{render, Index},
};

use crate::prelude::*;

pub fn setup_ui(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    font_assets: Res<assets::FontAssets>,
) {
    commands.spawn_bundle(UICameraBundle::new());
    font_mapping.set_default(font_assets.compass.clone());

    let context = BevyContext::new(|context| {
        render! {
            <kayak_ui::widgets::App>
              <ui::gui::topbar::TopBar />
            </kayak_ui::widgets::App>
        }
    });

    commands.insert_resource(context);
}
