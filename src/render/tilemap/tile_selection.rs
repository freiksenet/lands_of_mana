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

trait TerrainDescription {
    fn get_base_texture_id(&self) -> u32;
    fn get_base_variant(&self) -> Self;
    fn get_texture_id_for_corner(&self, center: &TerrainType, corner: &Corner) -> Option<u32>;
}

const CONNECTOR_TERRAIN_SIZE: u32 = 54;

impl TerrainDescription for TerrainType {
    fn get_base_texture_id(&self) -> u32 {
        *self as u32
    }

    fn get_base_variant(&self) -> Self {
        match self {
            TerrainType::Water | TerrainType::WaterOcean => TerrainType::Water,
            TerrainType::Ice => TerrainType::Ice,
            TerrainType::WaterSwamp => TerrainType::WaterSwamp,
            TerrainType::DesertRed | TerrainType::DesertRedCracked => TerrainType::DesertRed,
            TerrainType::SnowBlue | TerrainType::SnowBlueDune => TerrainType::SnowBlue,
            TerrainType::Lava | TerrainType::LavaCracks => TerrainType::Lava,
            TerrainType::Desert | TerrainType::DesertDune => TerrainType::Desert,
            TerrainType::Snow | TerrainType::SnowDune => TerrainType::Snow,
            TerrainType::Dirt | TerrainType::DirtGrass => TerrainType::Dirt,
            TerrainType::DesertYellow | TerrainType::DesertYellowCracked => {
                TerrainType::DesertYellow
            }
            TerrainType::Swamp | TerrainType::SwampBog | TerrainType::SwampReeds => {
                TerrainType::Swamp
            }
            TerrainType::GrassLand | TerrainType::GrassLandPasture => TerrainType::GrassLand,
        }
    }

    /// return a texture id for connectors. Nothing means - don't draw anything (invalid connector most likely)
    fn get_texture_id_for_corner(&self, center: &TerrainType, corner: &Corner) -> Option<u32> {
        let terrain = *self;
        let base_variant = terrain.get_base_variant();
        let base_center_variant = center.get_base_variant();
        let base_offset = self.get_base_texture_id() * CONNECTOR_TERRAIN_SIZE;
        let base_variant_offset = base_variant.get_base_texture_id() * CONNECTOR_TERRAIN_SIZE;
        let center_offset = center.get_base_texture_id() * CONNECTOR_TERRAIN_SIZE;

        // check if terrain is a variant of a base, use connector tiles
        if base_variant == base_center_variant {
            return get_variant_texture_id(corner).map(|id| id + base_offset);
        }
        // check if terrain is one of the sea types, here some seas (Ice, Swamp) are exclusive to their parent tiles
        // and will return None
        else if base_variant == TerrainType::Swamp
            && base_center_variant == TerrainType::WaterSwamp
        {
            if terrain == TerrainType::Swamp || terrain == TerrainType::SwampBog {
                return get_water_texture_id(corner).map(|id| id + center_offset);
            } else if terrain == TerrainType::SwampReeds {
                return get_water_special_texture_id(corner).map(|id| id + center_offset);
            }
        } else if base_variant == TerrainType::Snow && base_center_variant == TerrainType::Ice {
            if terrain == TerrainType::Snow {
                return get_water_texture_id(corner).map(|id| id + center_offset);
            } else if terrain == TerrainType::SnowDune {
                return get_water_special_texture_id(corner).map(|id| id + center_offset);
            }
        }
        // check if terrain is water
        else if base_center_variant == TerrainType::Water {
            return get_water_texture_id(corner).map(|id| id + base_offset);
        }
        // normal connectors
        else if base_center_variant != TerrainType::WaterSwamp
            && base_center_variant != TerrainType::Ice
        {
            return get_connector_texture_id(corner).map(|id| id + base_variant_offset);
        }

        None
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
            if let Some(corner) = self
                .north_west
                .get_texture_id_for_corner(&self.center, &Corner::LeftTop)
            {
                corners.push(corner);
            }
        }

        if (self.north_east > self.center)
            && ((self.north_east > self.north) && (self.north_east > self.east))
        {
            if let Some(corner) = self
                .north_east
                .get_texture_id_for_corner(&self.center, &Corner::RightTop)
            {
                corners.push(corner);
            }
        }

