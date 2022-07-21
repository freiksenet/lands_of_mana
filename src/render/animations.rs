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
    unit_query: Query<(&UnitType, &UnitOrders, &Children), (With<Unit>, Changed<UnitOrders>)>,
    mut animated_figures_query: Query<
        (&UnitFigure, &mut Animation, &mut Transform),
        With<UnitFigure>,
    >,
) {
    for (unit_type, orders, children) in unit_query.iter() {
        let figure_transforms = UnitSprite::get_figure_transforms(unit_type);
        let (new_animation_type, offset) = match orders.peek_order() {
            Some(UnitOrder::Move {
                move_direction,
                progress,
            }) => (FigureAnimationType::Walk, {
                let multiply =
                    std::cmp::max_by(2., 10. * (*progress as f32) / 100., |l, r| l.total_cmp(r));
                let mut base_vector = match move_direction {
                    Direction::North => Vec2::new(0., 1.),
                    Direction::NorthEast => Vec2::new(1., 1.),
                    Direction::East => Vec2::new(1., 0.),
                    Direction::SouthEast => Vec2::new(1., -1.),
                    Direction::South => Vec2::new(0., -1.),
                    Direction::SouthWest => Vec2::new(-1., -1.),
                    Direction::West => Vec2::new(-1., 0.),
                    Direction::NorthWest => Vec2::new(-1., 1.),
                };

                base_vector *= multiply;
                base_vector
            }),
            Some(UnitOrder::MoveToPosition { .. }) => (FigureAnimationType::Walk, Vec2::ZERO),
            _ => (FigureAnimationType::Idle, Vec2::ZERO),
        };
        for child in children.iter() {
            if let Ok((UnitFigure { index }, mut animation, mut transform)) =
                animated_figures_query.get_mut(*child)
            {
                let Animation::FigureAnimation {
                    ref mut animation_type,
                } = animation.as_mut();
                *animation_type = new_animation_type;

                *transform = *figure_transforms
                    .get(*index)
                    .unwrap_or(&Transform::identity());
                transform.translation += offset.extend(0.);
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
