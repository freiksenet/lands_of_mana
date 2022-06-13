use num_derive::FromPrimitive;

use crate::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Map {
    pub width: u32,
    pub height: u32,
}

impl Map {
    /// Get world midpoint in pixels
    pub fn get_pixel_midpoint(&self) -> Vec2 {
        Vec2::new(
            (self.width * 16) as f32 / 2.,
            (self.height * 16) as f32 / 2.,
        )
    }

    pub fn pixel_position_to_position(&self, pixel_position: Vec2) -> game::map::Position {
        let corner_position = pixel_position + self.get_pixel_midpoint();
        game::map::Position {
            x: (corner_position.x / 16.0).floor() as u32,
            y: (corner_position.y / 16.0).floor() as u32,
        }
    }

    /// for position, get pixel position of (0,0) of a tile
    pub fn position_to_pixel_position(&self, position: &game::map::Position) -> Vec2 {
        Vec2::new((position.x * 16) as f32, (position.y * 16) as f32) - self.get_pixel_midpoint()
    }
}

#[derive(Component, Debug, Clone)]
pub struct ProvinceBorder {
    pub color: Color,
}

#[derive(Component, Debug, Clone)]
pub struct TerrainBase {
    pub terrain_type: TerrainType,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Bundle, Clone, Debug)]
pub struct TerrainBundle {
    pub province: super::province::InProvince,
    pub position: Position,
    pub base: TerrainBase,
}

/// Terrain number indicates priority ordering when rendering (higher = higher priority)
/// It is also a texture id for base land
#[allow(dead_code)]
#[derive(Clone, Debug, Copy, PartialOrd, Ord, Eq, PartialEq, Hash, FromPrimitive)]
pub enum TerrainType {
    Water = 0,
    WaterOcean = 1,
    Ice = 2,
    WaterSwamp = 3,
    DesertRed = 4,
    DesertRedCracked = 5,
    SnowBlue = 6,
    SnowBlueDune = 7,
    Lava = 8,
    LavaCracks = 9,
    Desert = 10,
    DesertDune = 11,
    Snow = 12,
    SnowDune = 13,
    Dirt = 14,
    DirtGrass = 15,
    DesertYellow = 16,
    DesertYellowCracked = 17,
    Swamp = 18,
    SwampBog = 19,
    SwampReeds = 20,
    GrassLand = 21,
    GrassLandPasture = 22,
}
