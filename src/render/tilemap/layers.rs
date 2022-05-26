use num_derive::FromPrimitive;
use std::{cmp::min, collections::HashMap};

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

use super::assets;
use super::game;

#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
#[strum_discriminants(vis(pub))]
#[strum_discriminants(name(TilemapLayerType))]
pub enum TilemapLayer {
    Background(LayerInner),
    Base(LayerInner),
    Connectors(LayerInner),
    Borders(LayerInner),
    Sites(LayerInner),
}

impl TilemapLayer {
    pub fn new(
        commands: &mut Commands,
        world: &game::map::GameWorld,
        layer_type: &TilemapLayerType,
        z: f32,
    ) -> TilemapLayer {
        let size: Tilemap2dSize = match layer_type {
            TilemapLayerType::Background => Tilemap2dSize {
                x: world.width + 2 + 10,
                y: world.height + 2 + 10,
            },
            _ => Tilemap2dSize {
                x: world.width,
                y: world.height,
            },
        };
        let storage = Tile2dStorage::empty(size);
        // storage.set(
        //     &TilePos2d { x: 0, y: 0 },
        //     Some(
        //         commands
        //             .spawn()
        //             .insert_bundle(TileBundle {
        //                 ..Default::default()
        //             })
        //             .id(),
        //     ),
        // );
        let layer = LayerInner {
            size,
            entity: commands.spawn().id(),
            storage,
            z,
        };
        match layer_type {
            TilemapLayerType::Background => TilemapLayer::Background(layer),
            TilemapLayerType::Base => TilemapLayer::Base(layer),
            TilemapLayerType::Connectors => TilemapLayer::Connectors(layer),
            TilemapLayerType::Borders => TilemapLayer::Borders(layer),
            TilemapLayerType::Sites => TilemapLayer::Sites(layer),
        }
    }

    fn get_layer(self) -> LayerInner {
        match self {
            TilemapLayer::Background(l)
            | TilemapLayer::Base(l)
            | TilemapLayer::Connectors(l)
            | TilemapLayer::Borders(l)
            | TilemapLayer::Sites(l) => l,
        }
    }

    fn get_layer_mut(&mut self) -> &mut LayerInner {
        match self {
            TilemapLayer::Background(l)
            | TilemapLayer::Base(l)
            | TilemapLayer::Connectors(l)
            | TilemapLayer::Borders(l)
            | TilemapLayer::Sites(l) => l,
        }
    }

    pub fn insert_tile_bundle(
        &mut self,
        tile_pos: &TilePos2d,
        commands: &mut Commands,
        tile_bundle: TileBundle,
    ) {
        let layer = self.get_layer_mut();
        let tile_entity = commands
            .spawn()
            .insert_bundle(TileBundle {
                tilemap_id: TilemapId(layer.entity),
                ..tile_bundle
            })
            .id();
        layer.storage.set(tile_pos, Some(tile_entity));
    }

    fn default_bundle_params(self) -> (Entity, TilemapBundle) {
        let layer = self.get_layer();
        let size = layer.size;
        let storage = layer.storage;
        let tile_size = Tilemap2dTileSize { x: 16.0, y: 16.0 };
        let grid_size = Tilemap2dGridSize { x: 16.0, y: 16.0 };
        let transform =
            bevy_ecs_tilemap::helpers::get_centered_transform_2d(&size, &tile_size, layer.z);
        (
            layer.entity,
            TilemapBundle {
                grid_size,
                size,
                storage,
                tile_size,
                transform,
                ..Default::default()
            },
        )
    }

