use kayak_core::{
    styles::{Edge, LayoutType},
    Binding, Bound, Color,
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

use crate::prelude::*;

#[widget]
pub fn Resources() {
    let style = Style {
        width: StyleProp::Value(Units::Stretch(1.)),
        layout_type: StyleProp::Value(LayoutType::Row),
        background_color: StyleProp::Value(Color::new(0., 0., 0., 0.)),
        ..Style::default()
    };

    let resources_binding = context
        .query_world::<Res<Binding<ui::gui::bindings::PlayerResources>>, _, _>(
            move |player_resources| player_resources.clone(),
        );
    context.bind(&resources_binding);
    let resources = resources_binding.get();

    rsx! {
      <Element styles={Some(style)}>
        {VecTracker::from(resources.stockpile_resources.iter().map(|(resource_type, resource)| {
          constructor! {
              <StockpileResource resource_type={*resource_type} resource={*resource} />
          }
        }))}
        {VecTracker::from(resources.capacity_resources.iter().map(|(resource_type, resource)| {
          constructor! {
              <CapacityResource resource_type={*resource_type} resource={*resource} />
          }
        }))}
      </Element>
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq, Eq)]
pub struct StockpileResourceProps {
    #[prop_field]
    resource_type: game::world::StockpileResourceType,
    #[prop_field]
    resource: ui::gui::bindings::PlayerStockpileResource,
}

#[widget]
pub fn StockpileResource(props: StockpileResourceProps) {
    let style = Style {
        width: StyleProp::Value(Units::Stretch(1.)),

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
          content={format!("{:?}:\u{00A0}{:}", props.resource_type, props.resource.amount)}
          size={15.}
          line_height={Some(18.)}
          styles={Some(text_style)}
          />
      </Element>
    }
}

#[derive(WidgetProps, Clone, Debug, Default, PartialEq, Eq)]
pub struct CapacityResourceProps {
    #[prop_field]
    resource_type: game::world::CapacityResourceType,
    #[prop_field]
    resource: ui::gui::bindings::PlayerCapacityResource,
}

#[widget]
pub fn CapacityResource(props: CapacityResourceProps) {
    let style = Style {
        width: StyleProp::Value(Units::Stretch(1.)),

        ..Style::default()
    };

    let text_style = Style {
        color: StyleProp::Value(Color::new(0., 0., 0., 1.)),
        ..Style::default()
    };

    rsx! {
      <Element styles={Some(style)}>
        <Text
          content={format!("{:?}:\u{00A0}{:}/{:}", props.resource_type, props.resource.free, props.resource.total)}
          size={15.}
          line_height={Some(18.)}
          styles={Some(text_style)}
         />
      </Element>
    }
}
