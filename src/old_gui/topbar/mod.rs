use bevy::prelude::*;
use bevy_kayak_ui::ImageManager;
use kayak_core::{
    styles::{Edge, LayoutType},
    Binding, Bound, Color,
};
use kayak_ui::{
    core::{
        rsx,
        styles::{Style, StyleProp, Units},
        widget,
    },
    widgets::{Element, NinePatch, Text},
};

use crate::prelude::*;

mod resources;
mod toolbar;

#[widget]
pub fn TopBar() {
    let (top_bar_image_handle, pause_button, resume_button) = {
        let mut world = context.get_global_mut::<World>().unwrap();
        let cell = world.cell();
        let ui_assets = cell.get_resource::<assets::UiAssets>().unwrap();
        let mut image_manager = cell.get_resource_mut::<ImageManager>().unwrap();
        let top_bar_image_handle = image_manager.get(&ui_assets.window_light.clone());

        (
            top_bar_image_handle,
            get_button(&mut image_manager, &ui_assets.button_pause),
            get_button(&mut image_manager, &ui_assets.button_resume),
        )
    };

    let game_tick_binding = context
        .query_world::<Res<Binding<game::GameTick>>, _, _>(move |game_tick| game_tick.clone());
    context.bind(&game_tick_binding);
    let game::GameTick(game_tick) = game_tick_binding.get();
    let game_day_binding =
        context.query_world::<Res<Binding<game::GameDay>>, _, _>(move |game_day| game_day.clone());
    context.bind(&game_day_binding);
    let game::GameDay(game_day) = game_day_binding.get();

    let game_state_binding = context
        .query_world::<Res<Binding<game::InGameState>>, _, _>(move |game_state| game_state.clone());
    context.bind(&game_state_binding);
    let game_state = game_state_binding.get();

    let title_n_container_style = Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        width: StyleProp::Value(Units::Pixels(100.)),
        height: StyleProp::Value(Units::Pixels(40.)),
        left: StyleProp::Value(Units::Pixels(2.)),
        top: StyleProp::Value(Units::Pixels(2.)),
        padding: StyleProp::Value(Edge::all(Units::Pixels(4.))),
        ..Style::default()
    };

    let resources_container_style = Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        width: StyleProp::Value(Units::Percentage(60.)),
        height: StyleProp::Value(Units::Pixels(18.)),
        top: StyleProp::Value(Units::Pixels(2.)),
        padding: StyleProp::Value(Edge::axis(Units::Pixels(0.), Units::Pixels(8.))),
        ..Style::default()
    };

    let toolbar_container_style = Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        width: StyleProp::Value(Units::Pixels(100.)),
        height: StyleProp::Value(Units::Pixels(40.)),
        left: StyleProp::Value(Units::Stretch(1.)),
        right: StyleProp::Value(Units::Pixels(2.)),
        top: StyleProp::Value(Units::Pixels(2.)),
        padding: StyleProp::Value(Edge::all(Units::Pixels(4.))),
        ..Style::default()
    };

    let title_container_style = Style {
        width: StyleProp::Value(Units::Auto),
        ..Style::default()
    };

    let title_style = Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        color: StyleProp::Value(Color::new(0., 0., 0., 1.)),

        ..Style::default()
    };

    let tick_counter_container_style = Style { ..Style::default() };

    let tick_counter_style = Style {
        color: StyleProp::Value(Color::new(0., 0., 0., 1.)),
        ..Style::default()
    };

    let container_style = Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        width: StyleProp::Value(Units::Percentage(100.)),
        height: StyleProp::Value(Units::Percentage(100.)),
        col_between: StyleProp::Value(Units::Stretch(1.)),
        ..Default::default()
    };

    rsx! {
      <Element styles={Some(container_style)}>
        <NinePatch styles={Some(title_n_container_style)}
          handle={top_bar_image_handle}
          border={Edge::all(8.0)}>
          <Element styles={Some(title_container_style)}>
            <Text size={15.0}
                content={"Mom4X\u{00A0}TopBar".to_string()}
                styles={Some(title_style)} />
          </Element>
        </NinePatch>
        <NinePatch styles={Some(resources_container_style)}
          handle={top_bar_image_handle}
          border={Edge::all(8.0)}>
          <resources::Resources />
        </NinePatch>
        <NinePatch styles={Some(toolbar_container_style)}
          handle={top_bar_image_handle}
          border={Edge::all(8.0)}>
          <Element styles={Some(tick_counter_container_style)}>
            <Text size={15.0}
              content={format!("Day\u{00A0}{:04}", game_day + 1)}
              styles={Some(tick_counter_style)} />
            <Text size={15.0}
              content={format!("Tick\u{00A0}{:02}", game_tick + 1)}
              styles={Some(tick_counter_style)} />
          </Element>
          <toolbar::Toolbar
            game_state={game_state}
            pause_button={pause_button}
            resume_button={resume_button} />
        </NinePatch>
      </Element>
    }
}

fn get_button(
    image_manager: &mut bevy::ecs::world::WorldBorrowMut<'_, bevy_kayak_ui::ImageManager>,
    handles: &[Handle<Image>],
) -> (u16, u16, u16, u16) {
    let button_handles: Vec<u16> = handles
        .iter()
        .map(|handle| image_manager.get(&handle.clone()))
        .collect();
    (
        button_handles[0],
        button_handles[1],
        button_handles[2],
        button_handles[3],
    )
}
