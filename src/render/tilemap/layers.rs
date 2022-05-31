use std::cmp::min;

use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_ecs_tilemap::{
    map::{
        Tilemap2dGridSize, Tilemap2dSize, Tilemap2dTextureSize, Tilemap2dTileSize, TilemapId,
        TilemapTexture,
    },
    tiles::{Tile2dStorage, TileBundle, TilePos2d},
    TilemapBundle,
};
use strum_macros::{EnumDiscriminants, EnumIter};

use super::{assets, game};

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
        child_builder: &mut ChildBuilder,
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
        let layer = LayerInner {
            size,
            entity: child_builder.spawn().id(),
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
        builder: &mut ChildBuilder,
        tile_pos: &TilePos2d,
        tile_bundle: TileBundle,
    ) {
        let layer = self.get_layer_mut();
        let tile_entity = builder
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
    tilemap_layers: Vec<(TilemapLayerType, TilemapLayer)>,
}

impl TilemapLayerManager {
    pub fn new(parent: &mut EntityCommands, world: &game::map::GameWorld) -> TilemapLayerManager {
        let mut tilemap_layers = Vec::new();
        parent.with_children(|builder| {
            tilemap_layers.extend(vec![
                (
                    TilemapLayerType::Background,
                    TilemapLayer::new(builder, world, &TilemapLayerType::Background, 50.),
                ),
                (
                    TilemapLayerType::Base,
                    TilemapLayer::new(builder, world, &TilemapLayerType::Base, 1.),
                ),
                (
                    TilemapLayerType::Connectors,
                    TilemapLayer::new(builder, world, &TilemapLayerType::Connectors, 2.),
                ),
                (
                    TilemapLayerType::Connectors,
                    TilemapLayer::new(builder, world, &TilemapLayerType::Connectors, 3.),
                ),
                (
                    TilemapLayerType::Connectors,
                    TilemapLayer::new(builder, world, &TilemapLayerType::Connectors, 4.),
                ),
                (
                    TilemapLayerType::Connectors,
                    TilemapLayer::new(builder, world, &TilemapLayerType::Connectors, 5.),
                ),
                (
                    TilemapLayerType::Sites,
                    TilemapLayer::new(builder, world, &TilemapLayerType::Sites, 7.),
                ),
                (
                    TilemapLayerType::Borders,
                    TilemapLayer::new(builder, world, &TilemapLayerType::Borders, 10.),
                ),
            ]);
        });
        TilemapLayerManager { tilemap_layers }
    }

    pub fn insert_tile_bundle(
        &mut self,
        builder: &mut ChildBuilder,
        tilemap_layer_type: &TilemapLayerType,
        tile_pos: &TilePos2d,
        tile_bundle: TileBundle,
    ) {
        self.tilemap_layers
            .iter_mut()
            .find(|(layer_type, _)| layer_type == tilemap_layer_type)
            .unwrap()
            .1
            .insert_tile_bundle(builder, tile_pos, tile_bundle);
    }

    pub fn insert_terrain_bundles(
        &mut self,
        builder: &mut ChildBuilder,
        tile_pos: &TilePos2d,
        tile_bundles: &mut Vec<TileBundle>,
    ) {
        for (i, tile_bundle) in tile_bundles.drain(..min(tile_bundles.len(), 5)).enumerate() {
            match i {
                0 => {
                    self.insert_tile_bundle(builder, &TilemapLayerType::Base, tile_pos, tile_bundle)
                }
                1..=4 => {
                    let mut connector_layers: Vec<&mut (TilemapLayerType, TilemapLayer)> = self
                        .tilemap_layers
                        .iter_mut()
                        .filter(|(layer_type, _)| layer_type == &TilemapLayerType::Connectors)
                        .collect();
                    connector_layers[i]
                        .1
                        .insert_tile_bundle(builder, tile_pos, tile_bundle);
                }
                _ => panic!("Too many layers"),
            }
        }
    }

    pub fn drain_all_tilemaps_to_bundle(
        &mut self,
        tiles: &ResMut<assets::TileAssets>,
    ) -> Vec<(Entity, TilemapBundle, TilemapLayerType)> {
        let mut res = Vec::new();
        for (layer_type, layer) in self.tilemap_layers.drain(..) {
            let (entity, bundle) = layer.get_tilemap_bundle(tiles);
            res.push((entity, bundle, layer_type));
        }
        res
    }
}
