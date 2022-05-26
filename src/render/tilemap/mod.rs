use iyes_loopless::state::NextState;
use num_derive::FromPrimitive;
use std::collections::HashMap;

use strum::IntoEnumIterator;
use strum_macros::{Display, EnumDiscriminants, EnumIter};

use bevy::{ecs::system::EntityCommands, prelude::*};

use bevy_ecs_tilemap::{
    map::{
        Tilemap2dGridSize, Tilemap2dSize, Tilemap2dTextureSize, Tilemap2dTileSize, TilemapId,
        TilemapTexture,
    },
    tiles::{Tile2dStorage, TileBundle, TilePos2d, TileTexture},
    TilemapBundle,
};

mod layers;
mod tile_selection;

use self::tile_selection::TerrainCornersTexture;

use crate::assets;
use crate::game;

pub fn setup(
    mut commands: Commands,
    tiles: ResMut<assets::TileAssets>,
    world_query: Query<&game::map::GameWorld>,
    terrain_query: Query<(
        &game::map::Position,
        &game::map::TerrainBase,
        Option<&game::map::ProvinceBorder>,
    )>,
    city_query: Query<(&game::map::Position, &game::map::City)>,
) {
    let game_world = world_query.single();

    let size = Tilemap2dSize {
        x: game_world.width,
        y: game_world.height,
    };

    let temp_storage = Tile2dStorage::empty(size);

    let mut tilemap_layer_manager = layers::TilemapLayerManager::new(&mut commands, game_world);
    build_background(&mut commands, &mut tilemap_layer_manager, &size);

    let mut pos_to_terrain: HashMap<&game::map::Position, game::map::TerrainType> = HashMap::new();

    for (position, base, _) in terrain_query.iter() {
        pos_to_terrain.insert(position, base.terrain_type);
    }

    for (position, base, _border_option) in terrain_query.iter() {
        let tile_pos: TilePos2d = TilePos2d {
            x: position.x,
            y: position.y,
        };
        let corner = neighbors_to_corner(
            temp_storage.get_neighboring_pos(&tile_pos),
            &base.terrain_type,
            &pos_to_terrain,
        );

        let mut bundles: Vec<TileBundle> = corner
            .get_tile_textures()
            .drain(..)
            .map(|id| TileBundle {
                position: tile_pos,
                texture: TileTexture(id),
                ..Default::default()
            })
            .collect();
        tilemap_layer_manager.insert_terrain_bundles(&tile_pos, &mut commands, &mut bundles);
    }

    for (position, city) in city_query.iter() {
        for tile_bundle in build_city_tiles(position, city) {
            tilemap_layer_manager.insert_tile_bundle(
                (&layers::TilemapLayerType::Sites, 7.),
                &tile_bundle.position,
                &mut commands,
                tile_bundle,
            )
        }
    }

    for (entity, tilemap_bundle, tilemap_layer_type) in
        tilemap_layer_manager.drain_all_tilemaps_to_bundle(&tiles)
    {
        commands.entity(entity).insert_bundle(tilemap_bundle);
    }
}

fn neighbors_to_corner(
    neighbors: [Option<TilePos2d>; 8],
    base: &game::map::TerrainType,
    pos_to_terrain: &HashMap<&game::map::Position, game::map::TerrainType>,
) -> tile_selection::TerrainCorners {
    let [north, south, west, east, north_west, north_east, south_west, south_east] = neighbors;
    tile_selection::TerrainCorners {
        center: *base,
        north: unwrap_pos_to_terrain(north, base, pos_to_terrain),
        south: unwrap_pos_to_terrain(south, base, pos_to_terrain),
        west: unwrap_pos_to_terrain(west, base, pos_to_terrain),
        east: unwrap_pos_to_terrain(east, base, pos_to_terrain),
        north_west: unwrap_pos_to_terrain(north_west, base, pos_to_terrain),
        north_east: unwrap_pos_to_terrain(north_east, base, pos_to_terrain),
        south_west: unwrap_pos_to_terrain(south_west, base, pos_to_terrain),
        south_east: unwrap_pos_to_terrain(south_east, base, pos_to_terrain),
    }
}

fn unwrap_pos_to_terrain(
    tile_pos_option: Option<TilePos2d>,
    base: &game::map::TerrainType,
    pos_to_terrain: &HashMap<&game::map::Position, game::map::TerrainType>,
) -> game::map::TerrainType {
    *tile_pos_option
        .and_then(|pos| pos_to_terrain.get(&game::map::Position { x: pos.x, y: pos.y }))
        .unwrap_or(base)
}

