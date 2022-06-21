use kayak_ui::{
    bevy::{BevyContext, FontMapping},
    core::{
        render,
        render_command::RenderCommand,
        rsx,
        styles::{Style, StyleProp, Units},
        widget, Binding, Bound, Children, Index, WidgetProps,
    },
    widgets::Element,
};

use crate::prelude::*;

pub fn setup_ui(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    font_assets: Res<assets::FontAssets>,
) {
    commands.spawn_bundle(gui::camera::UICameraBundle::new());
    font_mapping.set_default(font_assets.compass.clone());
    let context = BevyContext::new(|context| {
        render! {
            <App>
              <gui::topbar::TopBar />
            </App>
        }
    });

    commands.insert_resource(context);
}

#[derive(WidgetProps, Default, Debug, PartialEq, Clone)]
pub struct AppProps {
    #[prop_field(Styles)]
    pub styles: Option<Style>,
    #[prop_field(Children)]
    pub children: Option<Children>,
}

#[widget]
pub fn App(props: AppProps) {
    let window_size = if let Ok(world) = context.get_global::<bevy::prelude::World>() {
        if let Some(window_size) = world.get_resource::<Binding<gui::bindings::PixelWindow>>() {
            window_size.clone()
        } else {
            return;
        }
    } else {
        return;
    };

    context.bind(&window_size);
    let window_size = window_size.get();
    props.styles = Some(Style::default().with_style(Style {
        render_command: StyleProp::Value(RenderCommand::Layout),
        width: StyleProp::Value(Units::Pixels(window_size.0)),
        height: StyleProp::Value(Units::Pixels(window_size.1)),
        ..Default::default()
    }));

    rsx! {
        <Element>
            {children}
        </Element>
    }
}
