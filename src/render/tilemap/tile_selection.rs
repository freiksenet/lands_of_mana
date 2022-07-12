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
use std::{collections::BTreeMap, fmt::Debug};

use bevy::utils::HashSet;
use num_derive::FromPrimitive;

use super::game::map::TerrainType;
use crate::game::map::{ForestType, MountainType, RoadType, TerrainTop};

// This is single pieces of a tile indicating that a certain terrain is on
// a certain corner / edge
#[derive(Clone, Debug, PartialEq, Eq, FromPrimitive, Copy, Hash)]
pub enum TilePiece {
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
// This is a combination of all corners of a tile
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tile {
    pub corners: HashSet<TilePiece>,
}

impl Tile {
    pub fn new(corners: Vec<TilePiece>) -> Tile {
        Tile {
            corners: HashSet::from_iter(corners.into_iter()),
        }
    }

    pub fn add(&mut self, piece: TilePiece) {
        self.corners.insert(piece);
    }

    pub fn add_tile(&mut self, other: Tile) {
        for corner in other.corners.iter() {
            self.add(*corner);
        }
    }

    pub fn is_at_least(&self, other: &Tile) -> bool {
        self.corners.is_superset(&other.corners)
    }

    pub fn is_at_least_one_of(&self, others: &[Tile]) -> bool {
        others.iter().any(|tile| self.is_at_least(tile))
    }

    pub fn is_exactly(&self, other: &Tile) -> bool {
        self.is_at_least(other) && other.is_at_least(self)
    }

    pub fn is_exactly_one_of(&self, others: &[Tile]) -> bool {
        others.iter().any(|tile| self.is_exactly(tile))
    }

    fn into_id(corners: &HashSet<TilePiece>) -> usize {
        corners.iter().fold(0, |acc, next| acc + *next as usize)
    }

    pub fn id(&self) -> usize {
        Self::into_id(&self.corners)
    }

    // Only output valid blob tile id
    pub fn blob_id(&self) -> usize {
        let mut new_corners: HashSet<TilePiece> = self.corners.iter().copied().collect();
        if new_corners.contains(&TilePiece::NorthEast) {
            new_corners.insert(TilePiece::North);
            new_corners.insert(TilePiece::East);
        }
        if new_corners.contains(&TilePiece::SouthEast) {
            new_corners.insert(TilePiece::South);
            new_corners.insert(TilePiece::East);
        }
        if new_corners.contains(&TilePiece::SouthWest) {
            new_corners.insert(TilePiece::South);
            new_corners.insert(TilePiece::West);
        }
        if new_corners.contains(&TilePiece::NorthWest) {
            new_corners.insert(TilePiece::North);
            new_corners.insert(TilePiece::West);
        }

        Self::into_id(&new_corners)
    }

    pub fn no_corners_id(&self) -> usize {
        self.no_corners_tile().id()
    }

    pub fn no_corners_tile(&self) -> Tile {
        let mut new_corners: HashSet<TilePiece> = (&self.corners).iter().copied().collect();
        new_corners.remove(&TilePiece::NorthEast);
        new_corners.remove(&TilePiece::SouthEast);
        new_corners.remove(&TilePiece::SouthWest);
        new_corners.remove(&TilePiece::NorthWest);
        Tile {
            corners: new_corners,
        }
    }

