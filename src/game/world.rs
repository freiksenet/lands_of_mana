use strum_macros::{EnumIter, EnumString};

use crate::prelude::*;
#[derive(Component, Debug, Default)]
pub struct Player {}

#[derive(Component, Debug, Default)]
pub struct PlayerName(pub String);

#[derive(Component, Debug, Default)]
pub struct PlayerColor(pub Color);

#[derive(Bundle, Debug, Default)]
pub struct PlayerBundle {
    pub player: Player,
    pub name: PlayerName,
    pub color: PlayerColor,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Component, Debug, PartialEq, Hash)]

pub struct OfPlayer(pub Entity);

#[derive(Bundle, Debug)]
pub struct PlayerStockpileBundle {
    pub resource: StockpileResourceType,
    pub amount: StockpileResourceAmount,
    pub player: OfPlayer,
}

#[derive(Component, Debug)]
pub struct StockpileResourceAmount(pub f32);

#[derive(Component, Debug, Clone, Copy, EnumIter, EnumString, Eq, PartialEq, Default, Hash)]
pub enum StockpileResourceType {
    #[default]
    Gold,
    Wood,
}

#[derive(Component, Debug)]
pub struct StockpileResourceProsumer(pub f32);

#[derive(Bundle, Debug)]
pub struct StockpileResourceProsumerBundle {
    pub resource: StockpileResourceType,
    pub prosumer: StockpileResourceProsumer,
    pub player: OfPlayer,
}

#[derive(Bundle, Debug)]
pub struct PlayerCapacityBundle {
    pub player: OfPlayer,
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

#[derive(Component, Debug)]
pub struct CapacityResourceProsumer(pub i32);

#[derive(Bundle, Debug)]
pub struct CapacityResourceProsumerBundle {
    pub resource: CapacityResourceType,
    pub prosumer: CapacityResourceProsumer,
    pub player: OfPlayer,
}
