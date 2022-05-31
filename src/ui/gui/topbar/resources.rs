use bevy::prelude::{Res, *};
use kayak_core::{
    styles::{Edge, LayoutType},
    Binding, Bound, Color, WidgetProps,
};
use kayak_render_macros::WidgetProps;
use kayak_ui::{
    core::{
        constructor, rsx,
        styles::{Style, StyleProp, Units},
        widget, VecTracker,
    },
    widgets::{Element, Text},
};

use crate::ui::gui;

#[widget]
pub fn Resources() {
    let style = Style {
        offset: StyleProp::Value(Edge {
            left: Units::Pixels(8.),
            ..Default::default()
        }),
        width: StyleProp::Value(Units::Stretch(1.)),
        layout_type: StyleProp::Value(LayoutType::Row),
        background_color: StyleProp::Value(Color::new(0., 0., 0., 0.)),
        ..Style::default()
    };

    let resources_binding =
        context.query_world::<Res<Binding<gui::PlayerResources>>, _, _>(move |player_resources| {
            player_resources.clone()
        });
    context.bind(&resources_binding);
    let resources = resources_binding.get();
    println!("{:?}", resources);

    rsx! {
      <Element styles={Some(style)}>
        {VecTracker::from(resources.stockpile_resources.iter().map(|resource| {
          constructor! {
              <StockpileResource resource={*resource} />
          }
        }))}
        {VecTracker::from(resources.capacity_resources.iter().map(|resource| {
          constructor! {
              <CapacityResource resource={*resource} />
          }
        }))}
      </Element>
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq, Eq)]
pub struct StockpileResourceProps {
    #[prop_field]
    resource: gui::PlayerStockpileResource,
}

#[widget]
pub fn StockpileResource(props: StockpileResourceProps) {
    let style = Style {
        offset: StyleProp::Value(Edge::axis(Units::Auto, Units::Pixels(4.))),
        width: StyleProp::Value(Units::Auto),

        ..Style::default()
    };

    let text_style = Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        color: StyleProp::Value(Color::new(0., 0., 0., 1.)),
        ..Style::default()
    };

    rsx! {
      <Element styles={Some(style)}>
        <Text
          content={format!("{:?}:\u{00A0}{:}", props.resource.resource_type, props.resource.amount)}
          size={24.}
          line_height={Some(32.)}
          styles={Some(text_style)}
          />
      </Element>
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq, Eq)]
pub struct CapacityResourceProps {
    #[prop_field]
    resource: gui::PlayerCapacityResource,
}

#[widget]
pub fn CapacityResource(props: CapacityResourceProps) {
    let style = Style {
        offset: StyleProp::Value(Edge::axis(Units::Auto, Units::Pixels(4.))),
        width: StyleProp::Value(Units::Auto),

        ..Style::default()
    };

    let text_style = Style {
        color: StyleProp::Value(Color::new(0., 0., 0., 1.)),
        ..Style::default()
    };

    rsx! {
      <Element styles={Some(style)}>
        <Text
          content={format!("{:?}:\u{00A0}{:}/{:}", props.resource.resource_type, props.resource.free, props.resource.total)}
          size={24.}
          line_height={Some(32.)}
          styles={Some(text_style)}
         />
      </Element>
    }
}
