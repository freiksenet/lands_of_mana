use std::time::Duration;

use benimator::*;
use bevy::prelude::*;

use strum_macros::{EnumIter, EnumString};

use crate::assets::CreatureAssets;
use crate::game;


pub trait UnitSprite {
    fn get_animation_frames(&self, animation_type: &AnimationType) -> Vec<benimator::Frame>;
    fn get_figure_transforms(&self, current_figures: usize) -> Vec<Transform>;
}

impl UnitSprite for game::units::Unit {
    fn get_animation_frames(&self, animation_type: &AnimationType) -> Vec<benimator::Frame> {
        let start_tile = match self.unit_type {
            game::units::UnitType::DebugBox => 0,
            game::units::UnitType::Skeleton => 216,
            game::units::UnitType::DeathKnight => 360,
            game::units::UnitType::GiantSpider => 264,
        };
        let start_animation_tile = start_tile + *animation_type as usize;
        (start_animation_tile..start_animation_tile + 4)
            .map(|tile| {
                benimator::Frame::new(
                    tile,
                    Duration::from_millis(if tile == start_animation_tile {
                        1000
                    } else {
                        150
                    }),
                )
            })
            .collect()
    }

    fn get_figure_transforms(&self, current_figures: usize) -> Vec<Transform> {
        let max_figures = match self.unit_type {
            game::units::UnitType::Skeleton => 4,
            game::units::UnitType::DeathKnight => 2,
            _ => 1,
        };
        let scale_amount = match self.unit_type {
            game::units::UnitType::DebugBox => 1.,
            game::units::UnitType::GiantSpider => 1.,
            _ => 0.75,
        };
        // those translations pretend that we have X figures on a 16x16 grid
        let mut figures_translations = match (self.unit_type, max_figures) {
            (_, 2) => {
                vec![Vec3::new(4., 0., 0.1), Vec3::new(4., 8., 0.)][..current_figures].to_vec()
            }
            (_, 4) => vec![
                Vec3::new(10., 10., 0.1),
                Vec3::new(10., 0., 0.2),
                Vec3::new(0., 10., 0.0),
                Vec3::new(0., 0., 0.1),
            ][..current_figures]
                .to_vec(),
            (game::units::UnitType::DebugBox, _) => vec![Vec3::new(0., 0., 0.)],
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
    creatures: Res<CreatureAssets>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
    world_query: Query<&game::map::GameWorld>,
    units: Query<(
        &game::map::Position,
        &game::units::Unit,
        &game::units::UnitFigures,
    )>,
) {
    let world = world_query.single();
    // TODO: centered transform
    let world_midpoint = Vec3::new(
        (world.width * 16) as f32 / 2. - 8.,
        (world.height * 16) as f32 / 2. - 8.,
        -75.,
    );
    for (position, unit, figures) in units.iter() {
        let base_position =
            Vec3::new((position.x * 16) as f32, (position.y * 16) as f32, 0.) - world_midpoint;
        let animation_frames = &unit.get_animation_frames(&AnimationType::Idle);
        let transforms = unit.get_figure_transforms(figures.health.len());
        for mut transform in transforms {
            transform.translation += base_position;
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: creatures.creatures.clone(),
                    transform,
                    ..Default::default()
                })
                .insert(animations.add(SpriteSheetAnimation::from_frames(animation_frames.clone())))
                .insert(Play);
        }
    }
}
