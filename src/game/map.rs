use std::hash::{Hash, Hasher};

use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct GameWorld {
    pub width: u32,
    pub height: u32,
}

#[derive(Component, Debug, Clone)]
pub struct Province {
    pub name: &'static str,
}

#[derive(Component, Debug, Clone)]
pub struct TerrainProvince {
    pub province: u32,
}

#[derive(Component, Debug, Clone)]
pub struct ProvinceBorder {
    pub color: Color,
}

#[derive(Component, Debug, Clone)]
pub struct TerrainBase {
    pub terrain_type: TerrainType,
}

#[derive(Component, Debug, Clone)]
pub struct TerrainPosition {
    pub x: u32,
    pub y: u32,
}

impl PartialEq for TerrainPosition {
    fn eq(&self, other: &TerrainPosition) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for TerrainPosition {}

impl Hash for TerrainPosition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct TerrainBundle {
    pub province: TerrainProvince,
    pub position: TerrainPosition,
    pub base: TerrainBase,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Copy)]
pub enum TerrainType {
    Sea,
    Ocean,
    Dryland,
    DrylandCracked,
    Grass,
    Pasture,
    SnowBlue,
    SnowBlueDune,
    Lava,
    LavaCracks,
    Swamp,
    WaterSwamp,
    Sand,
    SandDune,
    Snow,
    SnowDune,
    Dirt,
    Moss,
    Wasteland,
    WastelandCracked,
    Bog,
    Reeds,
    Ice,
}

impl PartialEq for TerrainType {
    fn eq(&self, other: &TerrainType) -> bool {
        get_terrain_ordering(self) == get_terrain_ordering(other)
    }
}

impl Eq for TerrainType {}

impl PartialOrd for TerrainType {
    fn partial_cmp(&self, other: &TerrainType) -> Option<std::cmp::Ordering> {
        Some(get_terrain_ordering(self).cmp(&get_terrain_ordering(other)))
    }
}

impl Ord for TerrainType {
    fn cmp(&self, other: &TerrainType) -> std::cmp::Ordering {
        get_terrain_ordering(self).cmp(&get_terrain_ordering(other))
    }
}

impl Hash for TerrainType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        get_terrain_ordering(self).hash(state)
    }
}

fn get_terrain_ordering(terrain_type: &TerrainType) -> u32 {
    match terrain_type {
        TerrainType::Sea => 0,
        TerrainType::Ocean => 1,
        TerrainType::Ice => 2,
        TerrainType::Dryland => 3,
        TerrainType::DrylandCracked => 4,
        TerrainType::SnowBlue => 5,
        TerrainType::SnowBlueDune => 6,
        TerrainType::Lava => 7,
        TerrainType::LavaCracks => 8,
        TerrainType::Swamp => 9,
        TerrainType::WaterSwamp => 10,
        TerrainType::Sand => 11,
        TerrainType::SandDune => 12,
        TerrainType::Snow => 13,
        TerrainType::SnowDune => 14,
        TerrainType::Dirt => 15,
        TerrainType::Moss => 16,
        TerrainType::Wasteland => 17,
        TerrainType::WastelandCracked => 18,
        TerrainType::Bog => 19,
        TerrainType::Reeds => 20,
        TerrainType::Grass => 21,
        TerrainType::Pasture => 22,
    }
}
