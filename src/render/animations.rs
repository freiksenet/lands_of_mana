use benimator;
use bevy::ecs::query::QueryItem;

use crate::{
    game::units::{Unit, UnitFigure, UnitOrder, UnitOrders, UnitType},
    prelude::*,
    render::units::{FigureAnimationType, UnitSprite},
};

pub struct AnimationsRenderPlugin {}

impl Plugin for AnimationsRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(benimator::AnimationPlugin::default())
            .add_system_set_to_stage(
                config::Stage::UiSync,
                ConditionSet::new()
                    .label_and_after(config::UiSyncLabel::Update)
                    .run_in_state(config::EngineState::InGame)
                    .with_system(run_animations)
                    .with_system(run_new_unit_animations)
                    .with_system(run_unit_animations)
                    .with_system(unit_animations_for_order)
                    .into(),
            );
    }
}

#[derive(Component, Debug)]
pub enum Animation {
    FigureAnimation { animation_type: FigureAnimationType },
}

fn run_animations(
    mut commands: Commands,
    state: Res<CurrentState<game::InGameState>>,
    animatable_query: Query<Entity, With<Animation>>,
) {
    if state.is_changed() {
        for entity in animatable_query.iter() {
            match state.0 {
                game::InGameState::Paused => {
                    commands.entity(entity).remove::<benimator::Play>();
                }
                game::InGameState::Running => {
                    commands.entity(entity).insert(benimator::Play);
                }
            };
        }
    }
}

type AnimatedFiguresQuery = (
    Entity,
    &'static UnitType,
    &'static Animation,
    Option<&'static Handle<benimator::SpriteSheetAnimation>>,
);

fn run_new_unit_animations(
    mut commands: Commands,
    animated_figures_query: Query<Entity, (With<UnitFigure>, Without<Animation>)>,
) {
    animated_figures_query.for_each(|entity| {
        commands.entity(entity).insert(Animation::FigureAnimation {
            animation_type: FigureAnimationType::Idle,
        });
    });
}

fn unit_animations_for_order(
    unit_query: Query<(&UnitOrders, &Children), (With<Unit>, Changed<UnitOrders>)>,
    mut animated_figures_query: Query<&mut Animation, With<UnitFigure>>,
) {
    for (orders, children) in unit_query.iter() {
        let new_animation_type = match orders.peek_order() {
            Some(UnitOrder::Move { .. }) | Some(UnitOrder::MoveToPosition { .. }) => {
                FigureAnimationType::Walk
            }
            // Some(_) |
            None => FigureAnimationType::Idle,
        };
        for child in children.iter() {
            if let Ok(Animation::FigureAnimation {
                ref mut animation_type,
            }) = animated_figures_query
                .get_mut(*child)
                .as_mut()
                .map(|m| m.as_mut())
            {
                *animation_type = new_animation_type;
            }
        }
    }
}

fn run_unit_animations(
    mut commands: Commands,
    mut animations: ResMut<Assets<benimator::SpriteSheetAnimation>>,
    animated_figures_query: Query<AnimatedFiguresQuery, (With<UnitFigure>, Changed<Animation>)>,
) {
    animated_figures_query.for_each(|item| set_animation(&mut commands, &mut animations, item));
}

fn set_animation(
    commands: &mut Commands,
    animations: &mut ResMut<Assets<benimator::SpriteSheetAnimation>>,
    (figure_entity, unit_type, Animation::FigureAnimation { animation_type }, handle_option): QueryItem<AnimatedFiguresQuery>,
) {
    let frames = unit_type.get_animation_frames(animation_type);
    let animation = benimator::SpriteSheetAnimation::from_frames(frames);
    commands.entity(figure_entity).insert(match handle_option {
        Some(handle) => animations.set(handle, animation),
        None => animations.add(animation),
    });
}
