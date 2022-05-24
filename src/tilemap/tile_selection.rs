use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
/// Module that knows how to select tiles based on neighbors
/// 1. Tile terrain determines the base tile, it is pushed to result tile list
/// 2. List of corner tiles is selected
///   a. If all 4 corners are same as base - proceed
///   b. For each corner, order them. If the terrain is lower priority, ignore corner
///   c. For each higher priority terrain insert corresponding piece
///   CORNER_LEFT_TOP, CORNER_RIGHT_TOP, CORNER_LEFT_BOTTOM, CORNER_RIGHT_BOTTOM
///   LEFT, TOP, RIGHT, BOTTOM
///   INCORNER_LEFT_TOP, INCORNER_RIGHT_TOP, INCORNER_LEFT_BOTTOM, INCORNER_RIGHT_BOTTOM
///
///
use std::{cmp::min, collections::BTreeMap, fmt::Debug, ops::Add};

use super::game::map::TerrainType;

#[derive(Clone, Debug, PartialEq, Eq, FromPrimitive, Copy)]
pub enum Corner {
    Empty = 0,
    LeftTop = 8,
    RightTop = 1,
    LeftBottom = 4,
    RightBottom = 2,
    Left = 12,
    Top = 9,
    Right = 3,
    Bottom = 6,
    LeftTopL = 13,
    RightTopL = 11,
    LeftBottomL = 14,
    RightBottomL = 7,
    LeftTopAndRightBottom = 10,
    LeftBottomAndRightTop = 5,
    Square = 15,
}

impl Add for Corner {
    type Output = Corner;

    fn add(self, other: Corner) -> Corner {
        let value = self as usize + other as usize;
        FromPrimitive::from_usize(min(value, Corner::Square as usize)).unwrap()
    }
}

fn get_texture_location(terrain_type: &TerrainType) -> u32 {
    match terrain_type {
        TerrainType::Sea | TerrainType::Ocean => 0,
        TerrainType::Dryland | TerrainType::DrylandCracked => 1,
        TerrainType::Grass | TerrainType::Pasture => 2,
        TerrainType::SnowBlue | TerrainType::SnowBlueDune => 3,
        TerrainType::Lava | TerrainType::LavaCracks => 4,
        TerrainType::Swamp | TerrainType::WaterSwamp => 5,
        TerrainType::Sand | TerrainType::SandDune => 6,
        TerrainType::Snow | TerrainType::SnowDune => 7,
        TerrainType::Dirt | TerrainType::Moss => 8,
        TerrainType::Wasteland | TerrainType::WastelandCracked => 9,
        TerrainType::Bog | TerrainType::Reeds => 10,
        TerrainType::Ice => 11,
    }
}

trait TerrainDescription {
    fn get_base_texture_id(&self) -> u32;
    fn get_texture_id_for_corner(&self, base: &TerrainType, corner: &Corner) -> u32;
}

impl TerrainDescription for TerrainType {
    fn get_base_texture_id(&self) -> u32 {
        match self {
            TerrainType::Sea
            | TerrainType::Dryland
            | TerrainType::Grass
            | TerrainType::SnowBlue
            | TerrainType::Lava
            | TerrainType::Swamp
            | TerrainType::Sand
            | TerrainType::Snow
            | TerrainType::Dirt
            | TerrainType::Wasteland
            | TerrainType::Bog
            | TerrainType::Ice => get_texture_location(self),
            TerrainType::Ocean
            | TerrainType::DrylandCracked
            | TerrainType::Pasture
            | TerrainType::SnowBlueDune
            | TerrainType::LavaCracks
            | TerrainType::WaterSwamp
            | TerrainType::SandDune
            | TerrainType::SnowDune
            | TerrainType::Moss
            | TerrainType::WastelandCracked
            | TerrainType::Reeds => get_texture_location(self) + 11,
        }
    }

