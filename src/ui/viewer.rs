use std::{cmp::Ordering, collections::BTreeSet, hash::Hash};

use bevy::{
    ecs::query::QueryItem,
    utils::{HashMap, HashSet},
};

use crate::{
    game::{
        map::{Position, Terrain},
        province::{CityTileIndex, InProvince},
        units::Unit,
    },
    prelude::*,
};

#[derive(Component, Debug, Default)]
pub struct Viewer {}

#[derive(Component, Debug)]
pub struct ViewerMap {
    position_grid: HashMap<Position, BTreeSet<EntityOnTile>>,
    entity_to_position: HashMap<EntityOnTile, Position>,
}

impl Default for ViewerMap {
    fn default() -> Self {
        ViewerMap {
            position_grid: HashMap::new(),
            entity_to_position: HashMap::new(),
        }
    }
}

impl ViewerMap {
    pub fn put_entity(&mut self, position: &Position, entity_on_tile: EntityOnTile) {
        if let Some(existing_position) = self.entity_to_position.get(&entity_on_tile) {
            if let Some(entities) = self.position_grid.get_mut(existing_position) {
                entities.remove(&entity_on_tile);
            }
        }
        #[allow(unused_must_use)]
        {
            self.position_grid.try_insert(*position, BTreeSet::new());
        }
        self.position_grid.entry(*position).and_modify(|m| {
            m.insert(entity_on_tile);
        });
        self.entity_to_position.insert(entity_on_tile, *position);
    }

    pub fn remove_entity(&mut self, entity: &Entity) {
        let entities_to_remove: Vec<(EntityOnTile, Position)> = self
            .entity_to_position
            .iter()
            .filter(|(entity_on_tile, _)| match entity_on_tile {
                EntityOnTile::Province {
                    province_entity,
                    tile_entity,
                } => province_entity == entity || tile_entity == entity,
                EntityOnTile::Terrain(terrain_entity) => terrain_entity == entity,
                EntityOnTile::City {
                    city_entity,
                    tile_entity,
                } => city_entity == entity || tile_entity == entity,
                EntityOnTile::Unit(unit_entity) => unit_entity == entity,
            })
            .map(|(entity, position)| (*entity, *position))
            .collect();
        for (entity_on_tile, position) in entities_to_remove {
            self.entity_to_position.remove(&entity_on_tile);
            if let Some(entities) = self.position_grid.get_mut(&position) {
                entities.remove(&entity_on_tile);
            }
        }
    }

    pub fn entities_at_position(&self, position: &Position) -> Option<&BTreeSet<EntityOnTile>> {
        self.position_grid.get(position)
    }

    pub fn entities_in_bounding_box(
        &self,
        bounding_box: &(Position, Position),
    ) -> HashSet<EntityOnTile> {
        let min_x = std::cmp::min(bounding_box.0.x, bounding_box.1.x);
        let max_x = std::cmp::max(bounding_box.0.x, bounding_box.1.x) + 1;
        let min_y = std::cmp::min(bounding_box.0.y, bounding_box.1.y);
        let max_y = std::cmp::max(bounding_box.0.y, bounding_box.1.y) + 1;
        let mut result = HashSet::new();

        for x in min_x..max_x {
            for y in min_y..max_y {
                if let Some(entities_at_position) = self.entities_at_position(&Position::new(x, y))
                {
                    entities_at_position.iter().for_each(|entity| {
                        result.insert(*entity);
                    })
                }
            }
        }
        result
    }
}

