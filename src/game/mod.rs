use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::state;

pub mod map;
pub mod units;

pub fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(map::GameWorld {
            width: 64,
            height: 32,
        })
        .with_children(|world| {
            for p_x in 0..4 {
                for p_y in 0..2 {
                    build_province(world.spawn(), p_x, p_y, 4, 2);
                }
            }
        });
    commands.insert_resource(NextState(state::GameState::LoadingGraphics));
}

fn build_province(mut entity: EntityCommands, p_x: u32, p_y: u32, p_xmax: u32, p_ymax: u32) {
    entity
        .insert(map::Province {
            name: "Condoria".to_string(),
        })
        .with_children(|province| {
            for x in 0..16 {
                for y in 0..16 {
                    let province_entity = province.parent_entity();

                    let mut terrain = province.spawn();
                    terrain.insert_bundle(map::TerrainBundle {
                        province: map::ProvinceId(province_entity),
                        position: map::Position {
                            x: x + (p_x * 16),
                            y: y + (p_y * 16),
                        },
                        base: map::TerrainBase {
                            terrain_type: match (p_x, p_y) {
                                (0, 0) => {
                                    if x > 0 && y > 0 && y < 15 && x < 15 {
                                        map::TerrainType::GrassLandPasture
                                    } else {
                                        map::TerrainType::GrassLand
                                    }
                                }
                                (0, 1) => map::TerrainType::Water,
                                (1, 0) => map::TerrainType::Swamp,
                                (1, 1) => map::TerrainType::Lava,
                                (2, 0) => map::TerrainType::DesertRed,
                                (2, 1) => map::TerrainType::Desert,
                                (3, 0) => map::TerrainType::Swamp,
                                (3, 1) => map::TerrainType::Snow,

                                _ => map::TerrainType::Desert,
                            },
                        },
                    });
                    if ((x == 0) && (p_x != 0))
                        || ((y == 0) && (p_y != 0))
                        || ((x == 15) && (p_x < p_xmax - 1))
                        || ((y == 15) && (p_y < p_ymax - 1))
                    {
                        terrain.insert(map::ProvinceBorder {
                            color: Color::Rgba {
                                red: 255.,
                                green: 0.,
                                blue: 0.,
                                alpha: 255.,
                            },
                        });
                    }

                    province.spawn().insert_bundle(map::CityBundle {
                        city: map::City {
                            city_type: map::CityType::Pyramid,
                        },
                        province: map::ProvinceId(province_entity),
                        position: map::Position {
                            x: 5 + (p_x * 16),
                            y: 5 + (p_y * 16),
                        },
                    });
                }
            }
        });
}

fn build_unit(commands: &mut Commands) {
    commands.spawn().insert_bundle(units::UnitBundle {
        position: map::Position { x: 16, y: 16 },
        unit: units::Unit {
            unit_type: units::UnitType::Skeleton,
        },
        figures: units::UnitFigures {
            health: vec![1, 1, 1, 1],
        },
    });

    commands.spawn().insert_bundle(units::UnitBundle {
        position: map::Position { x: 17, y: 16 },
        unit: units::Unit {
            unit_type: units::UnitType::Skeleton,
        },
        figures: units::UnitFigures {
            health: vec![1, 1, 1],
        },
    });

    commands.spawn().insert_bundle(units::UnitBundle {
        position: map::Position { x: 18, y: 16 },
        unit: units::Unit {
            unit_type: units::UnitType::DeathKnight,
        },
        figures: units::UnitFigures { health: vec![1, 1] },
    });

    commands.spawn().insert_bundle(units::UnitBundle {
        position: map::Position { x: 18, y: 16 },
        unit: units::Unit {
            unit_type: units::UnitType::GiantSpider,
        },
        figures: units::UnitFigures { health: vec![1] },
    });
}
