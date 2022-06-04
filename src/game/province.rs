use std::collections::HashMap;

use bevy::ecs::system::EntityCommands;
use strum_macros::{EnumIter, EnumString};

use super::world;
use crate::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Province {
    pub name: String,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct InProvince(pub Entity);

impl Default for InProvince {
    fn default() -> Self {
        Self(Entity::from_raw(0))
    }
}

#[derive(Component, Clone, Copy, Debug, EnumString, EnumIter)]
pub enum CityType {
    Capital,
    City,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct OfCity(pub Entity);

impl CityType {
    pub fn get_city_stats(&self) -> CityStats {
        match self {
            CityType::Capital => CityStats {
                city_type: CityType::Capital,
                base_stockpile_prosumers: HashMap::from([
                    (world::StockpileResourceType::Gold, 50.),
                    (world::StockpileResourceType::Wood, 10.),
                ]),
                base_capacity_prosumers: HashMap::new(),
            },
            CityType::City => CityStats {
                city_type: CityType::Capital,
                base_stockpile_prosumers: HashMap::from([(
                    world::StockpileResourceType::Gold,
                    15.,
                )]),
                base_capacity_prosumers: HashMap::from([(world::CapacityResourceType::Arcana, 2)]),
            },
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct CityBundle {
    pub province: InProvince,
    pub position: super::map::Position,
    pub city_type: CityType,
    pub player: super::world::OfPlayer,
}

impl CityBundle {
    pub fn new_empty_city(
        entity: &mut EntityCommands,
        player_entity: Entity,
        city_stats: CityStats,
        province: Entity,
        position: super::map::Position,
    ) -> Entity {
        entity
            .insert_bundle(CityBundle {
                province: InProvince(province),
                position,
                city_type: city_stats.city_type.clone(),
                player: super::world::OfPlayer(player_entity),
            })
            .with_children(|builder| {
                for (resource, amount) in &city_stats.base_stockpile_prosumers {
                    builder
                        .spawn()
                        .insert(super::world::StockpileResourceProsumer {
                            resource: *resource,
                            amount: *amount,
                        })
                        .insert(super::world::OfPlayer(player_entity));
                }

                for (resource, amount) in &city_stats.base_capacity_prosumers {
                    builder
                        .spawn()
                        .insert(super::world::CapacityResourceProsumer {
                            resource: *resource,
                            amount: *amount,
                        })
                        .insert(super::world::OfPlayer(player_entity));
                }
            })
            .id()
    }
}

#[derive(Clone, Debug)]
pub struct CityStats {
    pub city_type: CityType,
    pub base_stockpile_prosumers: HashMap<super::world::StockpileResourceType, f32>,
    pub base_capacity_prosumers: HashMap<super::world::CapacityResourceType, i32>,
}