    pub fn get_tilemap_bundle(self, tiles: &ResMut<assets::TileAssets>) -> (Entity, TilemapBundle) {
        match self {
            TilemapLayer::Background(_) => {
                let (entity, bundle) = self.default_bundle_params();
                (
                    entity,
                    TilemapBundle {
                        texture_size: Tilemap2dTextureSize { x: 256., y: 160. },
                        texture: TilemapTexture(tiles.fog_of_war_and_map.clone()),
                        ..bundle
                    },
                )
            }
            TilemapLayer::Base(_) => {
                let (entity, bundle) = self.default_bundle_params();
                (
                    entity,
                    TilemapBundle {
                        texture_size: Tilemap2dTextureSize { x: 192., y: 32. },
                        texture: TilemapTexture(tiles.terrain_base.clone()),
                        ..bundle
                    },
                )
            }
            TilemapLayer::Connectors(_) => {
                let (entity, bundle) = self.default_bundle_params();
                (
                    entity,
                    TilemapBundle {
                        texture_size: Tilemap2dTextureSize { x: 288., y: 1104. },
                        texture: TilemapTexture(tiles.terrain_connectors.clone()),
                        ..bundle
                    },
                )
            }
            TilemapLayer::Borders(_) => {
                let (entity, bundle) = self.default_bundle_params();
                (
                    entity,
                    TilemapBundle {
                        texture_size: Tilemap2dTextureSize { x: 256., y: 160. },
                        texture: TilemapTexture(tiles.fog_of_war_and_map.clone()),
                        ..bundle
                    },
                )
            }
            TilemapLayer::Sites(_) => {
                let (entity, bundle) = self.default_bundle_params();
                (
                    entity,
                    TilemapBundle {
                        texture_size: Tilemap2dTextureSize { x: 256., y: 1152. },
                        texture: TilemapTexture(tiles.sites.clone()),
                        ..bundle
                    },
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct LayerInner {
    pub size: Tilemap2dSize,
    pub entity: Entity,
    pub storage: Tile2dStorage,
    pub z: f32,
}
pub struct TilemapLayerManager {
    tilemap_layers: Vec<((TilemapLayerType, f32), TilemapLayer)>,
}

impl TilemapLayerManager {
    pub fn new(commands: &mut Commands, world: &game::map::GameWorld) -> TilemapLayerManager {
        let mut tilemap_layers = Vec::new();
        tilemap_layers.push((
            (TilemapLayerType::Background, 50.),
            TilemapLayer::new(commands, world, &TilemapLayerType::Background, 50.),
        ));
        tilemap_layers.push((
            (TilemapLayerType::Base, 1.),
            TilemapLayer::new(commands, world, &TilemapLayerType::Base, 1.),
        ));

        tilemap_layers.push((
            (TilemapLayerType::Connectors, 2.),
            TilemapLayer::new(commands, world, &TilemapLayerType::Connectors, 2.),
        ));

        tilemap_layers.push((
            (TilemapLayerType::Connectors, 3.),
            TilemapLayer::new(commands, world, &TilemapLayerType::Connectors, 3.),
        ));

        tilemap_layers.push((
            (TilemapLayerType::Connectors, 4.),
            TilemapLayer::new(commands, world, &TilemapLayerType::Connectors, 4.),
        ));

        tilemap_layers.push((
            (TilemapLayerType::Connectors, 5.),
            TilemapLayer::new(commands, world, &TilemapLayerType::Connectors, 5.),
        ));

        tilemap_layers.push((
            (TilemapLayerType::Sites, 7.),
            TilemapLayer::new(commands, world, &TilemapLayerType::Sites, 7.),
        ));

        tilemap_layers.push((
            (TilemapLayerType::Borders, 10.),
            TilemapLayer::new(commands, world, &TilemapLayerType::Borders, 10.),
        ));
        TilemapLayerManager { tilemap_layers }
    }

    pub fn insert_tile_bundle(
        &mut self,
        (tilemap_layer_type, z): (&TilemapLayerType, f32),
        tile_pos: &TilePos2d,
        commands: &mut Commands,
        tile_bundle: TileBundle,
    ) {
        self.tilemap_layers
            .iter_mut()
            .find(|((layer_type, layer_z), _)| layer_type == tilemap_layer_type && *layer_z == z)
            .unwrap()
            .1
            .insert_tile_bundle(tile_pos, commands, tile_bundle);
    }

    pub fn insert_terrain_bundles(
        &mut self,
        tile_pos: &TilePos2d,
        commands: &mut Commands,
        tile_bundles: &mut Vec<TileBundle>,
    ) {
        for (i, tile_bundle) in tile_bundles.drain(..min(tile_bundles.len(), 5)).enumerate() {
            match i {
                0 => self.insert_tile_bundle(
                    (&TilemapLayerType::Base, 1.),
                    tile_pos,
                    commands,
                    tile_bundle,
                ),
                1..=4 => self.insert_tile_bundle(
                    (&TilemapLayerType::Connectors, 1. + i as f32),
                    tile_pos,
                    commands,
                    tile_bundle,
                ),
                _ => panic!("Too many layers"),
            }
        }
    }

    pub fn drain_all_tilemaps_to_bundle(
        &mut self,
        tiles: &ResMut<assets::TileAssets>,
    ) -> Vec<(Entity, TilemapBundle)> {
        let mut res = Vec::new();
        for (_, layer) in self.tilemap_layers.drain(..) {
            res.push(layer.get_tilemap_bundle(&tiles))
        }
        res
    }
}
