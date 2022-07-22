use std::time::Duration;

use bevy::ecs::query::QueryItem;
use strum_macros::{EnumIter, EnumString};

use crate::{
    game::{
        map::{Map, Position},
        units::{Unit, UnitFigure, UnitType},
    },
    prelude::*,
    render::z_level::ZLevel,
};

pub struct RenderUnitsPlugin {}

impl Plugin for RenderUnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            config::Stage::UiSync,
            ConditionSet::new()
                .label_and_after(config::UiSyncLabel::Sync)
                .run_in_state(config::EngineState::InGame)
                .with_system(run_new_unit_position_to_transforms)
                .with_system(run_new_figures_spritesheet)
                .into(),
        )
        .add_system_set_to_stage(
            config::Stage::UiSync,
            ConditionSet::new()
                .label_and_after(config::UiSyncLabel::Update)
                .run_in_state(config::EngineState::InGame)
                .with_system(run_unit_position_to_transfors)
                .into(),
        );
    }
}

pub trait UnitSprite {
    fn get_default_tile_index(&self) -> usize;
    fn get_animation_frames(&self, animation_type: &FigureAnimationType) -> Vec<benimator::Frame>;
    fn get_figure_transforms(&self) -> Vec<Transform>;
}

impl UnitSprite for game::units::UnitType {
    fn get_default_tile_index(&self) -> usize {
        match self {
            game::units::UnitType::Skeleton => 216,
            game::units::UnitType::DeathKnight => 360,
            game::units::UnitType::GiantSpider => 264,
        }
    }

    fn get_animation_frames(&self, animation_type: &FigureAnimationType) -> Vec<benimator::Frame> {
        let start_tile = self.get_default_tile_index();
        let start_animation_tile = start_tile + *animation_type as usize;
        (start_animation_tile..start_animation_tile + 4)
            .map(|tile| benimator::Frame::new(tile, Duration::from_millis(150)))
            .collect()
    }

    fn get_figure_transforms(&self) -> Vec<Transform> {
        let max_figures = match self {
            game::units::UnitType::Skeleton => 4,
            game::units::UnitType::DeathKnight => 2,
            _ => 1,
        };
        let scale_amount = match self {
            game::units::UnitType::GiantSpider => 1.,
            _ => 0.5,
        };
        // those translations pretend that we have X figures on a 16x16 grid
        let mut figures_translations = match (self, max_figures) {
            (_, 2) => vec![Vec3::new(4., 0., 0.1), Vec3::new(4., 8., 0.)],

            (_, 4) => vec![
                Vec3::new(8., 8., 0.1),
                Vec3::new(8., 0., 0.2),
                Vec3::new(0., 8., 0.0),
                Vec3::new(0., 0., 0.1),
            ],

            (_, _) => vec![Vec3::new(0., 0., 0.)],
        };

        let scale_vec = Vec3::new(scale_amount, scale_amount, 1.);

        figures_translations
            .drain(..)
            .map(|figure_transaltion| {
                Transform::from_translation(figure_transaltion).with_scale(scale_vec)
            })
            .collect()
    }
}

#[derive(Clone, Copy, Debug, EnumString, EnumIter)]
pub enum FigureAnimationType {
    Idle = 0,
    Walk = 4,
    Attack = 8,
    Hit = 12,
    Death = 16,
}

type UnitPositionTransformQuery = (
    Entity,
    &'static Position,
    Option<&'static mut Transform>,
    Option<&'static GlobalTransform>,
);

pub fn run_unit_position_to_transfors(
    mut commands: Commands,
    map_query: Query<&game::map::Map>,
    mut units_query: Query<UnitPositionTransformQuery, (With<Unit>, Changed<Position>)>,
) {
    let map = map_query.single();
    units_query.for_each_mut(|unit_item| set_unit_transform(&mut commands, map, unit_item))
}

pub fn run_new_unit_position_to_transforms(
    mut commands: Commands,
    map_query: Query<&game::map::Map>,
    mut units_query: Query<UnitPositionTransformQuery, Added<Unit>>,
) {
    let map = map_query.single();
    units_query.for_each_mut(|unit_item| set_unit_transform(&mut commands, map, unit_item))
}

pub fn run_new_figures_spritesheet(
    mut commands: Commands,
    creatures: Res<assets::CreatureAssets>,
    figure_query: Query<(Entity, &UnitFigure, &UnitType), Added<UnitFigure>>,
) {
    for (figure_entity, figure, unit_type) in figure_query.iter() {
        let transforms = unit_type.get_figure_transforms();
        commands
            .entity(figure_entity)
            .insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: unit_type.get_default_tile_index(),
                    anchor: bevy::sprite::Anchor::BottomLeft,
                    ..Default::default()
                },
                texture_atlas: creatures.creatures.clone(),
                transform: *transforms
                    .get(figure.index)
                    .unwrap_or(&Transform::identity()),
                ..Default::default()
            });
    }
}

pub fn set_unit_transform(
    commands: &mut Commands,
    map: &Map,
    (unit_entity, position, transform_option, global_transform_option): QueryItem<
        UnitPositionTransformQuery,
    >,
) {
    let base_position = map
        .position_to_pixel_position(position)
        .extend(ZLevel::Units.into());
    match (transform_option, global_transform_option) {
        (Some(mut transform), Some(_)) => {
            transform.translation = base_position;
        }
        (_, _) => {
            commands.entity(unit_entity).insert_bundle(TransformBundle {
                local: Transform::from_translation(base_position),
                global: GlobalTransform::identity(),
            });
        }
    };
}
