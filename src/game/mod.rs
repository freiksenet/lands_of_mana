use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::state;

pub mod map;

pub fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(map::GameWorld {
            width: 64,
            height: 64,
        })
        .with_children(|world| {
            for p_x in 0..2 {
                for p_y in 0..2 {
                    build_province(world.spawn(), p_x, p_y, 2, 2);
                }
            }
        });
    commands.insert_resource(NextState(state::GameState::InGame));
}

fn build_province(mut entity: EntityCommands, p_x: u32, p_y: u32, p_xmax: u32, p_ymax: u32) {
    entity
        .insert(map::Province { name: "Condoria" })
        .with_children(|province| {
            let province_id = province.parent_entity().id();
            for x in 0..16 {
                for y in 0..16 {
                    let mut terrain = province.spawn();
                    terrain.insert_bundle(map::TerrainBundle {
                        province: map::TerrainProvince {
                            province: province_id,
                        },
                        position: map::TerrainPosition {
                            x: x + (p_x * 16),
                            y: y + (p_y * 16),
                        },
                        base: map::TerrainBase {
                            terrain_type: match (p_x, p_y) {
                                (0, 0) => map::TerrainType::Grass,
                                (0, 1) => map::TerrainType::Sea,
                                (1, 0) => map::TerrainType::Dryland,
                                (1, 1) => map::TerrainType::Lava,
                                _ => map::TerrainType::Dirt,
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
                }
            }
        });
}
