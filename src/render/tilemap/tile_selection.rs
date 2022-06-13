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
use std::{collections::BTreeMap, fmt::Debug, ops::Add};

use bevy::utils::HashSet;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use super::game::map::TerrainType;

// This is single pieces of Blob indicating that a certain terrain is on
// a certain corner
#[derive(Clone, Debug, PartialEq, Eq, FromPrimitive, Copy, Hash)]
pub enum BlobPiece {
    // Blob 0
    Empty = 0,

    // Blob 1
    North = 1,
    NorthEast = 2,
    East = 4,
    SouthEast = 8,
    South = 16,
    SouthWest = 32,
    West = 64,
    NorthWest = 128,
}
// This is a combination of pieces of blob
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Blob {
    pub corners: HashSet<BlobPiece>,
}

impl Blob {
    pub fn new(corners: Vec<BlobPiece>) -> Blob {
        Blob {
            corners: HashSet::from_iter(corners.into_iter()),
        }
    }

    pub fn add(&mut self, piece: BlobPiece) {
        self.corners.insert(piece);
    }

    pub fn add_blob(&mut self, other: Blob) {
        for corner in other.corners.iter() {
            self.add(*corner);
        }
    }

    pub fn blob_id(&self) -> usize {
        self.corners
            .iter()
            .fold(0, |acc, next| acc + *next as usize)
    }
}

// This is a tile type that we can render uniquely
enum TerrainConnectorType {
    None, // This includes everything we have missed also
    CornerNorthEast,
    CornerSouthEast,
    CornerSouthWest,
    CornerNorthWest,
    EdgeNorth,
    EdgeEast,
    EdgeWest,
    EdgeSouth,
    LNorthEast,
    LSouthEast,
    LSouthWest,
    LNorthWest,
    CornersSouthWestAndNorthEast,
    CornersNorthWestAndSouthEast,
}

impl TerrainConnectorType {
    #[allow(non_snake_case)]
    pub fn from_blob(blob: &Blob) -> TerrainConnectorType {
        let North: usize = BlobPiece::North as usize;
        let NorthEast: usize = BlobPiece::NorthEast as usize;
        let East: usize = BlobPiece::East as usize;
        let SouthEast: usize = BlobPiece::SouthEast as usize;
        let South: usize = BlobPiece::South as usize;
        let SouthWest: usize = BlobPiece::SouthWest as usize;
        let West: usize = BlobPiece::West as usize;
        let NorthWest: usize = BlobPiece::NorthWest as usize;
        let blob_id = blob.blob_id();
        match blob_id {
            b if b == NorthEast => TerrainConnectorType::CornerNorthEast,
            b if b == NorthWest => TerrainConnectorType::CornerNorthWest,
            b if b == SouthEast => TerrainConnectorType::CornerSouthEast,
            b if b == SouthWest => TerrainConnectorType::CornerSouthWest,
            b if [
                North,
                North + NorthEast,
                North + NorthWest,
                NorthWest + NorthEast,
                North + NorthWest + NorthEast,
            ]
            .contains(&b) =>
            {
                TerrainConnectorType::EdgeNorth
            }
            b if [
                East,
                East + SouthEast,
                East + NorthEast,
                NorthEast + SouthEast,
                East + NorthEast + SouthEast,
            ]
            .contains(&b) =>
            {
                TerrainConnectorType::EdgeEast
            }
            b if [
                South,
                South + SouthWest,
                South + SouthEast,
                SouthWest + SouthEast,
                South + SouthWest + SouthEast,
            ]
            .contains(&b) =>
            {
                TerrainConnectorType::EdgeSouth
            }
            b if [
                West,
                West + NorthWest,
                West + SouthWest,
                SouthWest + NorthWest,
                West + NorthWest + SouthWest,
            ]
            .contains(&b) =>
            {
                TerrainConnectorType::EdgeWest
            }
            // // NorthEastCorner
            b if [
                North + East,
                North + East + NorthEast,
                North + East + NorthWest,
                North + East + SouthEast,
                North + East + NorthWest + SouthEast,
                North + East + NorthEast + NorthWest,
                North + East + NorthEast + SouthEast,
                North + East + NorthEast + NorthWest + SouthEast,
            ]
            .contains(&b) =>
            {
                TerrainConnectorType::LNorthEast
            }
            b if [
                South + East,
                South + East + SouthEast,
                South + East + NorthEast,
                South + East + SouthWest,
                South + East + SouthWest + SouthEast,
                South + East + NorthEast + SouthWest,
                South + East + NorthEast + SouthEast,
                South + East + NorthEast + SouthWest + SouthEast,
            ]
            .contains(&b) =>
            {
                TerrainConnectorType::LSouthEast
            }
            b if [
                South + West,
                South + West + SouthWest,
                South + West + NorthWest,
                South + West + SouthEast,
                South + West + NorthWest + SouthEast,
                South + West + SouthWest + NorthWest,
                South + West + SouthWest + SouthEast,
                South + West + SouthWest + NorthWest + SouthEast,
            ]
            .contains(&b) =>
            {
                TerrainConnectorType::LSouthWest
            }
            b if [
                North + West,
                North + West + NorthWest,
                North + West + NorthEast,
                North + West + SouthEast,
                North + West + NorthWest + SouthEast,
                North + West + NorthWest + NorthEast,
                North + West + NorthWest + SouthEast,
                North + West + NorthWest + NorthEast + SouthEast,
            ]
            .contains(&b) =>
            {
                TerrainConnectorType::LNorthWest
            }
            //
            b if [SouthWest + NorthEast].contains(&b) => {
                TerrainConnectorType::CornersSouthWestAndNorthEast
            }
            b if [SouthEast + NorthWest].contains(&b) => {
                TerrainConnectorType::CornersNorthWestAndSouthEast
            }

            b => {
                println!("WARN: Missing tile connector {:?}", b);
                TerrainConnectorType::None
            }
        }
    }
}