type TerrainPositionQuery = (&'static Position, Entity, &'static InProvince);
type CityPositionQuery = (&'static Position, Entity, &'static Parent);
type UnitPositionQuery = (&'static Position, Entity);

fn set_terrain_to_viewer_map(
    viewer_map: &mut Mut<ViewerMap>,
    (position, terrain_entity, &InProvince(province_entity)): QueryItem<TerrainPositionQuery>,
) {
    viewer_map.put_entity(
        position,
        EntityOnTile::Province {
            province_entity,
            tile_entity: terrain_entity,
        },
    );
    viewer_map.put_entity(position, EntityOnTile::Terrain(terrain_entity));
}

fn set_city_to_viewer_map(
    viewer_map: &mut Mut<ViewerMap>,
    (position, tile_entity, &Parent(city_entity)): QueryItem<CityPositionQuery>,
) {
    viewer_map.put_entity(
        position,
        EntityOnTile::City {
            tile_entity,
            city_entity,
        },
    );
}

fn set_unit_to_viewer_map(
    viewer_map: &mut Mut<ViewerMap>,
    (position, unit_entity): QueryItem<UnitPositionQuery>,
) {
    viewer_map.put_entity(position, EntityOnTile::Unit(unit_entity));
}

pub fn add_new_entitites_viewer_map(
    mut viewer_query: Query<&mut ViewerMap, With<Viewer>>,
    terrain_query: Query<TerrainPositionQuery, (With<Terrain>, Added<Position>)>,
    city_query: Query<CityPositionQuery, (With<CityTileIndex>, Added<Position>)>,
    units_query: Query<UnitPositionQuery, (With<Unit>, Added<Position>)>,
) {
    let mut viewer_map = viewer_query.single_mut();
    terrain_query.for_each(|terrain_item| set_terrain_to_viewer_map(&mut viewer_map, terrain_item));
    city_query.for_each(|city_item| set_city_to_viewer_map(&mut viewer_map, city_item));
    units_query.for_each(|unit_item| set_unit_to_viewer_map(&mut viewer_map, unit_item));
}

pub fn update_position_on_viewer_map(
    mut viewer_query: Query<&mut ViewerMap, With<Viewer>>,
    terrain_query: Query<TerrainPositionQuery, (With<Terrain>, Changed<Position>)>,
    city_query: Query<CityPositionQuery, (With<CityTileIndex>, Changed<Position>)>,
    units_query: Query<UnitPositionQuery, (With<Unit>, Changed<Position>)>,
) {
    let mut viewer_map = viewer_query.single_mut();
    terrain_query.for_each(|terrain_item| set_terrain_to_viewer_map(&mut viewer_map, terrain_item));
    city_query.for_each(|city_item| set_city_to_viewer_map(&mut viewer_map, city_item));
    units_query.for_each(|unit_item| set_unit_to_viewer_map(&mut viewer_map, unit_item));
}

pub fn remove_entities_from_viewer_map(
    mut viewer_query: Query<&mut ViewerMap, With<Viewer>>,
    removed_positions: RemovedComponents<Position>,
) {
    let mut viewer_map = viewer_query.single_mut();

    for entity in removed_positions.iter() {
        viewer_map.remove_entity(&entity);
    }
}

#[derive(Debug, Eq, Clone, Copy)]
pub enum EntityOnTile {
    Province {
        province_entity: Entity,
        tile_entity: Entity,
    },
    Terrain(Entity),
    City {
        city_entity: Entity,
        tile_entity: Entity,
    },
    Unit(Entity),
}

impl Hash for EntityOnTile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            EntityOnTile::Province {
                province_entity,
                tile_entity,
            } => {
                province_entity.hash(state);
                tile_entity.hash(state);
            }
            EntityOnTile::City { tile_entity, .. } => tile_entity.hash(state),
            _ => self.entity().hash(state),
        };
    }
}

impl PartialOrd for EntityOnTile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EntityOnTile {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (
                EntityOnTile::Province {
                    province_entity,
                    tile_entity,
                },
                EntityOnTile::Province {
                    province_entity: other_province_entity,
                    tile_entity: other_tile_entity,
                },
            ) if province_entity == other_province_entity => tile_entity.cmp(other_tile_entity),
            (
                EntityOnTile::Province {
                    province_entity, ..
                },
                EntityOnTile::Province {
                    province_entity: other_province_entity,
                    ..
                },
            ) => province_entity.cmp(other_province_entity),
            (EntityOnTile::Terrain(entity), EntityOnTile::Terrain(other_entity))
            | (
                EntityOnTile::City {
                    tile_entity: entity,
                    ..
                },
                EntityOnTile::City {
                    tile_entity: other_entity,
                    ..
                },
            )
            | (EntityOnTile::Unit(entity), EntityOnTile::Unit(other_entity)) => {
                entity.cmp(other_entity)
            }

            (EntityOnTile::Province { .. }, _) => Ordering::Greater,
            (_, EntityOnTile::Province { .. }) => Ordering::Less,
            (EntityOnTile::Terrain(_), _) => Ordering::Greater,
            (_, EntityOnTile::Terrain(_)) => Ordering::Less,
            (EntityOnTile::City { .. }, _) => Ordering::Greater,
            (_, EntityOnTile::City { .. }) => Ordering::Less,
        }
    }
}

impl PartialEq for EntityOnTile {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                EntityOnTile::Province {
                    province_entity,
                    tile_entity,
                },
                EntityOnTile::Province {
                    province_entity: other_province_entity,
                    tile_entity: other_tile_entity,
                },
            ) => province_entity == other_province_entity && tile_entity == other_tile_entity,

            (EntityOnTile::Terrain(entity), EntityOnTile::Terrain(other_entity)) => {
                entity == other_entity
            }
            (
                EntityOnTile::City { tile_entity, .. },
                EntityOnTile::City {
                    tile_entity: other_tile_entity,
                    ..
                },
            ) => tile_entity == other_tile_entity,

            (EntityOnTile::Unit(entity), EntityOnTile::Unit(other_entity)) => {
                entity == other_entity
            }

            (_, _) => false,
        }
    }
}

impl EntityOnTile {
    pub fn entity(&self) -> Entity {
        *match self {
            EntityOnTile::Province {
                province_entity, ..
            } => province_entity,
            EntityOnTile::Terrain(entity) => entity,
            EntityOnTile::City { city_entity, .. } => city_entity,
            EntityOnTile::Unit(entity) => entity,
        }
    }
}
