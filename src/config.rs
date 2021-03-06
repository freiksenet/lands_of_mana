use bevy::ecs::system::AsSystemLabel;
use strum_macros::{EnumIter, EnumString};

use crate::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum EngineState {
    LoadingAssets,
    LoadingWorld,
    LoadingGraphics,
    InGame,
}

impl EngineState {
    pub fn next(&self) -> EngineState {
        match self {
            EngineState::LoadingAssets => EngineState::LoadingWorld,
            EngineState::LoadingWorld => EngineState::LoadingGraphics,
            EngineState::LoadingGraphics => EngineState::InGame,
            EngineState::InGame => EngineState::InGame,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, StageLabel)]
pub enum Stage {
    UiSync,
    GameTick,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, SystemLabel)]
pub enum UpdateStageLabel {
    UpdateCache, // Update cache entities after game tick
    Input,       // Recieve all input and send game actions
    GameActions, // Perform world changes based on input
}

impl OrderedLabel for UpdateStageLabel {
    fn after(&self) -> Option<UpdateStageLabel> {
        match self {
            UpdateStageLabel::UpdateCache => None,
            UpdateStageLabel::Input => Some(UpdateStageLabel::UpdateCache),
            UpdateStageLabel::GameActions => Some(UpdateStageLabel::Input),
        }
    }
}

impl<Marker> OrderedSystemLabel<Marker> for UpdateStageLabel where
    UpdateStageLabel: AsSystemLabel<Marker>
{
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, SystemLabel)]
pub enum UiSyncLabel {
    Sync,   // sync resource bindings for gui and update graphics components if needed
    Update, // do ui update
    Camera, // move camera
}

impl OrderedLabel for UiSyncLabel {
    fn after(&self) -> Option<UiSyncLabel> {
        match self {
            UiSyncLabel::Sync => None,
            UiSyncLabel::Update => Some(UiSyncLabel::Sync),
            UiSyncLabel::Camera => Some(UiSyncLabel::Update),
        }
    }
}

impl<Marker> OrderedSystemLabel<Marker> for UiSyncLabel where UiSyncLabel: AsSystemLabel<Marker> {}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, SystemLabel)]

pub enum GameTickStageLabel {
    Tick,            // Perform game tick update
    UpdateEntities,  // clean up time based things and things that need to removed
    UpdateResources, // update state of resources based on tick, incl upkeep
}

impl OrderedLabel for GameTickStageLabel {
    fn after(&self) -> Option<GameTickStageLabel> {
        match self {
            GameTickStageLabel::Tick => None,
            GameTickStageLabel::UpdateEntities => Some(GameTickStageLabel::Tick),
            GameTickStageLabel::UpdateResources => Some(GameTickStageLabel::UpdateEntities),
        }
    }
}

impl<Marker> OrderedSystemLabel<Marker> for GameTickStageLabel where
    GameTickStageLabel: AsSystemLabel<Marker>
{
}

pub trait OrderedLabel: Sized {
    fn after(&self) -> Option<Self>;
}

pub trait OrderedSystemLabel<Marker>: SystemLabel + OrderedLabel + AsSystemLabel<Marker> {}

pub trait LabelAndAfter {
    fn label_and_after<Marker>(self, label: impl OrderedSystemLabel<Marker>) -> Self;
}

impl LabelAndAfter for ConditionSet {
    fn label_and_after<Marker>(self, label: impl OrderedSystemLabel<Marker>) -> Self {
        if let Some(after_label) = label.after() {
            self.label(label).after(after_label)
        } else {
            self.label(label)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter, EnumString, Default)]
pub enum Direction {
    #[default]
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter, EnumString, Default)]
pub enum DirectionSides {
    #[default]
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter, EnumString, Default)]
pub enum DirectionCorners {
    #[default]
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter, EnumString, Default)]
pub enum DirectionSidesSymmetrical {
    #[default]
    North,
    East,
}