trait TerrainDescription {
    fn get_base_texture_id(&self) -> u32;
    fn get_base_variant(&self) -> Self;
    fn get_texture_id_for_corner(&self, center: &TerrainType, corner: &Blob) -> Option<u32>;
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
    fn get_texture_id_for_corner(&self, center: &TerrainType, corner: &Blob) -> Option<u32> {
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

    fn get_higher_sides(&self) -> Vec<(TerrainType, Blob)>;
}

impl TerrainCornersTexture for TerrainCorners {
    fn get_higher_sides(&self) -> Vec<(TerrainType, Blob)> {
        [
            (self.west, Blob::new(vec![BlobPiece::West])),
            (self.east, Blob::new(vec![BlobPiece::East])),
            (self.north, Blob::new(vec![BlobPiece::North])),
            (self.south, Blob::new(vec![BlobPiece::South])),
        ]
        .into_iter()
        .filter(|(terrain, _)| terrain > &self.center)
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
                .get_texture_id_for_corner(&self.center, &Blob::new(vec![BlobPiece::NorthWest]))
            {
                corners.push(corner);
            }
        }

        if (self.north_east > self.center)
            && ((self.north_east > self.north) && (self.north_east > self.east))
        {
            if let Some(corner) = self
                .north_east
                .get_texture_id_for_corner(&self.center, &Blob::new(vec![BlobPiece::NorthEast]))
            {
                corners.push(corner);
            }
        }

        if (self.south_west > self.center)
            && ((self.south_west > self.south) && (self.south_west > self.west))
        {
            if let Some(corner) = self
                .south_west
                .get_texture_id_for_corner(&self.center, &Blob::new(vec![BlobPiece::SouthWest]))
            {
                corners.push(corner);
            }
        }

        if (self.south_east > self.center)
            && ((self.south_east > self.south) && (self.south_east > self.east))
        {
            if let Some(corner) = self
                .south_east
                .get_texture_id_for_corner(&self.center, &Blob::new(vec![BlobPiece::SouthEast]))
            {
                corners.push(corner);
            }
        }

