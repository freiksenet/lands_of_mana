use kayak_core::{
    styles::{Edge, LayoutType},
    Color, EventType, OnEvent,
};
use kayak_render_macros::{use_state, WidgetProps};
use kayak_ui::{
    core::{
        rsx,
        styles::{Style, StyleProp, Units},
        widget,
    },
    widgets::{Element, Image},
};

use crate::game;

#[derive(WidgetProps, Clone, Debug, Default, PartialEq, Eq)]
pub struct ToolbarProps {
    #[prop_field]
    pub pause_button: (u16, u16, u16, u16),
    #[prop_field]
    pub resume_button: (u16, u16, u16, u16),

    #[prop_field]
    pub game_state: game::InGameState,
}

#[widget]
pub fn Toolbar(props: ToolbarProps) {
    let style = Style {
        offset: StyleProp::Value(Edge {
            left: Units::Pixels(8.),
            ..Default::default()
        }),
        width: StyleProp::Value(Units::Pixels(72.)),
        layout_type: StyleProp::Value(LayoutType::Row),
        background_color: StyleProp::Value(Color::new(0., 0., 0., 0.)),
        ..Style::default()
    };

    rsx! {
      <Element
        styles={Some(style)}>
        <ToolbarButton image_handles={props.pause_button} active={props.game_state==game::InGameState::Paused} />
        <ToolbarButton image_handles={props.resume_button} active={props.game_state==game::InGameState::Running} />
      </Element>
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq)]
struct ToolbarButtonProps {
    #[prop_field(Styles)]
    pub styles: Option<Style>,
    pub image_handles: (u16, u16, u16, u16),

    pub active: bool,
}

#[widget]
fn ToolbarButton(props: ToolbarButtonProps) {
    let is_active = props.active;
    let (default, _pressed, hovered, active) = props.image_handles;
    let (is_hovered, set_hovered, ..) = use_state!(false);

    let on_event = OnEvent::new(move |_, event| match event.event_type {
        EventType::MouseIn(..) => set_hovered(true),
        EventType::MouseOut(..) => {
            set_hovered(false);
        }
        _ => (),
    });

    let handle = match (is_active, is_hovered) {
        (true, _) => active,
        (false, true) => hovered,
        (false, false) => default,
    };

    rsx! {
    <Image handle={handle} styles={Some(Style {
            width: StyleProp::Value(Units::Pixels(32.)),
            height: StyleProp::Value(Units::Pixels(32.)),
            ..Style::default()
          })}
          on_event={Some(on_event)}
          />
        }
}
