use bevy::{ecs::system::EntityCommands, prelude::*};
use leafwing_input_manager::prelude::*;
use strum_macros::{EnumIter, EnumString};

use crate::game::map;
use crate::ui;

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
    pub fn insert_full(entity: &mut EntityCommands, unit: Unit, position: map::Position) -> Entity {
        let unit_stats = unit.unit_type.get_unit_stats();
        entity
            .insert_bundle(UnitBundle { unit, position })
            .with_children(|unit| {
                for _ in 0..unit_stats.max_figures {
                    unit.spawn().insert(UnitFigure {
                        health: unit_stats.max_health,
                    });
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
    DebugBox,
    Skeleton,
    DeathKnight,
    GiantSpider,
}

#[derive(Clone, Copy, Debug)]
pub struct UnitStats {
    pub max_figures: u32,
    pub max_health: u32,
}

impl UnitType {
    pub fn get_unit_stats(&self) -> UnitStats {
        match self {
            UnitType::DebugBox => UnitStats {
                max_figures: 1,
                max_health: 1,
            },
            UnitType::Skeleton => UnitStats {
                max_figures: 4,
                max_health: 4,
            },
            UnitType::DeathKnight => UnitStats {
                max_figures: 2,
                max_health: 10,
            },
            UnitType::GiantSpider => UnitStats {
                max_figures: 1,
                max_health: 20,
            },
        }
    }
}
