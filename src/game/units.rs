use bevy::prelude::*;
use strum_macros::{EnumIter, EnumString};

use crate::game::map;

#[derive(Component, Debug, Clone)]
pub struct UnitFigures {
    pub health: Vec<u32>,
}

#[derive(Bundle, Debug, Clone)]
pub struct UnitBundle {
    pub unit: Unit,
    pub position: map::Position,
    pub figures: UnitFigures,
}

#[derive(Component, Debug, Clone)]
pub struct Unit {
    pub unit_type: UnitType,
}

#[derive(Component, Clone, Copy, Debug, EnumString, EnumIter)]
pub enum UnitType {
    Skeleton,
    DeathKnight,
    GiantSpider,
}
