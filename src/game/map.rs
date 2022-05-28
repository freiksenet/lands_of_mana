use strum_macros::{EnumIter, EnumString};

use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct GameWorld {
    pub width: u32,
    pub height: u32,
}

impl GameWorld {
    /// Get world midpoint in pixels
    pub fn get_pixel_midpoint(&self) -> Vec3 {
        Vec3::new(
            (self.width * 16) as f32 / 2. - 8.,
            (self.height * 16) as f32 / 2. - 8.,
            0.,
        )
    }

    pub fn pixel_position_to_position(&self, pixel_position: Vec3) -> Position {
        let corner_position = pixel_position + self.get_pixel_midpoint();
        Position {
            x: (corner_position.x / 16.0).floor() as u32,
            y: (corner_position.y / 16.0).floor() as u32,
        }
    }

    /// for position, get pixel position of (0,0) of a tile
    pub fn position_to_pixel_position(&self, position: &Position) -> Vec3 {
        Vec3::new((position.x * 16) as f32, (position.y * 16) as f32, 0.)
            - self.get_pixel_midpoint()
    }
}

#[derive(Component, Debug, Clone)]
pub struct Province {
    pub name: String,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct ProvinceId(pub Entity);

impl Default for ProvinceId {
    fn default() -> Self {
        Self(Entity::from_raw(0))
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct City {
    pub city_type: CityType,
}

#[derive(Component, Clone, Copy, Debug, EnumString, EnumIter)]
pub enum CityType {
    City1,
    City2,
    Desert,
    Barbarian,
    Mystic,
    Pyramid,
    Dwarf,
    Lizardmen,
    Elf,
}

#[derive(Bundle, Clone, Debug)]
pub struct CityBundle {
    pub province: ProvinceId,
    pub position: Position,
    pub city: City,
}

#[derive(Component, Debug, Clone)]
pub struct InProvince {
    pub province: Entity,
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
    pub province: ProvinceId,
    pub position: Position,
    pub base: TerrainBase,
}

/// Terrain number indicates priority ordering when rendering (higher = higher priority)
/// It is also a texture id for base land
#[allow(dead_code)]
#[derive(Clone, Debug, Copy, PartialOrd, Ord, Eq, PartialEq, Hash)]
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
