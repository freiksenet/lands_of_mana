use bevy_egui::{egui, EguiContext};

use crate::{
    config::{EngineState, UiSyncLabel},
    game::{
        province::{City, CityType},
        units::{Unit, UnitType},
    },
    gui::{
        gui_context::{GuiContext, TextureType},
        widgets::*,
    },
    prelude::*,
    ui::{Selected, SelectedEntity, Viewer},
};

pub struct SelectedWindowPlugin {}

impl Plugin for SelectedWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            config::Stage::UiSync,
            ConditionSet::new()
                .run_in_state(EngineState::InGame)
                .label_and_after(UiSyncLabel::Update)
                .with_system(selected_window)
                .into(),
        );
    }
}

fn selected_window(
    mut egui_context: ResMut<EguiContext>,
    gui_context: Res<GuiContext>,
    selection_query: Query<&Selected, With<Viewer>>,
    unit_query: Query<&UnitType, With<Unit>>,
    city_query: Query<&CityType, With<City>>,
) {
    let Selected(selection) = selection_query.single();
    if !selection.is_empty() {
        NinePatchWindow::new(
            egui::RichText::new("Selected Units")
                .text_style(egui::TextStyle::Name("Heading2".into())),
        )
        .fixed_size(egui::vec2(320., 160.))
        .default_pos(egui::pos2(4., 300.))
        .title_bar_nine_patch(
            *gui_context
                .get_texture_id(TextureType::Window, "dark")
                .unwrap(),
            egui::vec2(32., 32.),
        )
        .body_nine_patch(
            *gui_context
                .get_texture_id(TextureType::Window, "bright")
                .unwrap(),
            egui::vec2(32., 32.),
        )
        .frame(
            egui::Frame::window(&egui_context.ctx_mut().style())
                .inner_margin(egui::style::Margin::symmetric(8., 0.)),
        )
        .show(egui_context.ctx_mut(), |ui| {
            let entities = selection.entities();
            for entity in entities {
                match entity {
                    SelectedEntity::Unit(entity) => {
                        if let Ok(unit_type) = unit_query.get(*entity) {
                            ui.label(format!("Unit: {:?}", unit_type));
                        }
                    }
                    SelectedEntity::City(entity) => {
                        if let Ok(city_type) = city_query.get(*entity) {
                            ui.label(format!("City: {:?}", city_type));
                        }
                    }
                }
            }
        });
    }
}
