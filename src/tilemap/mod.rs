use num_derive::FromPrimitive;
use std::collections::HashMap;

use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use bevy::prelude::*;

use bevy_ecs_tilemap::{
    map::{
        Tilemap2dGridSize, Tilemap2dSize, Tilemap2dTextureSize, Tilemap2dTileSize, TilemapId,
        TilemapTexture,
    },
    tiles::{Tile2dStorage, TileBundle, TilePos2d, TileTexture},
    TilemapBundle,
};

mod tile_selection;

use self::tile_selection::TerrainCornersTexture;

use super::assets;
use super::game;

#[derive(PartialEq, Hash, Debug, EnumIter, Eq, FromPrimitive, Copy, Clone, Display)]
enum TileMapLayerType {
    Base = 0,
    Connectors1 = 1,
    Connectors2 = 2,
    Connectors3 = 3,
    Connectors4 = 4,
    Borders = 10,
}
struct TileMapLayer {
    layer_type: TileMapLayerType,
    entity: Entity,
    storage: Tile2dStorage,
}

pub fn setup(
    mut commands: Commands,
    tiles: ResMut<assets::TileAssets>,
    world_query: Query<&game::map::GameWorld>,
    terrain_query: Query<(
        &game::map::TerrainPosition,
        &game::map::TerrainBase,
        Option<&game::map::ProvinceBorder>,
    )>,
) {
    let game_world = world_query.single();

    let tilemap_size = Tilemap2dSize {
        x: game_world.width,
        y: game_world.height,
    };

    let grid_size = Tilemap2dGridSize { x: 16.0, y: 16.0 };

    let mut layers: Vec<TileMapLayer> = TileMapLayerType::iter()
        .map(|layer_type| TileMapLayer {
            layer_type,
            entity: commands.spawn().id(),
            storage: Tile2dStorage::empty(tilemap_size),
        })
        .collect();

    let mut pos_to_terrain: HashMap<&game::map::TerrainPosition, game::map::TerrainType> =
        HashMap::new();

    for (position, base, _) in terrain_query.iter() {
        pos_to_terrain.insert(position, base.terrain_type);
    }

    for (position, base, _border_option) in terrain_query.iter() {
        let tile_pos: TilePos2d = TilePos2d {
            x: position.x,
            y: position.y,
        };
        let corner = neighbors_to_corner(
            layers[0].storage.get_neighboring_pos(&tile_pos),
            &base.terrain_type,
            &pos_to_terrain,
        );

        for (i, tile_texture) in corner
            .get_tile_textures()
            .iter()
            .map(|id| TileTexture(*id))
            .enumerate()
        {
            let tilemap_layer = &mut layers[i];
            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_layer.entity),
                    texture: tile_texture,
                    ..Default::default()
                })
                .id();

            tilemap_layer.storage.set(&tile_pos, Some(tile_entity));
        }

        // if let Some(border) = border_option {
        //     let layer = layers
        //         .iter_mut()
        //         .find(|layer| layer.layer_type == TileMapLayerType::Borders)
        //         .unwrap();
        //     // layer.storage.set(
        //     //     &tile_pos,
        //     //     Some(
        //     //         commands
        //     //             .spawn()
        //     //             .insert_bundle(TileBundle {
        //     //                 position: tile_pos,
        //     //                 tilemap_id: TilemapId(layer.entity),
        //     //                 texture: TileTexture(152),
        //     //                 ..Default::default()
        //     //             })
        //     //             .id(),
        //     //     ),
        //     // );
        // }
    }

    let tile_size = Tilemap2dTileSize { x: 16.0, y: 16.0 };
    let texture_handle_base: Handle<Image> = tiles.terrain_base.clone();
    let texture_handle_connectors: Handle<Image> = tiles.terrain_connectors.clone();
    let texture_handle_border: Handle<Image> = tiles.window.clone();

    let base_layer_index = layers
        .iter_mut()
        .position(|layer| layer.layer_type == TileMapLayerType::Base)
        .unwrap();
    let base_layer = layers.remove(base_layer_index);

    commands
        .entity(base_layer.entity)
        .insert_bundle(TilemapBundle {
            grid_size,
            size: tilemap_size,
            storage: base_layer.storage,
            texture_size: Tilemap2dTextureSize { x: 192., y: 32. },
            texture: TilemapTexture(texture_handle_base),
            tile_size,
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
                &tilemap_size,
                &tile_size,
                base_layer.layer_type as usize as f32,
            ),
            ..Default::default()
        });

    let border_layer_index = layers
        .iter_mut()
        .position(|layer| layer.layer_type == TileMapLayerType::Borders)
        .unwrap();

    let border_layer = layers.remove(border_layer_index);

    commands
        .entity(border_layer.entity)
        .insert_bundle(TilemapBundle {
            grid_size,
            size: tilemap_size,
            storage: border_layer.storage,
            texture_size: Tilemap2dTextureSize { x: 80., y: 1280. },
            texture: TilemapTexture(texture_handle_border),
            tile_size,
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
                &tilemap_size,
                &tile_size,
                border_layer.layer_type as usize as f32,
            ),
            ..Default::default()
        });

    for layer in layers.drain(..) {
        commands.entity(layer.entity).insert_bundle(TilemapBundle {
            grid_size,
            size: tilemap_size,
            storage: layer.storage,
            texture_size: Tilemap2dTextureSize { x: 272.0, y: 960.0 },
            texture: TilemapTexture(texture_handle_connectors.clone()),
            tile_size,
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
                &tilemap_size,
                &tile_size,
                layer.layer_type as usize as f32,
            ),
            ..Default::default()
        });
    }
}

fn neighbors_to_corner(
    neighbors: [Option<TilePos2d>; 8],
    base: &game::map::TerrainType,
    pos_to_terrain: &HashMap<&game::map::TerrainPosition, game::map::TerrainType>,
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
    pos_to_terrain: &HashMap<&game::map::TerrainPosition, game::map::TerrainType>,
) -> game::map::TerrainType {
    *tile_pos_option
        .and_then(|pos| pos_to_terrain.get(&game::map::TerrainPosition { x: pos.x, y: pos.y }))
        .unwrap_or(base)
}
