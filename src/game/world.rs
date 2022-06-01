use strum_macros::{EnumIter, EnumString};

use crate::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct GameWorld {
    pub width: u32,
    pub height: u32,
}

impl GameWorld {
    /// Get world midpoint in pixels
    pub fn get_pixel_midpoint(&self) -> Vec2 {
        Vec2::new(
            (self.width * 16) as f32 / 2. - 8.,
            (self.height * 16) as f32 / 2. - 8.,
        )
    }

    pub fn pixel_position_to_position(&self, pixel_position: Vec2) -> game::map::Position {
        let corner_position = pixel_position + self.get_pixel_midpoint();
        game::map::Position {
            x: ((corner_position.x + 8.) / 16.0).floor() as u32,
            y: ((corner_position.y + 8.) / 16.0).floor() as u32,
        }
    }

    /// for position, get pixel position of (0,0) of a tile
    pub fn position_to_pixel_position(&self, position: &game::map::Position) -> Vec2 {
        Vec2::new((position.x * 16) as f32, (position.y * 16) as f32) - self.get_pixel_midpoint()
    }
}

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
