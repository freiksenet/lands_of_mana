use bevy::prelude::*;
use iyes_loopless::prelude::*;
use kayak_core::{bind, Binding, MutableBound};
use kayak_ui::{
    bevy::{BevyContext, FontMapping, UICameraBundle},
    core::{render, Index},
    widgets::App,
};

use crate::{assets, game};

mod topbar;

pub fn setup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    font_assets: Res<assets::FontAssets>,
    game_time_query: Query<&game::GameTime>,
    game_state: Res<CurrentState<game::InGameState>>,
) {
    let game_time = game_time_query.single();
    commands.insert_resource(bind(*game_time));
    commands.insert_resource(bind(game_state.0));
    commands.spawn_bundle(UICameraBundle::new());
    font_mapping.set_default(font_assets.compass.clone());

    let context = BevyContext::new(|context| {
        render! {
            <App>
              <topbar::TopBar />
            </App>
        }
    });

    commands.insert_resource(context);
}

pub fn bind_game_time(
    game_time_binding: ResMut<Binding<game::GameTime>>,
    game_time_query: Query<&game::GameTime, Changed<game::GameTime>>,
) {
    let game_time_result = game_time_query.get_single();
    if let Ok(game_time) = game_time_result {
        game_time_binding.set(*game_time);
    }
}

pub fn bind_game_state(
    game_state_binding: ResMut<Binding<game::InGameState>>,
    game_state: Res<CurrentState<game::InGameState>>,
) {
    if game_state.is_changed() {
        game_state_binding.set(game_state.0);
    }
}

// commands
//     .spawn()
//     .insert(TopBar)
//     .insert_bundle(NodeBundle {
//         style: Style {
//             size: Size::new(Val::Px(2048.), Val::Px(32.)),
//             padding: Rect {
//                 left: Val::Px(12.),
//                 right: Val::Px(12.),
//                 ..default()
//             },
//             justify_content: JustifyContent::FlexStart,
//             align_items: AlignItems::Center,
//             align_self: AlignSelf::FlexEnd,
//             ..default()
//         },
//         image: UiImage(ui_assets.top_bar.clone()),
//         focus_policy: bevy::ui::FocusPolicy::Block,
//         ..default()
//     })
//     .with_children(|parent| {
//         // Title
//         parent.spawn_bundle(TextBundle {
//             text: Text::with_section(
//                 "mom4x Galore Top Bar",
//                 TextStyle {
//                     font: font_assets.compass.clone(),
//                     font_size: 16.,
//                     color: Color::BLACK,
//                 },
//                 Default::default(),
//             ),
//             ..default()
//         });

//         parent.spawn_bundle(NodeBundle {
//             color: Color::NONE.into(),
//             style: Style {
//                 flex_grow: 1.,
//                 ..default()
//             },
//             ..default()
//         });

//         parent
//             .spawn_bundle(NodeBundle {
//                 color: Color::NONE.into(),
//                 style: Style {
//                     size: Size::new(Val::Auto, Val::Px(32.)),
//                     align_self: AlignSelf::FlexEnd,
//                     justify_content: JustifyContent::SpaceAround,
//                     align_items: AlignItems::Center,
//                     ..default()
//                 },
//                 ..default()
//             })
//             .with_children(|parent| {
//                 parent
//                     .spawn()
//                     .insert(PauseButton)
//                     .insert_bundle(ButtonBundle {
//                         style: Style {
//                             size: Size::new(Val::Px(16.), Val::Px(16.)),
//                             justify_content: JustifyContent::Center,
//                             align_items: AlignItems::Center,
//                             margin: Rect {
//                                 bottom: Val::Px(1.),
//                                 ..default()
//                             },
//                             ..default()
//                         },
//                         image: UiImage(ui_assets.pause.clone()),
//                         ..default()
//                     });
//                 parent
//                     .spawn()
//                     .insert(ResumeButton)
//                     .insert_bundle(ButtonBundle {
//                         style: Style {
//                             size: Size::new(Val::Px(16.), Val::Px(16.)),
//                             justify_content: JustifyContent::Center,
//                             align_items: AlignItems::Center,
//                             margin: Rect {
//                                 left: Val::Px(6.),
//                                 bottom: Val::Px(1.),
//                                 ..default()
//                             },
//                             ..default()
//                         },
//                         image: UiImage(ui_assets.resume.clone()),
//                         ..default()
//                     });
//                 parent
//                     .spawn()
//                     .insert(GameTickDisplay)
//                     .insert_bundle(TextBundle {
//                         text: Text::with_section(
//                             std::format!("Day {:}, {:} / 10", game_time.day, game_time.tick),
//                             TextStyle {
//                                 font: font_assets.compass.clone(),
//                                 font_size: 16.,
//                                 color: Color::BLACK,
//                             },
//                             Default::default(),
//                         ),
//                         style: Style {
//                             margin: Rect {
//                                 left: Val::Px(6.),
//                                 ..default()
//                             },
//                             ..default()
//                         },
//                         ..default()
//                     });
//             });
//     });
