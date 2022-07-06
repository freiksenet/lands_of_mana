use bevy::{core::Stopwatch, utils::HashSet};

use crate::{prelude::*, ui::EntityOnTile};

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
    Unit(Entity),
    Units(HashSet<Entity>),
}

impl Selection {
    pub fn is_empty(&self) -> bool {
        matches!(self.0, SelectionType::None)
    }

    pub fn entities(&self) -> Vec<Entity> {
        match self {
            Selection(SelectionType::Units(entities)) => entities.iter().copied().collect(),
            Selection(SelectionType::Unit(entity)) => vec![*entity],
            Selection(SelectionType::None) => vec![],
        }
    }

    pub fn is_selected(&self, entity: Entity) -> bool {
        match &self.0 {
            SelectionType::Unit(selected_entity) if *selected_entity == entity => true,
            SelectionType::Units(selected_entities) if selected_entities.contains(&entity) => true,
            _ => false,
        }
    }

    pub fn is_selected_alone(&self, entity: Entity) -> bool {
        matches!(self.0, SelectionType::Unit(selected_entity) if selected_entity == entity)
    }

    pub fn clear(&mut self) {
        self.0 = SelectionType::None;
    }

    pub fn select_unit(&mut self, entity: Entity) {
        self.0 = SelectionType::Unit(entity);
    }

    pub fn select_units(&mut self, mut entities: Vec<Entity>) {
        self.0 = match entities.len() {
            0 => SelectionType::None,
            1 => SelectionType::Unit(entities[0]),
            _ => SelectionType::Units(HashSet::from_iter(entities.drain(..))),
        };
    }

    // add unit to a valid unit selection, ignores otherwise
    pub fn add_unit_to_selection(&mut self, entity: Entity) {
        match &self.0 {
            SelectionType::Units(selected_entities) => {
                let mut new_selected_entities = selected_entities.clone();
                new_selected_entities.insert(entity);
                self.0 = SelectionType::Units(new_selected_entities);
            }
            SelectionType::Unit(selected_entity) if *selected_entity != entity => {
                let mut selected_entities = HashSet::new();
                selected_entities.insert(*selected_entity);
                selected_entities.insert(entity);
                self.0 = SelectionType::Units(selected_entities);
            }
            SelectionType::None => {
                self.0 = SelectionType::Unit(entity);
            }
            _ => {}
        }
    }

    // remove unit from a valid unit selection, ignores otherwise
    pub fn remove_unit_from_selection(&mut self, entity: Entity) {
        match &self.0 {
            SelectionType::Units(selected_entities) => {
                let mut new_selected_entities = selected_entities.clone();
                new_selected_entities.remove(&entity);
                if new_selected_entities.len() <= 1 {
                    self.0 = SelectionType::Unit(*selected_entities.iter().next().unwrap());
                } else {
                    self.0 = SelectionType::Units(new_selected_entities);
                }
            }
            SelectionType::Unit(selected_entity) if *selected_entity == entity => {
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

#[derive(Debug, Clone, Default)]
pub enum CursorDragSelectType {
    #[default]
    None,
    Dragging(Vec2),
}

#[derive(Bundle, Debug, Default)]
pub struct CursorTargetBundle {
    pub target_time: CursorTargetTime,
    pub drag_select: CursorDragSelect,
    pub selection_target: CursorSelectionTarget,
    // pub interaction_target - what will happen if you right click
    // pub tooltip_target - what tooltip to show for this
    pub debug_tooltip: CursorDebugTooltipTarget,
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
