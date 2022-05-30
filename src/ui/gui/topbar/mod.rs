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

use crate::{
    assets,
    game::{self},
};

mod toolbar;

#[widget]
pub fn TopBar() {
    let (top_bar_image_handle, pause_button, resume_button) = {
        let mut world = context.get_global_mut::<World>().unwrap();
        let cell = world.cell();
        let ui_assets = cell.get_resource::<assets::UiAssets>().unwrap();
        let mut image_manager = cell.get_resource_mut::<ImageManager>().unwrap();
        let top_bar_image_handle = image_manager.get(&ui_assets.window_light_top.clone());

        (
            top_bar_image_handle,
            get_button(&mut image_manager, &ui_assets.button_pause),
            get_button(&mut image_manager, &ui_assets.button_resume),
        )
    };

    let game_time_binding = context
        .query_world::<Res<Binding<game::GameTime>>, _, _>(move |game_time| game_time.clone());
    context.bind(&game_time_binding);
    let game_time = game_time_binding.get();

    let game_state_binding = context
        .query_world::<Res<Binding<game::InGameState>>, _, _>(move |game_state| game_state.clone());
    context.bind(&game_state_binding);
    let game_state = game_state_binding.get();

    let container_style = Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        width: StyleProp::Value(Units::Percentage(100.)),
        height: StyleProp::Value(Units::Pixels(36.)),
        left: StyleProp::Value(Units::Pixels(0.)),
        top: StyleProp::Value(Units::Pixels(0.)),
        padding: StyleProp::Value(Edge::new(
            Units::Pixels(2.),
            Units::Pixels(8.),
            Units::Pixels(2.),
            Units::Pixels(8.),
        )),
        ..Style::default()
    };

    let title_style = Style {
        color: StyleProp::Value(Color::new(0., 0., 0., 1.)),
        width: StyleProp::Value(Units::Stretch(1.)),
        ..Style::default()
    };

    let tick_counter_container_style = Style {
        width: StyleProp::Value(Units::Stretch(1.)),
        padding_left: StyleProp::Value(Units::Stretch(1.)),
        ..Style::default()
    };

    let tick_counter_style = Style {
        color: StyleProp::Value(Color::new(0., 0., 0., 1.)),
        ..Style::default()
    };

    rsx! {
      <>
        <NinePatch styles={Some(container_style)}
          handle={top_bar_image_handle}
          border={Edge::all(16.0)}>
          <Text size={24.0}
            line_height={Some(32.)}
            content={"Mom4X TopBar".to_string()}
            styles={Some(title_style)} />
          <Element styles={Some(tick_counter_container_style)}>
            <Text size={24.0}
              line_height={Some(32.)}
              content={format!("Day {:04} | Tick {:02}", game_time.day + 1, game_time.tick + 1)}
              styles={Some(tick_counter_style)} />
          </Element>
          <toolbar::Toolbar
            game_state={game_state}
            pause_button={pause_button}
            resume_button={resume_button} />
        </NinePatch>
      </>
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