fn build_background(
    commands: &mut Commands,
    tilemap_layer_manager: &mut layers::TilemapLayerManager,
    size: &Tilemap2dSize,
) {
    for x in 0..6 {
        for y in 0..size.y + 12 {
            let left_pos = TilePos2d { x, y };
            tilemap_layer_manager.insert_tile_bundle(
                (&layers::TilemapLayerType::Background, 50.),
                &left_pos,
                commands,
                TileBundle {
                    position: left_pos,
                    texture: TileTexture(31),
                    ..Default::default()
                },
            );
            let right_pos = TilePos2d {
                x: size.x + 6 + x,
                y,
            };
            tilemap_layer_manager.insert_tile_bundle(
                (&layers::TilemapLayerType::Background, 50.),
                &right_pos,
                commands,
                TileBundle {
                    position: right_pos,
                    texture: TileTexture(31),
                    ..Default::default()
                },
            );
        }
    }

    for y in 0..6 {
        for x in 6..size.x + 6 {
            let top_pos = TilePos2d { x, y };
            tilemap_layer_manager.insert_tile_bundle(
                (&layers::TilemapLayerType::Background, 50.),
                &top_pos,
                commands,
                TileBundle {
                    position: top_pos,
                    texture: TileTexture(31),
                    ..Default::default()
                },
            );
            let bottom_pos = TilePos2d {
                x,
                y: size.y + 6 + y,
            };
            tilemap_layer_manager.insert_tile_bundle(
                (&layers::TilemapLayerType::Background, 50.),
                &bottom_pos,
                commands,
                TileBundle {
                    position: bottom_pos,
                    texture: TileTexture(31),
                    ..Default::default()
                },
            );
        }
    }

    let left_top = TilePos2d {
        x: 6,
        y: size.y + 5,
    };
    tilemap_layer_manager.insert_tile_bundle(
        (&layers::TilemapLayerType::Background, 50.),
        &left_top,
        commands,
        TileBundle {
            position: left_top,
            texture: TileTexture(32),
            ..Default::default()
        },
    );
    let right_top = TilePos2d {
        x: size.x + 5,
        y: size.y + 5,
    };
    tilemap_layer_manager.insert_tile_bundle(
        (&layers::TilemapLayerType::Background, 50.),
        &right_top,
        commands,
        TileBundle {
            position: right_top,
            texture: TileTexture(34),
            ..Default::default()
        },
    );
    let left_bottom = TilePos2d { x: 6, y: 6 };
    tilemap_layer_manager.insert_tile_bundle(
        (&layers::TilemapLayerType::Background, 50.),
        &left_bottom,
        commands,
        TileBundle {
            position: left_bottom,
            texture: TileTexture(64),
            ..Default::default()
        },
    );
    let right_bottom = TilePos2d {
        x: size.x + 5,
        y: 6,
    };
    tilemap_layer_manager.insert_tile_bundle(
        (&layers::TilemapLayerType::Background, 50.),
        &right_bottom,
        commands,
        TileBundle {
            position: right_bottom,
            texture: TileTexture(66),
            ..Default::default()
        },
    );

    for x in 7..size.x + 5 {
        let bottom_tile = TilePos2d { x, y: 6 };
        tilemap_layer_manager.insert_tile_bundle(
            (&layers::TilemapLayerType::Background, 50.),
            &bottom_tile,
            commands,
            TileBundle {
                position: bottom_tile,
                texture: TileTexture(65),
                ..Default::default()
            },
        );
        let top_tile = TilePos2d { x, y: size.y + 5 };
        tilemap_layer_manager.insert_tile_bundle(
            (&layers::TilemapLayerType::Background, 50.),
            &top_tile,
            commands,
            TileBundle {
                position: top_tile,
                texture: TileTexture(33),
                ..Default::default()
            },
        );
    }

    for y in 7..size.y + 5 {
        let left_tile = TilePos2d { x: 6, y };
        tilemap_layer_manager.insert_tile_bundle(
            (&layers::TilemapLayerType::Background, 50.),
            &left_tile,
            commands,
            TileBundle {
                position: left_tile,
                texture: TileTexture(48),
                ..Default::default()
            },
        );
        let right_tile = TilePos2d { x: size.x + 5, y };
        tilemap_layer_manager.insert_tile_bundle(
            (&layers::TilemapLayerType::Background, 50.),
            &right_tile,
            commands,
            TileBundle {
                position: right_tile,
                texture: TileTexture(50),
                ..Default::default()
            },
        );
    }
}

fn build_city_tiles(
    game_position: &game::map::Position,
    city: &game::map::City,
) -> Vec<TileBundle> {
    // Top right corner
    let base_tile = match city.city_type {
        game::map::CityType::City1 => 896,
        game::map::CityType::City2 => 900,
        game::map::CityType::Desert => 904,
        game::map::CityType::Barbarian => 908,
        game::map::CityType::Mystic => 960,
        game::map::CityType::Pyramid => 964,
        game::map::CityType::Dwarf => 972,
        game::map::CityType::Lizardmen => 1032,
        game::map::CityType::Elf => 1100,
    };
    let mut city_tiles = Vec::new();
    for x in (0..4) {
        for y in (0..4) {
            let position = TilePos2d {
                x: game_position.x + x,
                y: game_position.y + y,
            };
            // coordinates are from bottom left corner, while textures are from top right
            city_tiles.push(TileBundle {
                position,
                texture: TileTexture(base_tile + x + ((3 - y) * 16)),
                ..Default::default()
            });
        }
    }
    city_tiles
}