        let mut corner_map: BTreeMap<&TerrainType, Blob> = BTreeMap::new();
        let sides = self.get_higher_sides();
        for (terrain, corner) in sides.iter() {
            corner_map
                .entry(terrain)
                .and_modify(|blob| {
                    blob.add_blob(corner.clone());
                })
                .or_insert_with(|| corner.clone());
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

fn get_water_texture_id(blob: &Blob) -> Option<u32> {
    match TerrainConnectorType::from_blob(blob) {
        TerrainConnectorType::CornerNorthEast => Some(20),
        TerrainConnectorType::CornerSouthEast => Some(2),
        TerrainConnectorType::CornerSouthWest => Some(3),
        TerrainConnectorType::CornerNorthWest => Some(21),

        TerrainConnectorType::EdgeNorth => Some(7),
        TerrainConnectorType::EdgeEast => Some(26),
        TerrainConnectorType::EdgeWest => Some(24),
        TerrainConnectorType::EdgeSouth => Some(43),

        TerrainConnectorType::LNorthEast => Some(5),
        TerrainConnectorType::LSouthWest => Some(22),
        TerrainConnectorType::LSouthEast => Some(23),
        TerrainConnectorType::LNorthWest => Some(4),

        TerrainConnectorType::CornersSouthWestAndNorthEast => Some(37),
        TerrainConnectorType::CornersNorthWestAndSouthEast => Some(36),

        _ => None,
    }
}

fn get_water_special_texture_id(blob: &Blob) -> Option<u32> {
    match TerrainConnectorType::from_blob(blob) {
        TerrainConnectorType::CornerNorthEast => Some(20),
        TerrainConnectorType::CornerSouthEast => Some(10),
        TerrainConnectorType::CornerSouthWest => Some(11),
        TerrainConnectorType::CornerNorthWest => Some(30),

        TerrainConnectorType::EdgeNorth => Some(16),
        TerrainConnectorType::EdgeEast => Some(35),
        TerrainConnectorType::EdgeWest => Some(33),
        TerrainConnectorType::EdgeSouth => Some(52),

        TerrainConnectorType::LNorthEast => Some(14),
        TerrainConnectorType::LSouthWest => Some(31),
        TerrainConnectorType::LSouthEast => Some(32),
        TerrainConnectorType::LNorthWest => Some(13),

        TerrainConnectorType::CornersSouthWestAndNorthEast => Some(37),
        TerrainConnectorType::CornersNorthWestAndSouthEast => Some(36),

        _ => None,
    }
}

fn get_variant_texture_id(blob: &Blob) -> Option<u32> {
    match TerrainConnectorType::from_blob(blob) {
        // TerrainConnectorType::CornerNorthEast => Some(47),
        // TerrainConnectorType::CornerSouthEast => Some(11),
        // TerrainConnectorType::CornerSouthWest => Some(13),
        // TerrainConnectorType::CornerNorthWest => Some(49),

        // TerrainConnectorType::EdgeNorth => Some(15),
        // TerrainConnectorType::EdgeEast => Some(34),
        // TerrainConnectorType::EdgeWest => Some(32),
        // TerrainConnectorType::EdgeSouth => Some(51),
        TerrainConnectorType::LNorthEast => Some(16),
        TerrainConnectorType::LSouthWest => Some(50),
        TerrainConnectorType::LSouthEast => Some(52),
        TerrainConnectorType::LNorthWest => Some(14),

        // TerrainConnectorType::CornersSouthWestAndNorthEast => Some(99),
        // TerrainConnectorType::CornersNorthWestAndSouthEast => Some(100),
        _ => None,
    }
}

fn get_connector_texture_id(blob: &Blob) -> Option<u32> {
    match TerrainConnectorType::from_blob(blob) {
        TerrainConnectorType::CornerNorthEast => Some(47),
        TerrainConnectorType::CornerSouthEast => Some(11),
        TerrainConnectorType::CornerSouthWest => Some(13),
        TerrainConnectorType::CornerNorthWest => Some(49),

        TerrainConnectorType::EdgeNorth => Some(15),
        TerrainConnectorType::EdgeEast => Some(34),
        TerrainConnectorType::EdgeWest => Some(32),
        TerrainConnectorType::EdgeSouth => Some(51),

        TerrainConnectorType::LNorthEast => Some(28),
        TerrainConnectorType::LSouthWest => Some(45),
        TerrainConnectorType::LSouthEast => Some(46),
        TerrainConnectorType::LNorthWest => Some(27),

        TerrainConnectorType::CornersSouthWestAndNorthEast => Some(10),
        TerrainConnectorType::CornersNorthWestAndSouthEast => Some(9),

        _ => None,
    }
}
