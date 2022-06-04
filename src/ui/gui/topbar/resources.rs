use kayak_ui::{
    bevy::ImageManager,
    core::{
        constructor, rsx,
        styles::{Edge, LayoutType, Style, StyleProp, Units},
        widget, Binding, Bound, Color, VecTracker, WidgetProps,
    },
    widgets::{Element, Image, Text},
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
    let icon = {
        let mut world = context.get_global_mut::<World>().unwrap();
        let cell = world.cell();
        let icon_assets = cell.get_resource::<assets::IconAssets>().unwrap();
        let mut image_manager = cell.get_resource_mut::<ImageManager>().unwrap();
        image_manager.get(&match (props.resource_type) {
            game::world::StockpileResourceType::Gold => icon_assets.res_gold.clone(),
            game::world::StockpileResourceType::Wood => icon_assets.res_wood.clone(),
        })
    };

    let style = Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        padding: StyleProp::Value(Edge::all(Units::Stretch(1.))),
        width: StyleProp::Value(Units::Pixels(70.)),

        ..Style::default()
    };

    let text_style = Style {
        color: StyleProp::Value(Color::new(0., 0., 0., 1.)),
        ..Style::default()
    };

    let icon_style = Style {
        width: StyleProp::Value(Units::Pixels(14.)),
        height: StyleProp::Value(Units::Pixels(14.)),
        ..Style::default()
    };

    let income_text = if (props.resource.income >= 0.) {
        format!("+{:}", props.resource.income)
    } else {
        format!("-{:}", props.resource.income.abs())
    };

    rsx! {
      <Element styles={Some(style)}>
        <Image handle={icon} styles={Some(icon_style)} />
        <Text
          content={format!("{:}{:}", props.resource.amount, income_text)}
          size={15.}
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
    let icon = {
        let mut world = context.get_global_mut::<World>().unwrap();
        let cell = world.cell();
        let icon_assets = cell.get_resource::<assets::IconAssets>().unwrap();
        let mut image_manager = cell.get_resource_mut::<ImageManager>().unwrap();
        image_manager.get(&match (props.resource_type) {
            game::world::CapacityResourceType::Sun => icon_assets.mana_sun.clone(),
            game::world::CapacityResourceType::Arcana => icon_assets.mana_arcana.clone(),
            game::world::CapacityResourceType::Death => icon_assets.mana_death.clone(),
            game::world::CapacityResourceType::Chaos => icon_assets.mana_chaos.clone(),
            game::world::CapacityResourceType::Nature => icon_assets.mana_nature.clone(),
        })
    };

    let style = Style {
        padding: StyleProp::Value(Edge::all(Units::Stretch(1.))),
        layout_type: StyleProp::Value(LayoutType::Row),
        width: StyleProp::Value(Units::Stretch(1.)),

        ..Style::default()
    };

    let icon_style = Style {
        width: StyleProp::Value(Units::Pixels(14.)),
        height: StyleProp::Value(Units::Pixels(14.)),
        ..Style::default()
    };

    let text_style = Style {
        color: StyleProp::Value(Color::new(0., 0., 0., 1.)),
        ..Style::default()
    };

    rsx! {
      <Element styles={Some(style)}>
        <Image handle={icon} styles={Some(icon_style)} />
        <Text
          content={format!("{:}/{:}", props.resource.free, props.resource.total)}
          size={15.}
          styles={Some(text_style)}
         />
      </Element>
    }
}