    fn get_texture_id_for_corner(&self, base: &TerrainType, corner: &Corner) -> u32 {
        let texture_location = get_texture_location(self);
        ((texture_location - 1) * 102)
            + (if base == &TerrainType::Sea {
                match corner {
                    Corner::LeftTop => 20,
                    Corner::RightTop => 19,
                    Corner::LeftBottom => 3,
                    Corner::RightBottom => 2,
                    Corner::Left => 23,
                    Corner::Top => 7,
                    Corner::Right => 25,
                    Corner::Bottom => 41,
                    Corner::LeftTopL => 4,
                    Corner::RightTopL => 5,
                    Corner::LeftBottomL => 21,
                    Corner::RightBottomL => 22,
                    Corner::LeftTopAndRightBottom => 34,
                    Corner::LeftBottomAndRightTop => 35,
                    Corner::Empty => todo!(),
                    Corner::Square => todo!(),
                }
            } else {
                match corner {
                    Corner::LeftTop => 47,
                    Corner::RightTop => 45,
                    Corner::LeftBottom => 13,
                    Corner::RightBottom => 11,
                    Corner::Left => 30,
                    Corner::Top => 46,
                    Corner::Right => 28,
                    Corner::Bottom => 12,
                    Corner::LeftTopL => 26,
                    Corner::RightTopL => 27,
                    Corner::LeftBottomL => 43,
                    Corner::RightBottomL => 44,
                    Corner::LeftTopAndRightBottom => 9,
                    Corner::LeftBottomAndRightTop => 10,
                    Corner::Empty => todo!(),
                    Corner::Square => todo!(),
                }
            })
    }
}

pub struct TerrainCorners {
    pub center: TerrainType,
    pub north: TerrainType,
    pub south: TerrainType,
    pub west: TerrainType,
    pub east: TerrainType,
    pub north_west: TerrainType,
    pub north_east: TerrainType,
    pub south_west: TerrainType,
    pub south_east: TerrainType,
}

pub trait TerrainCornersTexture {
    fn get_tile_textures(&self) -> Vec<u32>;

    fn get_higher_sides(&self) -> Vec<(TerrainType, Corner)>;
}

impl TerrainCornersTexture for TerrainCorners {
    fn get_higher_sides(&self) -> Vec<(TerrainType, Corner)> {
        [
            (self.west, Corner::Left),
            (self.east, Corner::Right),
            (self.north, Corner::Top),
            (self.south, Corner::Bottom),
        ]
        .iter()
        .filter(|(terrain, _)| terrain > &self.center)
        .copied()
        .collect()
    }

    fn get_tile_textures(&self) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::new();
        result.push(self.center.get_base_texture_id());
        // Figure out if we need to draw corners
        let mut corners: Vec<u32> = Vec::new();

        if (self.north_west > self.center)
            && ((self.north_west > self.north) && (self.north_west > self.west))
        {
            corners.push(
                self.north_west
                    .get_texture_id_for_corner(&self.center, &Corner::LeftTop),
            );
        }

        if (self.north_east > self.center)
            && ((self.north_east > self.north) && (self.north_east > self.east))
        {
            corners.push(
                self.north_east
                    .get_texture_id_for_corner(&self.center, &Corner::RightTop),
            );
        }

        if (self.south_west > self.center)
            && ((self.south_west > self.south) && (self.south_west > self.west))
        {
            corners.push(
                self.south_west
                    .get_texture_id_for_corner(&self.center, &Corner::LeftBottom),
            );
        }

        if (self.south_east > self.center)
            && ((self.south_east > self.south) && (self.south_east > self.east))
        {
            corners.push(
                self.south_east
                    .get_texture_id_for_corner(&self.center, &Corner::RightBottom),
            );
        }

        let mut corner_map: BTreeMap<&TerrainType, Corner> = BTreeMap::new();
        let sides = self.get_higher_sides();
        for (terrain, corner) in sides.iter() {
            let existing_corner = corner_map.get(terrain).unwrap_or(&Corner::Empty);
            corner_map.insert(terrain, *corner + *existing_corner);
        }

        for (terrain, corner) in corner_map {
            result.push(terrain.get_texture_id_for_corner(&self.center, &corner))
        }

        result.append(&mut corners);
        result
    }
}