    fn blob_type_from_id(id: usize) -> BlobType {
        match id {
            // * * *
            // * ? *
            // * * *
            0 => BlobType::Empty,

            // * # *
            // * ? *
            // * * *
            1 => BlobType::Top(BlobTopDirection::North),
            4 => BlobType::Top(BlobTopDirection::East),
            16 => BlobType::Top(BlobTopDirection::South),
            64 => BlobType::Top(BlobTopDirection::West),

            // * # *
            // * ? #
            // * * *
            5 => BlobType::TopRight(BlobTopDirection::North),
            20 => BlobType::TopRight(BlobTopDirection::East),
            80 => BlobType::TopRight(BlobTopDirection::South),
            65 => BlobType::TopRight(BlobTopDirection::West),

            // * # #
            // * ? #
            // * * *
            7 => BlobType::TopRightCTopRight(BlobTopDirection::North),
            28 => BlobType::TopRightCTopRight(BlobTopDirection::East),
            112 => BlobType::TopRightCTopRight(BlobTopDirection::South),
            193 => BlobType::TopRightCTopRight(BlobTopDirection::West),

            // * # *
            // * ? *
            // * # *
            17 => BlobType::TopBottom(BlobTopDirectionTwo::North),
            68 => BlobType::TopBottom(BlobTopDirectionTwo::East),

            // * # *
            // * ? #
            // * # *
            21 => BlobType::TopRightBottom(BlobTopDirection::North),
            84 => BlobType::TopRightBottom(BlobTopDirection::East),
            81 => BlobType::TopRightBottom(BlobTopDirection::South),
            69 => BlobType::TopRightBottom(BlobTopDirection::West),

            // * # #
            // * ? #
            // * # *
            23 => BlobType::TopRightBottomCTopRight(BlobTopDirection::North),
            92 => BlobType::TopRightBottomCTopRight(BlobTopDirection::East),
            113 => BlobType::TopRightBottomCTopRight(BlobTopDirection::South),
            197 => BlobType::TopRightBottomCTopRight(BlobTopDirection::West),

            // * # *
            // * ? #
            // * # #
            29 => BlobType::TopRightBottomCBottomRight(BlobTopDirection::North),
            116 => BlobType::TopRightBottomCBottomRight(BlobTopDirection::East),
            209 => BlobType::TopRightBottomCBottomRight(BlobTopDirection::South),
            71 => BlobType::TopRightBottomCBottomRight(BlobTopDirection::West),

            // * # #
            // * ? #
            // * # #
            31 => BlobType::TopRightBottomCTopRightCBottomRight(BlobTopDirection::North),
            124 => BlobType::TopRightBottomCTopRightCBottomRight(BlobTopDirection::East),
            241 => BlobType::TopRightBottomCTopRightCBottomRight(BlobTopDirection::South),
            199 => BlobType::TopRightBottomCTopRightCBottomRight(BlobTopDirection::West),

            // * # *
            // # ? #
            // * # *
            85 => BlobType::TopRightBottomLeft,

            // * # #
            // # ? #
            // * # *
            87 => BlobType::TopRightBottomLeftCTopRight(BlobTopDirection::North),
            93 => BlobType::TopRightBottomLeftCTopRight(BlobTopDirection::East),
            117 => BlobType::TopRightBottomLeftCTopRight(BlobTopDirection::South),
            213 => BlobType::TopRightBottomLeftCTopRight(BlobTopDirection::West),

            // * # #
            // # ? #
            // * # #
            95 => BlobType::TopRightBottomLeftCTopRightCBottomRight(BlobTopDirection::North),
            125 => BlobType::TopRightBottomLeftCTopRightCBottomRight(BlobTopDirection::East),
            245 => BlobType::TopRightBottomLeftCTopRightCBottomRight(BlobTopDirection::South),
            214 => BlobType::TopRightBottomLeftCTopRightCBottomRight(BlobTopDirection::West),

            // * # #
            // # ? #
            // # # *
            119 => BlobType::TopRightBottomLeftCTopRightCBottomLeft(BlobTopDirectionTwo::North),
            221 => BlobType::TopRightBottomLeftCTopRightCBottomLeft(BlobTopDirectionTwo::East),

            // * # *
            // # ? #
            // # # #
            127 => BlobType::TopRightBottomLeftCTopRightCBottomRightCBottomLeft(
                BlobTopDirection::North,
            ),
            253 => {
                BlobType::TopRightBottomLeftCTopRightCBottomRightCBottomLeft(BlobTopDirection::East)
            }
            247 => BlobType::TopRightBottomLeftCTopRightCBottomRightCBottomLeft(
                BlobTopDirection::South,
            ),
            223 => {
                BlobType::TopRightBottomLeftCTopRightCBottomRightCBottomLeft(BlobTopDirection::West)
            }

            // # # #
            // # ? #
            // # # #
            255 => BlobType::Full,

            b => panic!("Invalid blob: {:?}", b),
        }
    }

