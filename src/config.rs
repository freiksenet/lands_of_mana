use bevy::ecs::system::AsSystemLabel;

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
    Input,       // Recieve all input and send game actions
    GameActions, // Perform world changes based on input
}

impl OrderedLabel for UpdateStageLabel {
    fn after(&self) -> Option<UpdateStageLabel> {
        match self {
            UpdateStageLabel::Input => None,
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
    Bindings, // sync resource bindings
    Update,   // do ui update
}

impl OrderedLabel for UiSyncLabel {
    fn after(&self) -> Option<UiSyncLabel> {
        match self {
            UiSyncLabel::Bindings => None,
            UiSyncLabel::Update => Some(UiSyncLabel::Bindings),
        }
    }
}

impl<Marker> OrderedSystemLabel<Marker> for UiSyncLabel where UiSyncLabel: AsSystemLabel<Marker> {}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, SystemLabel)]

pub enum GameTickStageLabel {
    UpdateEntities,  // clean up time based things and things that need to removed
    UpdateResources, // update state of resources based on tick, incl upkeep
    Tick,            // Perform game tick update
}

impl OrderedLabel for GameTickStageLabel {
    fn after(&self) -> Option<GameTickStageLabel> {
        match self {
            GameTickStageLabel::UpdateEntities => None,
            GameTickStageLabel::UpdateResources => Some(GameTickStageLabel::UpdateEntities),
            GameTickStageLabel::Tick => Some(GameTickStageLabel::UpdateResources),
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
