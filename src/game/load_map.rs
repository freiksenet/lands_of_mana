use std::{path::Path, str::FromStr};

use euclid::point2;
use fart_2d_geom::ConvexPolygon;
use num_traits::FromPrimitive;
use strum_macros::{Display, EnumIter, EnumString};
use tiled::{
    FiniteTileLayer, LayerType, Loader, Map, ObjectLayer, ObjectShape, PropertyValue, TileLayer,
};

use super::{
    map::{ForestType, MountainType, RoadType, TerrainTop},
    province::CityBundle,
};
use crate::{
    game::map::{Position, TerrainBase, TerrainBundle, TerrainType},
    prelude::*,
};

pub fn load_map(
    mut commands: Commands,
    world_query: Query<Entity, With<game::GameWorld>>,
    player_query: Query<Entity, With<game::world::Player>>,
) {
    let mut loader = Loader::new();
    let map = loader
        .load_tmx_map(Path::new("./assets/maps/world_of_magic.tmx"))
        .unwrap();

    let base_layer = get_tile_layer(&map, TileLayerName::Base);
    let rivers_layer = get_tile_layer(&map, TileLayerName::Rivers);
    let roads_layer = get_tile_layer(&map, TileLayerName::Roads);
    let forests_and_mountains_layer = get_tile_layer(&map, TileLayerName::ForestsAndMountains);
    let width = base_layer.width();
    let height = base_layer.height();

    let world_entity = world_query.single();
    let player_entity = player_query.single();

    commands
        .entity(world_entity)
        .insert(game::map::Map { width, height });

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
                .insert(game::province::Province {
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
            let river = rivers_layer.get_tile(map_x as i32, map_y as i32);
            let road = roads_layer.get_tile(map_x as i32, map_y as i32);
            let forest_and_mountain =
                forests_and_mountains_layer.get_tile(map_x as i32, map_y as i32);
            let center_point = point2((map_x * 16) as f32 + 8., (map_y * 16) as f32 + 8.);
            let province_option = province_polygons
                .iter()
                .find(|(_, polygon)| polygon.contains_point(center_point));
            if let Some((province_entity, _)) = province_option {
                let x = map_x;
                let y = height - map_y - 1;
                let terrain_top = match (river, road, forest_and_mountain) {
                    (Some(_), None, _) => TerrainTop::River,
                    (None, Some(_), _) => TerrainTop::Road(RoadType::Path),
                    (Some(_), Some(_), _) => TerrainTop::RiverWithBridge(RoadType::Path),
                    (_, _, Some(tile)) if tile.id() >= 1248 => {
                        TerrainTop::Mountain(MountainType::Rock)
                    }
                    (_, _, Some(_)) => TerrainTop::Forest(ForestType::Pine),
                    _ => TerrainTop::None,
                };
                let terrain = commands
                    .spawn_bundle(TerrainBundle {
                        province: game::province::InProvince(*province_entity),
                        position: Position::new(x, y),
                        base: TerrainBase(TerrainType::from_u32(tile.id()).unwrap()),
                        top: terrain_top,
                        ..Default::default()
                    })
                    .id();
                commands.entity(*province_entity).add_child(terrain);
            } else {
                panic!("NOT FOUND{:?}", (map_x, map_y, center_point, tile.id()));
            }
        }
    }

    let cities_layer = get_object_layer(&map, ObjectLayerName::Cities);
    for city in cities_layer.objects() {
        let center_point = point2(city.x, city.y);
        let province_option = province_polygons
            .iter()
            .find(|(_, polygon)| polygon.contains_point(center_point));
        if let Some((province_entity, _)) = province_option {
            let x = (city.x / 16.) as u32;
            let y = height - (city.y / 16.) as u32 - 1;
            if let Some(PropertyValue::StringValue(city_type_str)) =
                city.properties.get("city_type")
            {
                let city_type = game::province::CityType::from_str(city_type_str).unwrap();
                let city = CityBundle::new_empty_city(
                    &mut commands.spawn(),
                    player_entity,
                    city_type.get_city_stats(),
                    *province_entity,
                    game::map::Position { x, y },
                );
                commands.entity(*province_entity).add_child(city);
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
        LayerType::TileLayer(TileLayer::Finite(layer)) => layer,
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
    Rivers,
    Roads,
    ForestsAndMountains,
}

#[derive(Clone, Copy, Debug, EnumString, EnumIter, Display)]
enum ObjectLayerName {
    Sites,
    Cities,
    Provinces,
}
