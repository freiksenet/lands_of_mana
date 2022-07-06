use bevy_egui::{egui, EguiContext};

use crate::{
    config::{EngineState, UiSyncLabel},
    game::{
        map::{Position, Terrain, TerrainBase},
        province::{City, CityType, Province},
        units::{Unit, UnitType},
        world::{OfPlayer, Player, PlayerColor, PlayerName},
    },
    prelude::*,
    ui::{CursorDebugTooltipTarget, EntityOnTile},
};

pub struct DebugTooltipPlugin {}

impl Plugin for DebugTooltipPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            config::Stage::UiSync,
            ConditionSet::new()
                .run_in_state(EngineState::InGame)
                .label_and_after(UiSyncLabel::Update)
                .with_system(debug_tooltip)
                .into(),
        );
    }
}

fn debug_tooltip(
    mut egui_context: ResMut<EguiContext>,
    debug_tooltip_query: Query<&CursorDebugTooltipTarget>,
    terrain_query: Query<(&TerrainBase, &Position), With<Terrain>>,
    province_query: Query<&Province>,
    city_query: Query<(&CityType, Option<&OfPlayer>), With<City>>,
    unit_query: Query<(&UnitType, &OfPlayer), With<Unit>>,
    player_query: Query<(&PlayerName, &PlayerColor), With<Player>>,
) {
    if let Ok(CursorDebugTooltipTarget {
        entities: Some(entities),
    }) = debug_tooltip_query.get_single()
    {
        egui::Window::new("Debug Tooltip")
            .auto_sized()
            .min_width(400.)
            .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(5., 5.))
            .show(egui_context.ctx_mut(), |ui| {
                for entity in entities {
                    let text = match *entity {
                        EntityOnTile::Terrain(entity) => {
                            if let Ok((TerrainBase(terrain_type), position)) =
                                terrain_query.get(entity)
                            {
                                Some(format!(
                                    "Terrain {:?} at {:?}x{:?}",
                                    terrain_type, position.x, position.y
                                ))
                            } else {
                                None
                            }
                        }
                        EntityOnTile::Province {
                            province_entity, ..
                        } => {
                            if let Ok(province) = province_query.get(province_entity) {
                                Some(format!("Province {:?}", province.name))
                            } else {
                                None
                            }
                        }
                        EntityOnTile::City { city_entity, .. } => {
                            match city_query.get(city_entity) {
                                Ok((city_type, Some(&OfPlayer(player)))) => {
                                    if let Ok((name, color)) = player_query.get(player) {
                                        Some(format!(
                                            "City {:?} of Player {:?} ({})",
                                            city_type,
                                            name.0,
                                            format_color(&color.0),
                                        ))
                                    } else {
                                        Some(format!("City {:?}", city_type))
                                    }
                                }
                                Ok((city_type, _)) => Some(format!("City {:?}", city_type)),
                                _ => None,
                            }
                        }
                        EntityOnTile::Unit(entity) => {
                            if let Ok((unit_type, &OfPlayer(player))) = unit_query.get(entity) {
                                if let Ok((name, color)) = player_query.get(player) {
                                    Some(format!(
                                        "Unit {:?} of Player {:?} ({})",
                                        unit_type,
                                        name.0,
                                        format_color(&color.0),
                                    ))
                                } else {
                                    Some(format!("Unit {:?}", unit_type))
                                }
                            } else {
                                None
                            }
                        }
                    };
                    ui.label(
                        text.unwrap_or_else(|| "INVALID ENTITY".to_string())
                            .as_str(),
                    );
                }
            });
    }
}

fn format_color(color: &Color) -> String {
    format!(
        "{:02X}{:02X}{:02X}",
        (color.r() * 255.) as usize,
        (color.g() * 255.) as usize,
        (color.b() * 255.) as usize
    )
}
