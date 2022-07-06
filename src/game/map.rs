use num_derive::FromPrimitive;

use crate::prelude::*;

#[derive(Component, Debug)]
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

    pub fn pixel_position_to_cursor_position(
        &self,
        pixel_position: Vec2,
    ) -> (Option<Position>, Position) {
        let corner_position = pixel_position + self.get_pixel_midpoint();
        let x = (corner_position.x / 16.0).floor() as i32;
        let y = (corner_position.y / 16.0).floor() as i32;
        let bound_x = clamp(
            (corner_position.x / 16.0).floor() as i32,
            0,
            (self.width - 1) as i32,
        );
        let bound_y = clamp(
            (corner_position.y / 16.0).floor() as i32,
            0,
            (self.height - 1) as i32,
        );
        let exact_position = if x == bound_x && y == bound_y {
            Some(Position::new(x as u32, y as u32))
        } else {
            None
        };

        (
            exact_position,
            Position::new(bound_x as u32, bound_y as u32),
        )
    }

    /// for position, get pixel position of (0,0) of a tile
    pub fn position_to_pixel_position(&self, position: &Position) -> Vec2 {
        Vec2::new((position.x * 16) as f32, (position.y * 16) as f32) - self.get_pixel_midpoint()
    }
}

pub fn clamp<A>(input: A, min: A, max: A) -> A
where
    A: std::cmp::Ord,
{
    std::cmp::min(std::cmp::max(input, min), max)
}
#[derive(Component, Debug)]
pub struct ProvinceBorder {
    pub color: Color,
}

#[derive(Component, Debug, Default)]
pub struct TerrainBase(pub TerrainType);

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Position { x, y }
    }

    pub fn shift(&self, x: u32, y: u32) -> Position {
        Position {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::new(0, 0)
    }
}

#[derive(Component, Debug, Default)]
pub struct Terrain {}

#[derive(Bundle, Debug, Default)]
pub struct TerrainBundle {
    pub terrain: Terrain,
    pub province: super::province::InProvince,
    pub position: Position,
    pub base: TerrainBase,
}

/// Terrain number indicates priority ordering when rendering (higher = higher priority)
/// It is also a texture id for base land
#[allow(dead_code)]
#[derive(Clone, Debug, Default, Copy, PartialOrd, Ord, Eq, PartialEq, Hash, FromPrimitive)]
pub enum TerrainType {
    #[default]
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
