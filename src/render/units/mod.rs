use std::time::Duration;

use benimator::*;
use bevy::prelude::*;

use crate::assets::CreatureAssets;

pub fn setup(
    mut commands: Commands,
    creatures: Res<CreatureAssets>,
    mut animations: ResMut<Assets<SpriteSheetAnimation>>,
) {
    let scale_vec = Vec3::new(0.75, 0.75, 1.);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: creatures.creatures.clone(),
            transform: Transform::from_translation(Vec3::new(-4., -4., 50.)).with_scale(scale_vec),
            ..Default::default()
        })
        .insert(animations.add(SpriteSheetAnimation::from_range(
            24..=27,
            Duration::from_millis(150),
        )))
        .insert(Play);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: creatures.creatures.clone(),
            transform: Transform::from_translation(Vec3::new(4., -4., 50.)).with_scale(scale_vec),
            ..Default::default()
        })
        .insert(animations.add(SpriteSheetAnimation::from_range(
            24..=27,
            Duration::from_millis(150),
        )))
        .insert(Play);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: creatures.creatures.clone(),
            transform: Transform::from_translation(Vec3::new(4., 4., 50.)).with_scale(scale_vec),
            ..Default::default()
        })
        .insert(animations.add(SpriteSheetAnimation::from_range(
            24..=27,
            Duration::from_millis(150),
        )))
        .insert(Play);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: creatures.creatures.clone(),
            transform: Transform::from_translation(Vec3::new(-4., 4., 50.)).with_scale(scale_vec),
            ..Default::default()
        })
        .insert(animations.add(SpriteSheetAnimation::from_range(
            24..=27,
            Duration::from_millis(150),
        )))
        .insert(Play);
}
