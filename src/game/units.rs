use std::collections::HashMap;

use bevy::ecs::system::EntityCommands;
use strum_macros::{EnumIter, EnumString};

use crate::prelude::{
    game::{map, world},
    *,
};

#[derive(Component, Debug, Clone)]
pub struct UnitFigure {
    pub health: u32,
}

#[derive(Bundle, Debug, Clone)]
pub struct UnitBundle {
    pub unit: Unit,
    pub position: map::Position,
}

impl UnitBundle {
    pub fn insert_full(
        entity: &mut EntityCommands,
        player_entity: Entity,
        unit: Unit,
        position: map::Position,
    ) -> Entity {
        let unit_stats = unit.unit_type.get_unit_stats();
        entity
            .insert_bundle(UnitBundle { unit, position })
            .with_children(|unit| {
                for _ in 0..unit_stats.max_figures {
                    unit.spawn().insert(UnitFigure {
                        health: unit_stats.max_health,
                    });
                }
                for (resource, amount) in &unit_stats.capacity_cost {
                    unit.spawn()
                        .insert(world::CapacityResourceProsumer {
                            resource: *resource,
                            amount: *amount,
                        })
                        .insert(world::OfPlayer(player_entity));
                }
            })
            .insert(ui::Selectable {
                ..Default::default()
            })
            .id()
    }
}

#[derive(Component, Debug, Clone)]
pub struct Unit {
    pub unit_type: UnitType,
}

#[derive(Clone, Copy, Debug, EnumString, EnumIter)]
pub enum UnitType {
    Skeleton,
    DeathKnight,
    GiantSpider,
}

#[derive(Clone, Debug)]
pub struct UnitStats {
    pub max_figures: u32,
    pub max_health: u32,
    pub cost: HashMap<world::StockpileResourceType, f32>,
    pub capacity_cost: HashMap<world::CapacityResourceType, i32>,
}

impl UnitType {
    pub fn get_unit_stats(&self) -> UnitStats {
        match self {
            UnitType::Skeleton => UnitStats {
                max_figures: 4,
                max_health: 4,
                cost: HashMap::from([(world::StockpileResourceType::Gold, 100.)]),
                capacity_cost: HashMap::from([(world::CapacityResourceType::Death, -1)]),
            },
            UnitType::DeathKnight => UnitStats {
                max_figures: 2,
                max_health: 10,
                cost: HashMap::from([(world::StockpileResourceType::Gold, 200.)]),
                capacity_cost: HashMap::from([(world::CapacityResourceType::Death, -1)]),
            },
            UnitType::GiantSpider => UnitStats {
                max_figures: 1,
                max_health: 20,
                cost: HashMap::from([(world::StockpileResourceType::Gold, 500.)]),
                capacity_cost: HashMap::from([(world::CapacityResourceType::Death, -1)]),
            },
        }
    }
}
