use std::collections::HashMap;

use bevy::ecs::system::EntityCommands;
use strum_macros::{EnumIter, EnumString};

use crate::{
    game::{
        map::Position,
        world::{
            CapacityResourceProsumer, CapacityResourceProsumerBundle, CapacityResourceType,
            OfPlayer, StockpileResourceProsumer, StockpileResourceProsumerBundle,
            StockpileResourceType,
        },
    },
    prelude::*,
};

#[derive(Component, Debug)]
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

#[derive(Component, Debug, Default)]
pub struct City {}

#[derive(Component, Clone, Copy, Debug, EnumString, EnumIter, Default)]
pub enum CityType {
    #[default]
    Empty,
    City,
    MageTower,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct OfCity(pub Entity);

impl CityType {
    pub fn get_city_stats(&self) -> CityStats {
        match self {
            CityType::Empty => CityStats {
                city_type: CityType::Empty,
                base_stockpile_prosumers: HashMap::new(),
                base_capacity_prosumers: HashMap::new(),
                size: (2, 2),
            },
            CityType::MageTower => CityStats {
                city_type: CityType::MageTower,
                base_stockpile_prosumers: HashMap::from([
                    (StockpileResourceType::Gold, 50.),
                    (StockpileResourceType::Wood, 10.),
                ]),
                base_capacity_prosumers: HashMap::from([
                    (CapacityResourceType::Arcana, 5),
                    (CapacityResourceType::Chaos, 5),
                    (CapacityResourceType::Death, 5),
                    (CapacityResourceType::Nature, 5),
                    (CapacityResourceType::Sun, 5),
                ]),
                size: (2, 2),
            },
            CityType::City => CityStats {
                city_type: CityType::City,
                base_stockpile_prosumers: HashMap::from([(StockpileResourceType::Gold, 15.)]),
                base_capacity_prosumers: HashMap::new(),
                size: (2, 2),
            },
        }
    }
}

#[derive(Bundle, Debug, Default)]
pub struct CityBundle {
    pub city: City,
    pub province: InProvince,
    pub position: Position,
    pub city_type: CityType,
}

#[derive(Component, Debug, Default)]
pub struct CityTileIndex(pub usize, pub usize);

#[derive(Bundle, Debug, Default)]
pub struct CityTileBundle {
    pub index: CityTileIndex,
    pub city_type: CityType,
    pub position: Position,
}

impl CityBundle {
    pub fn new_empty_city(
        entity: &mut EntityCommands,
        player_entity: Entity,
        city_stats: CityStats,
        province: Entity,
        position: Position,
    ) -> Entity {
        entity
            .insert_bundle(CityBundle {
                province: InProvince(province),
                position,
                city_type: city_stats.city_type,
                ..Default::default()
            })
            .insert(super::world::OfPlayer(player_entity))
            .with_children(|builder| {
                for x in 0..city_stats.size.0 {
                    for y in 0..city_stats.size.1 {
                        builder.spawn().insert_bundle(CityTileBundle {
                            city_type: city_stats.city_type,
                            index: CityTileIndex(x, y),
                            position: position.shift(x as u32, y as u32),
                        });
                    }
                }
                for (resource, amount) in &city_stats.base_stockpile_prosumers {
                    builder
                        .spawn()
                        .insert_bundle(StockpileResourceProsumerBundle {
                            player: OfPlayer(player_entity),
                            resource: *resource,
                            prosumer: StockpileResourceProsumer(*amount),
                        })
                        .insert(OfPlayer(player_entity));
                }

                for (resource, amount) in &city_stats.base_capacity_prosumers {
                    builder
                        .spawn()
                        .insert_bundle(CapacityResourceProsumerBundle {
                            player: OfPlayer(player_entity),
                            resource: *resource,
                            prosumer: CapacityResourceProsumer(*amount),
                        })
                        .insert(OfPlayer(player_entity));
                }
            })
            .id()
    }
}

#[derive(Debug)]
pub struct CityStats {
    pub city_type: CityType,
    pub base_stockpile_prosumers: HashMap<StockpileResourceType, f32>,
    pub base_capacity_prosumers: HashMap<CapacityResourceType, i32>,
    pub size: (usize, usize),
}
