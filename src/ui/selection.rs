use std::hash::Hash;

use bevy::{core::Stopwatch, utils::HashSet};

use crate::{game::map::Position, prelude::*, ui::EntityOnTile};

#[derive(Component, Debug, Default)]
pub struct Selectable {}

#[derive(Component, Debug, Default)]
pub enum SelectionInteractionState {
    #[default]
    None,
    Selecting,
    Selected,
}

#[derive(Component, Debug, Default)]
pub struct Selected(pub Selection);

#[derive(Debug, Default)]
pub struct Selection(SelectionType);

#[derive(Debug, Default)]
pub enum SelectionType {
    #[default]
    None,
    Single(SelectedEntity),
    Multiple(HashSet<SelectedEntity>),
}

#[derive(Debug, Clone, Copy, Eq)]
pub enum SelectedEntity {
    City(Entity),
    Unit(Entity),
}

impl Hash for SelectedEntity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.entity().hash(state);
    }
}

impl PartialEq for SelectedEntity {
    fn eq(&self, other: &Self) -> bool {
        self.entity().eq(other.entity())
    }
}
impl SelectedEntity {
    pub fn entity(&self) -> &Entity {
        match self {
            SelectedEntity::City(entity) | SelectedEntity::Unit(entity) => entity,
        }
    }
}

impl Selection {
    pub fn is_empty(&self) -> bool {
        matches!(self.0, SelectionType::None)
    }

    pub fn entities(&self) -> Vec<&SelectedEntity> {
        match self {
            Selection(SelectionType::Multiple(entities)) => entities.iter().collect(),
            Selection(SelectionType::Single(entity)) => vec![entity],
            Selection(SelectionType::None) => vec![],
        }
    }

    pub fn is_selected(&self, entity: Entity) -> bool {
        match &self.0 {
            SelectionType::Single(selected_entity) if *selected_entity.entity() == entity => true,
            SelectionType::Multiple(selected_entities)
                if selected_entities.contains(&SelectedEntity::Unit(entity)) =>
            {
                true
            }
            _ => false,
        }
    }

    pub fn is_selected_alone(&self, entity: Entity) -> bool {
        matches!(self.0, SelectionType::Single(selected_entity) if *selected_entity.entity() == entity)
    }

    pub fn clear(&mut self) {
        self.0 = SelectionType::None;
    }

    pub fn select_entities(&mut self, entities: Vec<&SelectedEntity>) {
        self.0 = match entities.len() {
            0 => SelectionType::None,
            1 => SelectionType::Single(*entities[0]),
            _ => SelectionType::Multiple(HashSet::from_iter(entities.into_iter().copied())),
        };
    }

    pub fn select_city(&mut self, entity: Entity) {
        self.0 = SelectionType::Single(SelectedEntity::City(entity));
    }

    pub fn select_unit(&mut self, entity: Entity) {
        self.0 = SelectionType::Single(SelectedEntity::Unit(entity));
    }

    pub fn select_units(&mut self, entities: Vec<Entity>) {
        self.0 = match entities.len() {
            0 => SelectionType::None,
            1 => SelectionType::Single(SelectedEntity::Unit(entities[0])),
            _ => SelectionType::Multiple(HashSet::from_iter(
                entities.into_iter().map(SelectedEntity::Unit),
            )),
        };
    }

    // add unit to a valid unit selection, ignores otherwise
    pub fn add_unit_to_selection(&mut self, entity: Entity) {
        match &self.0 {
            SelectionType::Multiple(selected_entities) => {
                let mut new_selected_entities = selected_entities.clone();
                new_selected_entities.insert(SelectedEntity::Unit(entity));
                self.0 = SelectionType::Multiple(new_selected_entities);
            }
            SelectionType::Single(selected_unit @ SelectedEntity::Unit(selected_entity))
                if *selected_entity != entity =>
            {
                let mut selected_entities = HashSet::new();
                selected_entities.insert(*selected_unit);
                selected_entities.insert(SelectedEntity::Unit(entity));
                self.0 = SelectionType::Multiple(selected_entities);
            }
            SelectionType::None => {
                self.0 = SelectionType::Single(SelectedEntity::Unit(entity));
            }
            _ => {}
        }
    }

    // remove unit from a valid unit selection, ignores otherwise
    pub fn remove_unit_from_selection(&mut self, entity: Entity) {
        match &self.0 {
            SelectionType::Multiple(selected_entities) => {
                let mut new_selected_entities = selected_entities.clone();
                new_selected_entities.remove(&SelectedEntity::Unit(entity));
                if new_selected_entities.len() <= 1 {
                    self.0 = SelectionType::Single(*selected_entities.iter().next().unwrap());
                } else {
                    self.0 = SelectionType::Multiple(new_selected_entities);
                }
            }
            SelectionType::Single(SelectedEntity::Unit(selected_entity))
                if *selected_entity == entity =>
            {
                self.0 = SelectionType::None;
            }
            _ => {}
        }
    }
}

#[derive(Component, Debug, Default)]
pub struct CursorTargetTime(pub Stopwatch);

#[derive(Component, Debug, Default)]
pub struct CursorSelectionTarget(pub Selection);

#[derive(Component, Debug, Default)]
pub struct CursorDragSelect(pub CursorDragSelectType);

#[derive(Debug, Default)]
pub enum CursorDragSelectType {
    #[default]
    None,
    Dragging(Vec2, Position, Selection),
}

#[derive(Component, Debug, Default)]
pub struct CursorDebugTooltipTarget {
    pub entities: Option<Vec<EntityOnTile>>,
}

#[derive(Bundle, Debug, Default)]
pub struct SelectableBundle {
    selectable: Selectable,
    interaction_state: SelectionInteractionState,
}