        if (self.south_west > self.center)
            && ((self.south_west > self.south) && (self.south_west > self.west))
        {
            if let Some(corner) = self
                .south_west
                .get_texture_id_for_corner(&self.center, &Corner::LeftBottom)
            {
                corners.push(corner);
            }
        }

        if (self.south_east > self.center)
            && ((self.south_east > self.south) && (self.south_east > self.east))
        {
            if let Some(corner) = self
                .south_east
                .get_texture_id_for_corner(&self.center, &Corner::RightBottom)
            {
                corners.push(corner);
            }
        }

        let mut corner_map: BTreeMap<&TerrainType, Corner> = BTreeMap::new();
        let sides = self.get_higher_sides();
        for (terrain, corner) in sides.iter() {
            let existing_corner = corner_map.get(terrain).unwrap_or(&Corner::Empty);
            corner_map.insert(terrain, *corner + *existing_corner);
        }

        for (terrain, corner) in corner_map {
            if let Some(id) = terrain.get_texture_id_for_corner(&self.center, &corner) {
                result.push(id)
            }
        }

        result.append(&mut corners);
        result
    }
}

fn get_water_texture_id(corner: &Corner) -> Option<u32> {
    match corner {
        Corner::LeftTop => Some(21),
        Corner::RightTop => Some(20),
        Corner::LeftBottom => Some(3),
        Corner::RightBottom => Some(2),
        Corner::Left => Some(24),
        Corner::Top => Some(7),
        Corner::Right => Some(26),
        Corner::Bottom => Some(43),
        Corner::LeftTopL => Some(4),
        Corner::RightTopL => Some(5),
        Corner::LeftBottomL => Some(22),
        Corner::RightBottomL => Some(23),
        Corner::LeftTopAndRightBottom => Some(36),
        Corner::LeftBottomAndRightTop => Some(37),
        Corner::Empty => None,
        Corner::Square => None,
    }
}

fn get_water_special_texture_id(corner: &Corner) -> Option<u32> {
    match corner {
        Corner::LeftTop => Some(30),
        Corner::RightTop => Some(29),
        Corner::LeftBottom => Some(11),
        Corner::RightBottom => Some(10),
        Corner::Left => Some(33),
        Corner::Top => Some(16),
        Corner::Right => Some(35),
        Corner::Bottom => Some(52),
        Corner::LeftTopL => Some(13),
        Corner::RightTopL => Some(14),
        Corner::LeftBottomL => Some(31),
        Corner::RightBottomL => Some(32),
        Corner::LeftTopAndRightBottom => Some(36),
        Corner::LeftBottomAndRightTop => Some(37),
        Corner::Empty => None,
        Corner::Square => None,
    }
}

fn get_variant_texture_id(corner: &Corner) -> Option<u32> {
    match corner {
        Corner::LeftTop => Some(49),
        Corner::RightTop => Some(47),
        Corner::LeftBottom => Some(13),
        Corner::RightBottom => Some(11),
        Corner::Left => Some(32),
        Corner::Top => Some(15),
        Corner::Right => Some(34),
        Corner::Bottom => Some(51),
        Corner::LeftTopL => Some(14),
        Corner::RightTopL => Some(16),
        Corner::LeftBottomL => Some(50),
        Corner::RightBottomL => Some(52),
        Corner::LeftTopAndRightBottom => Some(100),
        Corner::LeftBottomAndRightTop => Some(99),
        Corner::Empty => None,
        Corner::Square => None,
    }
}

fn get_connector_texture_id(corner: &Corner) -> Option<u32> {
    match corner {
        Corner::LeftTop => Some(49),
        Corner::RightTop => Some(47),
        Corner::LeftBottom => Some(13),
        Corner::RightBottom => Some(11),
        Corner::Left => Some(32),
        Corner::Top => Some(15),
        Corner::Right => Some(34),
        Corner::Bottom => Some(51),
        Corner::LeftTopL => Some(27),
        Corner::RightTopL => Some(28),
        Corner::LeftBottomL => Some(45),
        Corner::RightBottomL => Some(46),
        Corner::LeftTopAndRightBottom => Some(9),
        Corner::LeftBottomAndRightTop => Some(10),
        Corner::Empty => None,
        Corner::Square => None,
    }
}
