use std::{collections::HashMap, iter::zip, time::Duration};

use benimator::*;
use strum_macros::{EnumIter, EnumString};

use crate::prelude::*;

pub trait UnitSprite {
    fn get_default_tile_index(&self) -> usize;
    fn get_animation_frames(&self, animation_type: &AnimationType) -> Vec<benimator::Frame>;
    fn get_figure_transforms(&self) -> Vec<Transform>;
}

impl UnitSprite for game::units::Unit {
    fn get_default_tile_index(&self) -> usize {
        match self.unit_type {
            game::units::UnitType::Skeleton => 216,
            game::units::UnitType::DeathKnight => 360,
            game::units::UnitType::GiantSpider => 264,
        }
    }

    fn get_animation_frames(&self, animation_type: &AnimationType) -> Vec<benimator::Frame> {
        let start_tile = self.get_default_tile_index();
        let start_animation_tile = start_tile + *animation_type as usize;
        (start_animation_tile..start_animation_tile + 4)
            .map(|tile| benimator::Frame::new(tile, Duration::from_millis(150)))
            .collect()
    }

    fn get_figure_transforms(&self) -> Vec<Transform> {
        let max_figures = match self.unit_type {
            game::units::UnitType::Skeleton => 4,
            game::units::UnitType::DeathKnight => 2,
            _ => 1,
        };
        let scale_amount = match self.unit_type {
            game::units::UnitType::GiantSpider => 1.,
            _ => 0.75,
        };
        // those translations pretend that we have X figures on a 16x16 grid
        let mut figures_translations = match (self.unit_type, max_figures) {
            (_, 2) => vec![Vec3::new(4., 0., 0.1), Vec3::new(4., 8., 0.)],

            (_, 4) => vec![
                Vec3::new(10., 10., 0.1),
                Vec3::new(10., 0., 0.2),
                Vec3::new(0., 10., 0.0),
                Vec3::new(0., 0., 0.1),
            ],

            (_, _) => vec![Vec3::new(0., 0., 0.)],
        };

        // compensation due to scale scaling into center
        let scale_translation_amount = (scale_amount * 16. - 16.) / 2.;
        let scale_translation = Vec3::new(scale_translation_amount, scale_translation_amount, 0.);

        // actual scaling
        let scale_vec = Vec3::new(scale_amount, scale_amount, 1.);

        figures_translations
            .drain(..)
            .map(|figure_transaltion| {
                Transform::from_scale(scale_vec)
                    .with_translation(figure_transaltion * scale_vec + scale_translation)
            })
            .collect()
    }
}

#[derive(Clone, Copy, Debug, EnumString, EnumIter)]
pub enum AnimationType {
    Idle = 0,
    Walk = 4,
    Attack = 8,
    Hit = 12,
    Death = 16,
}

pub fn setup(
    mut commands: Commands,
    creatures: Res<assets::CreatureAssets>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
    map_query: Query<&game::map::Map>,
    unit_query: Query<(Entity, &game::map::Position, &game::units::Unit)>,
    figure_query: Query<(Entity, &Parent), With<game::units::UnitFigure>>,
) {
    let map = map_query.single();
    let mut units_to_figures: HashMap<u32, Vec<Entity>> = HashMap::new();
    for (entity, parent) in figure_query.iter() {
        units_to_figures
            .entry(parent.id())
            .or_insert(Vec::new())
            .push(entity);
    }

    for (unit_entity_id, mut figure_entities) in units_to_figures.drain() {
        let unit_entity = Entity::from_raw(unit_entity_id);
        let (_, position, unit) = unit_query.get(unit_entity).unwrap();

        let base_position =
            map.position_to_pixel_position(position).extend(0.) + Vec3::new(0., 0., 75.);
        commands.entity(unit_entity).insert_bundle(TransformBundle {
            local: Transform::from_translation(base_position),
            global: GlobalTransform::identity(),
        });
        let animation_frames = &unit.get_animation_frames(&AnimationType::Idle);
        let transforms = unit.get_figure_transforms();

        for (figure_entity, transform) in zip(figure_entities.drain(..), transforms) {
            commands
                .entity(figure_entity)
                .insert_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: unit.get_default_tile_index(),
                        ..Default::default()
                    },
                    texture_atlas: creatures.creatures.clone(),
                    ..Default::default()
                })
                .insert_bundle(TransformBundle {
                    global: GlobalTransform::identity(),
                    local: transform,
                })
                .insert(
                    animations.add(SpriteSheetAnimation::from_frames(animation_frames.clone())),
                );
        }
    }
}

pub fn animations(
    mut commands: Commands,
    state: Res<CurrentState<game::InGameState>>,
    animatable_query: Query<Entity, With<game::units::UnitFigure>>,
) {
    if state.is_changed() {
        for entity in animatable_query.iter() {
            match state.0 {
                game::InGameState::Paused => {
                    commands.entity(entity).remove::<Play>();
                }
                game::InGameState::Running => {
                    commands.entity(entity).insert(Play);
                }
            };
        }
    }
}

pub fn selected(
    mut commands: Commands,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
    _unit_query: Query<(&game::units::Unit, &Children)>,
    figure_query: Query<(Entity, &Handle<SpriteSheetAnimation>), With<game::units::UnitFigure>>,
    selections_query: Query<
        (&game::units::Unit, &ui::Selectable, &Children),
        (With<game::units::Unit>, Changed<ui::Selectable>),
    >,
) {
    for (unit, selectable, children) in selections_query.iter() {
        for &child in children.iter() {
            if let Ok((figure, handle)) = figure_query.get(child) {
                commands
                    .entity(figure)
                    // .remove::<Handle<SpriteSheetAnimation>>()
                    .insert(
                        animations.set(
                            handle,
                            SpriteSheetAnimation::from_frames(
                                unit.get_animation_frames(match selectable.is_selected {
                                    true => &AnimationType::Death,
                                    false => &AnimationType::Idle,
                                })
                                .clone(),
                            ),
                        ),
                    );
            }
        }
    }
}