    pub fn blob_type(&self) -> BlobType {
        Self::blob_type_from_id(self.blob_id())
    }

    // blob type ignoring corners
    pub fn edge_blob_type(&self) -> BlobType {
        Self::blob_type_from_id(self.no_corners_id())
    }
}

#[derive(Debug)]
pub enum BlobTopDirection {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
pub enum BlobTopDirectionTwo {
    North,
    East,
}

#[derive(Debug)]
pub enum BlobType {
    Empty,
    Top(BlobTopDirection),
    TopRight(BlobTopDirection),
    TopRightCTopRight(BlobTopDirection),
    TopBottom(BlobTopDirectionTwo),
    TopRightBottom(BlobTopDirection),
    TopRightBottomCTopRight(BlobTopDirection),
    TopRightBottomCBottomRight(BlobTopDirection),
    TopRightBottomCTopRightCBottomRight(BlobTopDirection),
    TopRightBottomLeft,
    TopRightBottomLeftCTopRight(BlobTopDirection),
    TopRightBottomLeftCTopRightCBottomRight(BlobTopDirection),
    TopRightBottomLeftCTopRightCBottomLeft(BlobTopDirectionTwo),
    TopRightBottomLeftCTopRightCBottomRightCBottomLeft(BlobTopDirection),
    Full,
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
    pub fn from_tile(tile: &Tile) -> TerrainConnectorType {
        let North: usize = TilePiece::North as usize;
        let NorthEast: usize = TilePiece::NorthEast as usize;
        let East: usize = TilePiece::East as usize;
        let SouthEast: usize = TilePiece::SouthEast as usize;
        let South: usize = TilePiece::South as usize;
        let SouthWest: usize = TilePiece::SouthWest as usize;
        let West: usize = TilePiece::West as usize;
        let NorthWest: usize = TilePiece::NorthWest as usize;
        match tile.id() {
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

const CONNECTOR_TERRAIN_SIZE: u32 = 54;

impl TerrainType {
    pub fn get_base_texture_id(&self) -> u32 {
        *self as u32
    }

    pub fn get_base_variant(&self) -> Self {
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
    pub fn get_texture_id_for_corner(
        &self,
        center: &TerrainType,
        tile: &Tile,
        river_tile: &Tile,
    ) -> Option<u32> {
        let terrain = *self;
        let base_variant = terrain.get_base_variant();
        let base_center_variant = center.get_base_variant();
        let base_offset = self.get_base_texture_id() * CONNECTOR_TERRAIN_SIZE;
        let base_variant_offset = base_variant.get_base_texture_id() * CONNECTOR_TERRAIN_SIZE;
        let center_offset = center.get_base_texture_id() * CONNECTOR_TERRAIN_SIZE;

        // check if terrain is a variant of a base, use connector tiles
        if base_variant == base_center_variant {
            return get_variant_texture_id(tile).map(|id| id + base_offset);
        }
        // check if terrain is one of the sea types, here some seas (Ice, Swamp) are exclusive to their parent tiles
        // and will return None
        else if base_variant == TerrainType::Swamp
            && base_center_variant == TerrainType::WaterSwamp
        {
            if terrain == TerrainType::Swamp || terrain == TerrainType::SwampBog {
                return get_water_texture_id(tile, river_tile).map(|id| id + center_offset);
            } else if terrain == TerrainType::SwampReeds {
                return get_water_special_texture_id(river_tile).map(|id| id + center_offset);
            }
        } else if base_variant == TerrainType::Snow && base_center_variant == TerrainType::Ice {
            if terrain == TerrainType::Snow {
                return get_water_texture_id(tile, river_tile).map(|id| id + center_offset);
            } else if terrain == TerrainType::SnowDune {
                return get_water_special_texture_id(tile).map(|id| id + center_offset);
            }
        }
        // check if terrain is water
        else if base_center_variant == TerrainType::Water {
            return get_water_texture_id(tile, river_tile).map(|id| id + base_offset);
        }
        // normal connectors
        else if base_center_variant != TerrainType::WaterSwamp
            && base_center_variant != TerrainType::Ice
        {
            return get_connector_texture_id(tile).map(|id| id + base_variant_offset);
        }

        None
    }
}

pub struct TerrainCorners {
    pub center: (TerrainType, TerrainTop),
    pub north: (TerrainType, TerrainTop),
    pub south: (TerrainType, TerrainTop),
    pub west: (TerrainType, TerrainTop),
    pub east: (TerrainType, TerrainTop),
    pub north_west: (TerrainType, TerrainTop),
    pub north_east: (TerrainType, TerrainTop),
    pub south_west: (TerrainType, TerrainTop),
    pub south_east: (TerrainType, TerrainTop),
}

impl TerrainCorners {
    fn get_higher_sides(&self) -> Vec<(TerrainType, Tile)> {
        [
            (self.west.0, Tile::new(vec![TilePiece::West])),
            (self.east.0, Tile::new(vec![TilePiece::East])),
            (self.north.0, Tile::new(vec![TilePiece::North])),
            (self.south.0, Tile::new(vec![TilePiece::South])),
        ]
        .into_iter()
        .filter(|(terrain, _)| terrain > &self.center.0)
        .collect()
    }

    fn get_river_blob(&self) -> Tile {
        self.get_tile_by(TileKind::Side, |(_, top)| {
            matches!(top, TerrainTop::River | TerrainTop::RiverWithBridge(_))
        })
    }

    pub fn get_tile_textures(&self) -> Vec<u32> {
        let mut result: Vec<u32> = Vec::new();
        let river_blob = self.get_river_blob();
        let empty_blob = Tile::new(vec![]);
        result.push(self.center.0.get_base_texture_id());
        // Figure out if we need to draw corners
        let mut corners: Vec<u32> = Vec::new();

        if (self.north_west.0 > self.center.0)
            && ((self.north_west.0 > self.north.0) && (self.north_west.0 > self.west.0))
        {
            if let Some(corner) = self.north_west.0.get_texture_id_for_corner(
                &self.center.0,
                &Tile::new(vec![TilePiece::NorthWest]),
                &empty_blob,
            ) {
                corners.push(corner);
            }
        }

        if (self.north_east.0 > self.center.0)
            && ((self.north_east.0 > self.north.0) && (self.north_east.0 > self.east.0))
        {
            if let Some(corner) = self.north_east.0.get_texture_id_for_corner(
                &self.center.0,
                &Tile::new(vec![TilePiece::NorthEast]),
                &empty_blob,
            ) {
                corners.push(corner);
            }
        }

        if (self.south_west.0 > self.center.0)
            && ((self.south_west.0 > self.south.0) && (self.south_west.0 > self.west.0))
        {
            if let Some(corner) = self.south_west.0.get_texture_id_for_corner(
                &self.center.0,
                &Tile::new(vec![TilePiece::SouthWest]),
                &empty_blob,
            ) {
                corners.push(corner);
            }
        }

        if (self.south_east.0 > self.center.0)
            && ((self.south_east.0 > self.south.0) && (self.south_east.0 > self.east.0))
        {
            if let Some(corner) = self.south_east.0.get_texture_id_for_corner(
                &self.center.0,
                &Tile::new(vec![TilePiece::SouthEast]),
                &empty_blob,
            ) {
                corners.push(corner);
            }
        }

        let mut corner_map: BTreeMap<&TerrainType, Tile> = BTreeMap::new();
        let sides = self.get_higher_sides();
        for (terrain, corner) in sides.iter() {
            corner_map
                .entry(terrain)
                .and_modify(|tile| {
                    tile.add_tile(corner.clone());
                })
                .or_insert_with(|| corner.clone());
        }

        for (terrain, corner) in corner_map {
            if let Some(id) =
                terrain.get_texture_id_for_corner(&self.center.0, &corner, &river_blob)
            {
                result.push(id)
            }
        }

        result.append(&mut corners);
        result
    }

    pub fn get_road_texture(&self) -> Option<u32> {
        let blob = self.get_tile_by(TileKind::Side, |(_, top)| {
            matches!(top, TerrainTop::Road(_) | TerrainTop::RiverWithBridge(_))
        });
        match self.center.1 {
            TerrainTop::Road(road_type) => {
                Some(get_base_road_texture_id(road_type) + get_road_texture_id(&blob))
            }
            TerrainTop::RiverWithBridge(road_type) => Some(get_bridge_texture_id(road_type, &blob)),
            _ => None,
        }
    }

    pub fn get_river_texture(&self) -> Option<u32> {
        let blob = self.get_river_blob();
        match self.center.1 {
            TerrainTop::River | TerrainTop::RiverWithBridge(_) => Some(get_river_texture_id(&blob)),
            _ if matches!(
                self.center.0,
                TerrainType::Water | TerrainType::WaterOcean | TerrainType::WaterSwamp
            ) =>
            {
                if self.north.1.is_river() {
                    return Some(21);
                } else if self.south.1.is_river() {
                    return Some(4);
                } else if self.west.1.is_river() {
                    return Some(5);
                } else if self.east.1.is_river() {
                    return Some(20);
                }
                None
            }
            _ => None,
        }
    }

    fn get_forest_tile(&self) -> Option<(Tile, ForestType)> {
        let tiles = self.get_full_tile();
        tiles
            .into_iter()
            .map(|((_, top), piece)| {
                if let TerrainTop::Forest(forest_type) = top {
                    Some((Tile::new(vec![piece]), forest_type))
                } else {
                    None
                }
            })
            .fold(None, |acc, next| match (acc, next) {
                (None, None) => None,
                (None, s @ Some(_)) => s,
                (s @ Some(_), None) => s,
                (Some((mut tile, forest_type)), Some((other_tile, _))) => {
                    tile.add_tile(other_tile);
                    Some((tile, forest_type))
                }
            })
    }

    fn get_mountain_tile(&self) -> Option<(Tile, MountainType)> {
        let tiles = self.get_full_tile();
        tiles
            .into_iter()
            .map(|((_, top), piece)| {
                if let TerrainTop::Mountain(mountain_type) = top {
                    Some((Tile::new(vec![piece]), mountain_type))
                } else {
                    None
                }
            })
            .fold(None, |acc, next| match (acc, next) {
                (None, None) => None,
                (None, s @ Some(_)) => s,
                (s @ Some(_), None) => s,
                (Some((mut tile, mountain_type)), Some((other_tile, _))) => {
                    tile.add_tile(other_tile);
                    Some((tile, mountain_type))
                }
            })
    }

    pub fn get_forest_texture(&self) -> Option<u32> {
        let forest_blob = self.get_forest_tile();
        match (self.center.1, forest_blob) {
            (TerrainTop::Forest(forest_type), _) => {
                Some(get_base_forest_texture_id(forest_type) + 55)
            }
            (_, Some((tile, forest_type))) => Some(
                get_base_forest_texture_id(forest_type) + get_forest_or_mountain_texture_id(&tile),
            ),
            _ => None,
        }
    }

    pub fn get_mountain_texture(&self) -> Option<u32> {
        let mountain_blob = self.get_mountain_tile();
        match (self.center.1, mountain_blob) {
            (TerrainTop::Mountain(mountain_type), _) => {
                Some(get_base_mountain_texture_id(mountain_type) + 55)
            }
            (_, Some((tile, mountain_type))) => Some(
                get_base_mountain_texture_id(mountain_type)
                    + get_forest_or_mountain_texture_id(&tile),
            ),
            _ => None,
        }
    }

    fn get_corners(&self) -> Vec<((TerrainType, TerrainTop), TilePiece)> {
        vec![
            (self.north_west, TilePiece::NorthWest),
            (self.north_east, TilePiece::NorthEast),
            (self.south_east, TilePiece::SouthEast),
            (self.south_west, TilePiece::SouthWest),
        ]
    }

    fn get_sides(&self) -> Vec<((TerrainType, TerrainTop), TilePiece)> {
        vec![
            (self.north, TilePiece::North),
            (self.east, TilePiece::East),
            (self.south, TilePiece::South),
            (self.west, TilePiece::West),
        ]
    }

    fn get_full_tile(&self) -> Vec<((TerrainType, TerrainTop), TilePiece)> {
        let mut corners = self.get_corners();
        let mut sides = self.get_sides();
        corners.append(&mut sides);
        corners
    }

    pub fn get_tile_by(
        &self,
        kind: TileKind,
        predicate: fn(&(TerrainType, TerrainTop)) -> bool,
    ) -> Tile {
        let pieces = match kind {
            TileKind::Corner => self.get_corners(),
            TileKind::Side => self.get_sides(),
            TileKind::Full => self.get_full_tile(),
        };
        Tile::new(
            pieces
                .into_iter()
                .filter(|(p, _)| predicate(p))
                .map(|i| i.1)
                .collect(),
        )
    }
}

pub enum TileKind {
    Corner,
    Side,
    Full,
}

fn get_water_texture_id(tile: &Tile, river_tile: &Tile) -> Option<u32> {
    match TerrainConnectorType::from_tile(tile) {
        TerrainConnectorType::CornerNorthEast => Some(20),
        TerrainConnectorType::CornerSouthEast => Some(2),
        TerrainConnectorType::CornerSouthWest => Some(3),
        TerrainConnectorType::CornerNorthWest => Some(21),

        TerrainConnectorType::EdgeNorth if river_tile.id() != TilePiece::North as usize => Some(7),
        TerrainConnectorType::EdgeEast if river_tile.id() != TilePiece::East as usize => Some(26),
        TerrainConnectorType::EdgeWest if river_tile.id() != TilePiece::West as usize => Some(24),
        TerrainConnectorType::EdgeSouth if river_tile.id() != TilePiece::South as usize => Some(43),

        TerrainConnectorType::LNorthEast => Some(5),
        TerrainConnectorType::LSouthWest => Some(22),
        TerrainConnectorType::LSouthEast => Some(23),
        TerrainConnectorType::LNorthWest => Some(4),

        TerrainConnectorType::CornersSouthWestAndNorthEast => Some(37),
        TerrainConnectorType::CornersNorthWestAndSouthEast => Some(36),

        _ => None,
    }
}

fn get_water_special_texture_id(tile: &Tile) -> Option<u32> {
    match TerrainConnectorType::from_tile(tile) {
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

fn get_variant_texture_id(tile: &Tile) -> Option<u32> {
    match TerrainConnectorType::from_tile(tile) {
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

fn get_connector_texture_id(tile: &Tile) -> Option<u32> {
    match TerrainConnectorType::from_tile(tile) {
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

pub fn get_base_road_texture_id(road_type: RoadType) -> u32 {
    match road_type {
        RoadType::Path => 32,
        RoadType::BrownCobblestone => 40,
        RoadType::BlueCobblestone => 112,
        RoadType::Bricks => 120,
    }
}

pub fn get_bridge_texture_id(road_type: RoadType, tile: &Tile) -> u32 {
    (match road_type {
        RoadType::Path => 24,
        RoadType::BrownCobblestone => 26,
        RoadType::BlueCobblestone => 26,
        RoadType::Bricks => 26,
    }) + match tile.blob_type() {
        BlobType::Top(BlobTopDirection::North)
        | BlobType::Top(BlobTopDirection::South)
        | BlobType::TopBottom(BlobTopDirectionTwo::North) => 1,
        _ => 0,
    }
}

pub fn get_road_texture_id(tile: &Tile) -> u32 {
    match tile.edge_blob_type() {
        BlobType::Top(BlobTopDirection::North) => 49,
        BlobType::Top(BlobTopDirection::South) => 17,
        BlobType::Top(BlobTopDirection::West) => 4,
        BlobType::Top(BlobTopDirection::East) => 2,
        BlobType::TopBottom(BlobTopDirectionTwo::North) => 33,
        BlobType::TopBottom(BlobTopDirectionTwo::East) => 3,
        BlobType::TopRight(BlobTopDirection::North) => 35,
        BlobType::TopRight(BlobTopDirection::West) => 36,
        BlobType::TopRight(BlobTopDirection::East) => 19,
        BlobType::TopRight(BlobTopDirection::South) => 20,
        BlobType::TopRightBottom(BlobTopDirection::North) => 32,
        BlobType::TopRightBottom(BlobTopDirection::East) => 34,
        BlobType::TopRightBottom(BlobTopDirection::South) => 48,
        BlobType::TopRightBottom(BlobTopDirection::West) => 18,
        BlobType::TopRightBottomLeft => 16,
        _ => 64,
    }
}

pub fn get_river_texture_id(tile: &Tile) -> u32 {
    match tile.edge_blob_type() {
        BlobType::Top(BlobTopDirection::North) => 16,
        BlobType::Top(BlobTopDirection::South) => 16,
        BlobType::Top(BlobTopDirection::West) => 1,
        BlobType::Top(BlobTopDirection::East) => 1,
        BlobType::TopBottom(BlobTopDirectionTwo::North) => 16,
        BlobType::TopBottom(BlobTopDirectionTwo::East) => 1,
        BlobType::TopRight(BlobTopDirection::North) => 18,
        BlobType::TopRight(BlobTopDirection::East) => 2,
        BlobType::TopRight(BlobTopDirection::South) => 3,
        BlobType::TopRight(BlobTopDirection::West) => 19,
        BlobType::TopRightBottom(BlobTopDirection::North) => 22,
        BlobType::TopRightBottom(BlobTopDirection::East) => 6,
        BlobType::TopRightBottom(BlobTopDirection::South) => 7,
        BlobType::TopRightBottom(BlobTopDirection::West) => 23,
        _ => 1,
    }
}

pub fn get_base_forest_texture_id(forest_type: ForestType) -> u32 {
    match forest_type {
        ForestType::Beech => 156,
        ForestType::Pine => 312,
        ForestType::Spruce => 624,
        ForestType::Oak => 1092,
    }
}

pub fn get_base_mountain_texture_id(mountain_type: MountainType) -> u32 {
    match mountain_type {
        MountainType::Dirt => 1248,
        MountainType::Sand => 1261,
        MountainType::Rock => 1274,
        MountainType::RockIceCapped => 1287,
    }
}

pub fn get_forest_or_mountain_texture_id(tile: &Tile) -> u32 {
    match (tile, tile.no_corners_tile()) {
        (t, _) if t.is_exactly(&Tile::new(vec![TilePiece::NorthEast])) => 111,
        (t, _) if t.is_exactly(&Tile::new(vec![TilePiece::SouthEast])) => 7,
        (t, _) if t.is_exactly(&Tile::new(vec![TilePiece::SouthWest])) => 9,
        (t, _) if t.is_exactly(&Tile::new(vec![TilePiece::NorthWest])) => 113,

        (t, nc)
            if t.is_exactly(&Tile::new(vec![TilePiece::NorthWest, TilePiece::NorthEast]))
                || nc.is_exactly(&Tile::new(vec![TilePiece::North])) =>
        {
            115
        }
        (t, nc)
            if t.is_exactly(&Tile::new(vec![TilePiece::NorthEast, TilePiece::SouthEast]))
                || nc.is_exactly(&Tile::new(vec![TilePiece::East])) =>
        {
            62
        }
        (t, nc)
            if t.is_exactly(&Tile::new(vec![TilePiece::SouthEast, TilePiece::SouthWest]))
                || nc.is_exactly(&Tile::new(vec![TilePiece::South])) =>
        {
            11
        }
        (t, nc)
            if t.is_exactly(&Tile::new(vec![TilePiece::SouthWest, TilePiece::NorthWest]))
                || nc.is_exactly(&Tile::new(vec![TilePiece::West])) =>
        {
            64
        }

        (t, _) if t.is_exactly(&Tile::new(vec![TilePiece::NorthEast, TilePiece::SouthWest])) => 109,
        (t, _) if t.is_exactly(&Tile::new(vec![TilePiece::NorthWest, TilePiece::SouthEast])) => 110,

        (_, t) if t.is_exactly(&Tile::new(vec![TilePiece::North, TilePiece::East])) => 53,

        (_, t) if t.is_exactly(&Tile::new(vec![TilePiece::East, TilePiece::South])) => 105,

        (_, t) if t.is_exactly(&Tile::new(vec![TilePiece::South, TilePiece::West])) => 104,
        (_, t) if t.is_exactly(&Tile::new(vec![TilePiece::West, TilePiece::North])) => 52,

        b => {
            println!("{:?}", b);
            60
        }
    }
}
