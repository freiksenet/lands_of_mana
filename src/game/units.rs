use std::collections::HashMap;

use bevy::ecs::{query::QueryItem, system::EntityCommands};
use strum_macros::{Display, EnumIter, EnumString};

use crate::{
    game::{map, map::Position, world, world::OfPlayer, GameTick},
    prelude::*,
};

#[derive(Component, Debug)]
pub struct UnitFigure {
    pub index: usize,
}

#[derive(Component, Debug)]
pub struct UnitFigureHealth(u32);

#[derive(Bundle, Debug)]
pub struct UnitFigureBundle {
    pub figure: UnitFigure,
    pub health: UnitFigureHealth,
    pub unit_type: UnitType,
}

impl UnitFigureBundle {
    pub fn new(unit_type: UnitType, index: usize, health: UnitFigureHealth) -> UnitFigureBundle {
        UnitFigureBundle {
            figure: UnitFigure { index },
            health,
            unit_type,
        }
    }
}

#[derive(Bundle, Debug, Default)]
pub struct UnitBundle {
    pub unit: Unit,
    pub unit_type: UnitType,
    pub position: map::Position,
    pub player: OfPlayer,
    pub orders: UnitOrders,
}

impl UnitBundle {
    pub fn insert_full(
        entity: &mut EntityCommands,
        player_entity: Entity,
        unit_type: UnitType,
        position: map::Position,
    ) -> Entity {
        let unit_stats = unit_type.get_unit_stats();
        entity
            .insert_bundle(UnitBundle {
                unit: Unit {},
                unit_type,
                position,
                player: OfPlayer(player_entity),
                ..Default::default()
            })
            .with_children(|unit| {
                for index in 0..unit_stats.max_figures {
                    unit.spawn().insert_bundle(UnitFigureBundle::new(
                        unit_type,
                        index,
                        UnitFigureHealth(unit_stats.max_health),
                    ));
                }
                for (resource, amount) in &unit_stats.capacity_cost {
                    unit.spawn()
                        .insert_bundle(world::CapacityResourceProsumerBundle {
                            player: world::OfPlayer(player_entity),
                            resource: *resource,
                            prosumer: world::CapacityResourceProsumer(*amount),
                        });
                }
            })
            .insert(ui::Selectable {})
            .id()
    }
}

#[derive(Component, Debug, Default)]
pub struct Unit {}

#[derive(Component, Clone, Copy, Default, Debug, EnumString, EnumIter, Display)]
pub enum UnitType {
    #[default]
    Skeleton,
    DeathKnight,
    GiantSpider,
}

#[derive(Debug)]
pub struct UnitStats {
    pub max_figures: usize,
    pub max_health: u32,
    pub cost: HashMap<world::StockpileResourceType, f32>,
    pub capacity_cost: HashMap<world::CapacityResourceType, i32>,
}

impl UnitType {
    pub fn get_unit_stats(&self) -> UnitStats {
        match self {
            UnitType::Skeleton => UnitStats {
                max_figures: 4,
                max_health: 4,
                cost: HashMap::from([(world::StockpileResourceType::Gold, 100.)]),
                capacity_cost: HashMap::from([(world::CapacityResourceType::Death, -1)]),
            },
            UnitType::DeathKnight => UnitStats {
                max_figures: 2,
                max_health: 10,
                cost: HashMap::from([(world::StockpileResourceType::Gold, 200.)]),
                capacity_cost: HashMap::from([(world::CapacityResourceType::Death, -1)]),
            },
            UnitType::GiantSpider => UnitStats {
                max_figures: 1,
                max_health: 20,
                cost: HashMap::from([(world::StockpileResourceType::Gold, 500.)]),
                capacity_cost: HashMap::from([(world::CapacityResourceType::Death, -1)]),
            },
        }
    }
}

#[derive(Component, Debug, Default)]
pub struct UnitOrders {
    orders: Vec<UnitOrder>,
}

impl UnitOrders {
    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }

    pub fn new_order(&mut self, order: UnitOrder) {
        let mut orders = vec![];
        if !self.orders.is_empty() && self.orders[0].is_interruptable() {
            self.orders.truncate(1);
        }
        orders.push(order);
        self.orders = orders;
    }

    pub fn insert_order(&mut self, order: UnitOrder) {
        self.orders.insert(0, order);
    }

    pub fn peek_order(&self) -> Option<&UnitOrder> {
        if self.orders.is_empty() {
            None
        } else {
            Some(&self.orders[0])
        }
    }

    pub fn next_order(&mut self) -> Option<UnitOrder> {
        if self.orders.is_empty() {
            None
        } else {
            Some(self.orders.remove(0))
        }
    }

    pub fn processed_order(&mut self) {
        if !self.orders.is_empty() {
            self.orders.remove(0);
        }
    }
}

#[derive(Debug)]
pub enum UnitOrder {
    Move {
        move_direction: MoveDirection,
        // 1-100
        progress: u32,
    },
    MoveToPosition {
        target_position: Position,
    },
}

impl UnitOrder {
    pub fn is_interruptable(&self) -> bool {
        match self {
            &UnitOrder::Move { progress, .. } => progress <= 25,
            UnitOrder::MoveToPosition { .. } => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumString)]
pub enum MoveDirection {
    NorthWest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
}

impl MoveDirection {
    pub fn cost(&self) -> f32 {
        match self {
            MoveDirection::North
            | MoveDirection::East
            | MoveDirection::South
            | MoveDirection::West => 1.,
            _ => 1.5,
        }
    }
}

type UnitOrdersQuery = (&'static mut UnitOrders, &'static mut Position);

pub fn unit_orders(
    game_tick_query: Query<ChangeTrackers<GameTick>>,
    mut unit_orders_query: Query<UnitOrdersQuery, With<Unit>>,
) {
    let game_tick_change_tracker = game_tick_query.single();
    if game_tick_change_tracker.is_changed() {
        unit_orders_query.for_each_mut(process_unit_orders)
    }
}

fn process_unit_orders((mut unit_orders, mut position): QueryItem<UnitOrdersQuery>) {
    while let Some(mut next_order) = unit_orders.next_order() {
        match next_order {
            UnitOrder::Move {
                move_direction,
                ref mut progress,
            } => {
                *progress += 25;
                if *progress >= 100 {
                    position.move_to_direction(&move_direction);
                } else {
                    unit_orders.insert_order(next_order);
                }
                break;
            }
            UnitOrder::MoveToPosition { target_position } => {
                if *position != target_position {
                    unit_orders.insert_order(next_order);
                    unit_orders.insert_order(UnitOrder::Move {
                        move_direction: position.direction_to(&target_position),
                        progress: 0,
                    });
                }
            }
        }
    }
}
