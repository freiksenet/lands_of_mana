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

use crate::prelude::{render::z_level::ZLevel, *};

#[derive(Debug, Clone, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
#[strum_discriminants(vis(pub))]
#[strum_discriminants(name(TilemapLayerType))]
pub enum TilemapLayer {
    Background(LayerInner),
    Base(LayerInner),
    Connectors(LayerInner),
    Borders(LayerInner),
    Rivers(LayerInner),
    Roads(LayerInner),
    Forests(LayerInner),
    Mountains(LayerInner),
    Topology(LayerInner),
    Decorations(LayerInner),
    Sites(LayerInner),
}

impl TilemapLayer {
    pub fn new(
        child_builder: &mut ChildBuilder,
        map: &game::map::Map,
        layer_type: &TilemapLayerType,
        z: f32,
    ) -> TilemapLayer {
        let size: Tilemap2dSize = match layer_type {
            TilemapLayerType::Background => Tilemap2dSize {
                x: map.width + 2 + 10,
                y: map.height + 2 + 10,
            },
            _ => Tilemap2dSize {
                x: map.width,
                y: map.height,
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
            TilemapLayerType::Rivers => TilemapLayer::Rivers(layer),
            TilemapLayerType::Roads => TilemapLayer::Roads(layer),
            TilemapLayerType::Forests => TilemapLayer::Forests(layer),
            TilemapLayerType::Mountains => TilemapLayer::Mountains(layer),

            TilemapLayerType::Topology => TilemapLayer::Topology(layer),
            TilemapLayerType::Decorations => TilemapLayer::Decorations(layer),
        }
    }

    fn get_layer(self) -> LayerInner {
        match self {
            TilemapLayer::Background(l)
            | TilemapLayer::Base(l)
            | TilemapLayer::Connectors(l)
            | TilemapLayer::Borders(l)
            | TilemapLayer::Sites(l)
            | TilemapLayer::Rivers(l)
            | TilemapLayer::Roads(l)
            | TilemapLayer::Forests(l)
            | TilemapLayer::Mountains(l)
            | TilemapLayer::Topology(l)
            | TilemapLayer::Decorations(l) => l,
        }
    }

    fn get_layer_mut(&mut self) -> &mut LayerInner {
        match self {
            TilemapLayer::Background(l)
            | TilemapLayer::Base(l)
            | TilemapLayer::Connectors(l)
            | TilemapLayer::Borders(l)
            | TilemapLayer::Sites(l)
            | TilemapLayer::Rivers(l)
            | TilemapLayer::Roads(l)
            | TilemapLayer::Forests(l)
            | TilemapLayer::Mountains(l)
            | TilemapLayer::Topology(l)
            | TilemapLayer::Decorations(l) => l,
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
            TilemapLayer::Rivers(_) => {
                let (entity, bundle) = self.default_bundle_params();
                (
                    entity,
                    TilemapBundle {
                        texture_size: Tilemap2dTextureSize { x: 256., y: 256. },
                        texture: TilemapTexture(tiles.roads_and_rivers.clone()),
                        ..bundle
                    },
                )
            }
            TilemapLayer::Roads(_) => {
                let (entity, bundle) = self.default_bundle_params();
                (
                    entity,
                    TilemapBundle {
                        texture_size: Tilemap2dTextureSize { x: 256., y: 256. },
                        texture: TilemapTexture(tiles.roads_and_rivers.clone()),
                        ..bundle
                    },
                )
            }
            TilemapLayer::Forests(_) => {
                let (entity, bundle) = self.default_bundle_params();
                (
                    entity,
                    TilemapBundle {
                        texture_size: Tilemap2dTextureSize { x: 832., y: 464. },
                        texture: TilemapTexture(tiles.forest_and_mountains.clone()),
                        ..bundle
                    },
                )
            }
            TilemapLayer::Mountains(_) => {
                let (entity, bundle) = self.default_bundle_params();
                (
                    entity,
                    TilemapBundle {
                        texture_size: Tilemap2dTextureSize { x: 832., y: 464. },
                        texture: TilemapTexture(tiles.forest_and_mountains.clone()),
                        ..bundle
                    },
                )
            }
            TilemapLayer::Topology(_) => {
                let (entity, bundle) = self.default_bundle_params();
                (
                    entity,
                    TilemapBundle {
                        texture_size: Tilemap2dTextureSize { x: 192., y: 912. },
                        texture: TilemapTexture(tiles.decorations.clone()),
                        ..bundle
                    },
                )
            }
            TilemapLayer::Decorations(_) => {
                let (entity, bundle) = self.default_bundle_params();
                (
                    entity,
                    TilemapBundle {
                        texture_size: Tilemap2dTextureSize { x: 192., y: 912. },
                        texture: TilemapTexture(tiles.decorations.clone()),
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
    pub fn new(parent: &mut EntityCommands, map: &game::map::Map) -> TilemapLayerManager {
        let mut tilemap_layers = Vec::new();
        parent.with_children(|builder| {
            tilemap_layers.extend(vec![
                (
                    TilemapLayerType::Background,
                    TilemapLayer::new(
                        builder,
                        map,
                        &TilemapLayerType::Background,
                        ZLevel::Background.into(),
                    ),
                ),
                (
                    TilemapLayerType::Base,
                    TilemapLayer::new(builder, map, &TilemapLayerType::Base, ZLevel::Base.into()),
                ),
                (
                    TilemapLayerType::Connectors,
                    TilemapLayer::new(
                        builder,
                        map,
                        &TilemapLayerType::Connectors,
                        ZLevel::Connectors1.into(),
                    ),
                ),
                (
                    TilemapLayerType::Connectors,
                    TilemapLayer::new(
                        builder,
                        map,
                        &TilemapLayerType::Connectors,
                        ZLevel::Connectors2.into(),
                    ),
                ),
                (
                    TilemapLayerType::Connectors,
                    TilemapLayer::new(
                        builder,
                        map,
                        &TilemapLayerType::Connectors,
                        ZLevel::Connectors3.into(),
                    ),
                ),
                (
                    TilemapLayerType::Connectors,
                    TilemapLayer::new(
                        builder,
                        map,
                        &TilemapLayerType::Connectors,
                        ZLevel::Connectors4.into(),
                    ),
                ),
                (
                    TilemapLayerType::Sites,
                    TilemapLayer::new(builder, map, &TilemapLayerType::Sites, ZLevel::Sites.into()),
                ),
                (
                    TilemapLayerType::Borders,
                    TilemapLayer::new(
                        builder,
                        map,
                        &TilemapLayerType::Borders,
                        ZLevel::Borders.into(),
                    ),
                ),
                (
                    TilemapLayerType::Rivers,
                    TilemapLayer::new(
                        builder,
                        map,
                        &TilemapLayerType::Rivers,
                        ZLevel::Rivers.into(),
                    ),
                ),
                (
                    TilemapLayerType::Roads,
                    TilemapLayer::new(builder, map, &TilemapLayerType::Roads, ZLevel::Roads.into()),
                ),
                (
                    TilemapLayerType::Forests,
                    TilemapLayer::new(
                        builder,
                        map,
                        &TilemapLayerType::Forests,
                        ZLevel::Forests.into(),
                    ),
                ),
                (
                    TilemapLayerType::Mountains,
                    TilemapLayer::new(
                        builder,
                        map,
                        &TilemapLayerType::Mountains,
                        ZLevel::Mountains.into(),
                    ),
                ),
                (
                    TilemapLayerType::Topology,
                    TilemapLayer::new(
                        builder,
                        map,
                        &TilemapLayerType::Topology,
                        ZLevel::Topology.into(),
                    ),
                ),
                (
                    TilemapLayerType::Decorations,
                    TilemapLayer::new(
                        builder,
                        map,
                        &TilemapLayerType::Decorations,
                        ZLevel::Decorations.into(),
                    ),
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
