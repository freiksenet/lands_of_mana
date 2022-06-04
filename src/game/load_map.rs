use std::{collections::HashMap, path::Path};

use euclid::point2;
use fart_2d_geom::ConvexPolygon;
use num_traits::FromPrimitive;
use strum_macros::{Display, EnumIter, EnumString};
use tiled::{
    Chunk, FiniteTileLayer, InfiniteTileLayer, LayerType, Loader, Map, ObjectLayer, ObjectShape,
    TileLayer,
};

use crate::prelude::*;

pub fn load_map(mut commands: Commands) {
    let mut loader = Loader::new();
    let map = loader
        .load_tmx_map(Path::new("./maps/world_of_magic_export.tmx"))
        .unwrap();

    let base_layer = get_tile_layer(&map, TileLayerName::Base);
    let width = base_layer.width();
    let height = base_layer.height();

    let world_entity = commands
        .spawn_bundle((super::GameWorld {}, super::map::Map { width, height }))
        .id();

    let mut province_polygons = Vec::new();
    let province_layer = get_object_layer(&map, ObjectLayerName::Provinces);
    for province in province_layer.objects() {
        if let ObjectShape::Polygon { points } = &province.shape {
            let province_polygon = ConvexPolygon::<f32, ()>::hull(
                points
                    .iter()
                    .map(|(x, y)| point2(province.x + *x, province.y + *y))
                    .collect(),
            )
            .unwrap();
            let province_entity = commands
                .spawn()
                .insert(super::map::Province {
                    name: province.id().to_string(),
                })
                .id();
            commands.entity(world_entity).add_child(province_entity);
            province_polygons.push((province_entity, province_polygon));
        }
    }

    for map_x in 0..width {
        for map_y in 0..height {
            let tile = base_layer.get_tile(map_x as i32, map_y as i32).unwrap();
            let center_point = point2((map_x * 16) as f32 + 8., (map_y * 16) as f32 + 8.);
            let province_option = province_polygons
                .iter()
                .find(|(_, polygon)| polygon.contains_point(center_point));
            if let Some((province_entity, _)) = province_option {
                let x = map_x;
                let y = height - map_y - 1;
                let terrain = commands
                    .spawn_bundle(super::map::TerrainBundle {
                        province: super::map::ProvinceId(*province_entity),
                        position: super::map::Position { x, y },
                        base: super::map::TerrainBase {
                            terrain_type: super::map::TerrainType::from_u32(tile.id()).unwrap(),
                        },
                    })
                    .id();
                commands.entity(*province_entity).add_child(terrain);
            } else {
                panic!("NOT FOUND{:?}", (center_point, tile.id()));
            }
        }
    }

    commands.insert_resource(NextState(config::EngineState::LoadingWorld.next()));
}

fn get_tile_layer(map: &Map, layer: TileLayerName) -> FiniteTileLayer {
    match map
        .layers()
        .find(|found_layer| found_layer.name == layer.to_string())
        .unwrap()
        .layer_type()
    {
        LayerType::TileLayer(layer) => match layer {
            TileLayer::Finite(layer) => layer,
            _ => panic!("Wrong layer type"),
        },
        _ => panic!("Wrong layer type"),
    }
}

fn get_object_layer(map: &Map, layer: ObjectLayerName) -> ObjectLayer {
    match map
        .layers()
        .find(|found_layer| found_layer.name == layer.to_string())
        .unwrap()
        .layer_type()
    {
        LayerType::ObjectLayer(layer) => layer,
        _ => panic!("Wrong layer type"),
    }
}

#[derive(Clone, Copy, Debug, EnumString, EnumIter, Display)]
enum TileLayerName {
    Base,
}

#[derive(Clone, Copy, Debug, EnumString, EnumIter, Display)]
enum ObjectLayerName {
    Sites,
    Provinces,
}
