use strum_macros::{EnumIter, EnumString};

use crate::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Viewer {}

#[derive(Component, Debug, Clone)]
pub struct PlayerName(pub String);

#[derive(Component, Debug, Clone)]
pub struct PlayerColor(pub Color);

#[derive(Bundle, Clone, Debug)]
pub struct PlayerBundle {
    pub name: PlayerName,
    pub color: PlayerColor,
}

#[derive(Component, Debug, Clone)]

pub struct OfPlayer(pub Entity);

#[derive(Bundle, Clone, Debug)]
pub struct PlayerStockpileBundle {
    pub resource: StockpileResourceType,
    pub amount: StockpileResourceAmount,
}

#[derive(Component, Debug, Clone)]
pub struct StockpileResourceAmount(pub f32);

#[derive(Component, Debug, Clone, Copy, EnumIter, EnumString, Eq, PartialEq, Default, Hash)]
pub enum StockpileResourceType {
    #[default]
    Gold,
    Wood,
}

#[derive(Component, Clone, Debug)]
pub struct StockpileResourceProsumer {
    pub resource: StockpileResourceType,
    pub amount: f32,
}

#[derive(Bundle, Clone, Debug)]
pub struct PlayerCapacityBundle {
    pub resource: CapacityResourceType,
}

#[derive(Component, Debug, Clone, Copy, EnumIter, EnumString, Eq, PartialEq, Default, Hash)]
pub enum CapacityResourceType {
    #[default]
    Sun,
    Arcana,
    Death,
    Chaos,
    Nature,
}

#[derive(Component, Clone, Debug)]
pub struct CapacityResourceProsumer {
    pub resource: CapacityResourceType,
    pub amount: i32,
}
