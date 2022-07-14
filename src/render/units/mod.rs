use std::time::Duration;

use bevy::ecs::query::QueryItem;
use bevy_pixel_camera::PixelProjection;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*, shapes::Polygon};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

use crate::{
    game::{
        map::{Map, Position},
        units::{Unit, UnitFigure, UnitType},
    },
    prelude::*,
    render::z_level::ZLevel,
    ui::{
        camera, CursorDragSelect, CursorDragSelectType, CursorSelectionTarget, CursorTargetTime,
        Selectable, Selected, Viewer,
    },
};

pub struct UnitsRenderPlugin {}

impl Plugin for UnitsRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ShapePlugin)
            .add_enter_system(config::EngineState::LoadingGraphics, setup_drag_selection)
            .add_system_set_to_stage(
                config::Stage::UiSync,
                ConditionSet::new()
                    .label_and_after(config::UiSyncLabel::Sync)
                    .run_in_state(config::EngineState::InGame)
                    .with_system(run_new_unit_position_to_transforms)
                    .with_system(run_new_figures_spritesheet)
                    .with_system(run_add_new_selection_box)
                    .with_system(run_selection_box_display_type)
                    .into(),
            )
            .add_system_set_to_stage(
                config::Stage::UiSync,
                ConditionSet::new()
                    .label_and_after(config::UiSyncLabel::Update)
                    .run_in_state(config::EngineState::InGame)
                    .with_system(run_unit_position_to_transfors)
                    .with_system(run_update_selection_box)
                    .with_system(update_drag_selection)
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

pub struct WithOrderDirectionDisplay {}

#[derive(Component, Debug)]
pub struct WithSelectionBox {
    pub selection_display_type: SelectionDisplayType,
    pub selection_box_entity: Entity,
}

#[derive(Component, Debug, Default)]
pub struct SelectionBox {}

#[derive(Bundle, Debug, Default)]
pub struct SelectionBoxBundle {
    selection_box: SelectionBox,
    #[bundle]
    transform: TransformBundle,
}

#[derive(Clone, Copy, Debug, EnumString, EnumIter, Default, PartialEq)]
pub enum SelectionDisplayType {
    #[default]
    None,
    Selected,
    Selecting,
    SelectedAndSelecting,
}

impl SelectionDisplayType {
    pub fn box_type_and_color(&self) -> (SelectionBoxColor, SelectionBoxType) {
        match self {
            SelectionDisplayType::Selected => (SelectionBoxColor::Green, SelectionBoxType::Dotted),
            SelectionDisplayType::Selecting => {
                (SelectionBoxColor::Green, SelectionBoxType::Brackets)
            }
            SelectionDisplayType::SelectedAndSelecting => {
                (SelectionBoxColor::Green, SelectionBoxType::Solid)
            }
            _ => (SelectionBoxColor::default(), SelectionBoxType::default()),
        }
    }
}

pub fn run_selection_box_display_type(
    viewer_query: Query<
        (
            &Selected,
            &CursorTargetTime,
            &CursorSelectionTarget,
            &CursorDragSelect,
            ChangeTrackers<Selected>,
            ChangeTrackers<CursorSelectionTarget>,
            ChangeTrackers<CursorDragSelect>,
        ),
        With<Viewer>,
    >,
    mut selection_box_display_query: Query<(Entity, &mut WithSelectionBox)>,
) {
    let (
        Selected(selection),
        _cursor_time,
        selection_target,
        cursor_drag_select,
        selection_tracker,
        selection_target_tracker,
        cursor_drag_select_tracker,
    ) = viewer_query.single();
    if selection_tracker.is_changed()
        || selection_target_tracker.is_changed()
        || cursor_drag_select_tracker.is_changed()
    {
        let selecting = match &cursor_drag_select.0 {
            CursorDragSelectType::Dragging(_, _, selection) => selection,
            _ => &selection_target.0,
        };
        for (entity, mut selection_box_display) in selection_box_display_query.iter_mut() {
            if selection.is_selected(entity) && selecting.is_selected(entity) {
                selection_box_display.selection_display_type =
                    SelectionDisplayType::SelectedAndSelecting;
            } else if selection.is_selected(entity) {
                selection_box_display.selection_display_type = SelectionDisplayType::Selected;
            } else if selecting.is_selected(entity) {
                selection_box_display.selection_display_type = SelectionDisplayType::Selecting;
            } else if selection_box_display.selection_display_type != SelectionDisplayType::None {
                selection_box_display.selection_display_type = SelectionDisplayType::None;
            }
        }
    }
}

// pub struct DirectionDisplay {

// }

#[derive(Clone, Copy, Debug, EnumString, EnumIter, Default)]
pub enum SelectionBoxColor {
    #[default]
    White = 0,
    Black = 2,
    Red = 4,
    Green = 6,
}

#[derive(Clone, Copy, Debug, EnumString, EnumIter, Default, PartialEq, Eq)]
pub enum SelectionBoxType {
    #[default]
    Solid = 0,
    Brackets = 16,
    Dotted = 32,
    Ticks = 48,
    Arrows = 64,
}

#[derive(Component, Clone, Copy, Debug, EnumString, EnumIter, Default)]
pub enum SelectionBoxCorner {
    #[default]
    NorthWest = 0,
    NorthEast = 1,
    SouthWest = 8,
    SouthEast = 9,
}

#[derive(Bundle)]
pub struct SelectionBoxCornerBundle {
    corner: SelectionBoxCorner,
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
}

impl SelectionBoxCornerBundle {
    pub fn new(
        selection_atlas: Handle<TextureAtlas>,
        corner: SelectionBoxCorner,
        selection_type: SelectionBoxType,
        selection_color: SelectionBoxColor,
    ) -> SelectionBoxCornerBundle {
        let scale_vec = Vec3::new(1., 1., 0.);
        SelectionBoxCornerBundle {
            corner,
            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: corner as usize + selection_type as usize + selection_color as usize,
                    anchor: bevy::sprite::Anchor::Center,
                    ..Default::default()
                },
                texture_atlas: selection_atlas,
                transform: Transform::from_scale(scale_vec).with_translation(match corner {
                    SelectionBoxCorner::NorthWest => {
                        Vec3::new(0., 16., ZLevel::UnitDecorations.into())
                    }
                    SelectionBoxCorner::NorthEast => {
                        Vec3::new(16., 16., ZLevel::UnitDecorations.into())
                    }
                    SelectionBoxCorner::SouthWest => {
                        Vec3::new(0., 0., ZLevel::UnitDecorations.into())
                    }
                    SelectionBoxCorner::SouthEast => {
                        Vec3::new(16., 0., ZLevel::UnitDecorations.into())
                    }
                }),
                visibility: Visibility { is_visible: false },
                ..Default::default()
            },
        }
    }
}

pub fn run_add_new_selection_box(
    mut commands: Commands,
    ui_assets: Res<assets::UiAssets>,
    selectable_query: Query<Entity, (With<Selectable>, Without<WithSelectionBox>)>,
) {
    for selected in selectable_query.iter() {
        let selection_type = SelectionBoxType::Solid;
        let selection_color = SelectionBoxColor::default();
        let selection_box = commands
            .spawn()
            .insert_bundle(SelectionBoxBundle::default())
            .with_children(|builder| {
                for corner in SelectionBoxCorner::iter() {
                    builder.spawn_bundle(SelectionBoxCornerBundle::new(
                        ui_assets.selectors.clone(),
                        corner,
                        selection_type,
                        selection_color,
                    ));
                }
            })
            .id();
        commands
            .entity(selected)
            .add_child(selection_box)
            .insert(WithSelectionBox {
                selection_box_entity: selection_box,
                selection_display_type: SelectionDisplayType::None,
            });
    }
}

pub fn run_update_selection_box(
    selection_box_type_query: Query<&WithSelectionBox, Changed<WithSelectionBox>>,
    selection_box_query: Query<&Children, With<SelectionBox>>,
    mut selection_box_corner_query: Query<(
        &SelectionBoxCorner,
        &mut TextureAtlasSprite,
        &mut Visibility,
    )>,
) {
    for WithSelectionBox {
        selection_box_entity,
        selection_display_type,
    } in selection_box_type_query.iter()
    {
        if let Ok(corners) = selection_box_query.get(*selection_box_entity) {
            for corner_entity in corners.iter() {
                if let Ok((corner, mut atlas_sprite, mut visibility)) =
                    selection_box_corner_query.get_mut(*corner_entity)
                {
                    let (selection_type, selection_color) =
                        selection_display_type.box_type_and_color();
                    atlas_sprite.index =
                        *corner as usize + selection_type as usize + selection_color as usize;
                    if *selection_display_type == SelectionDisplayType::None {
                        visibility.is_visible = false;
                    } else {
                        visibility.is_visible = true;
                    }
                }
            }
        }
    }
}

#[derive(Component, Debug, Default)]
pub struct DragSelectionBox {}

pub fn setup_drag_selection(mut commands: Commands, viewer_query: Query<Entity, With<Viewer>>) {
    let viewer = viewer_query.single();
    commands.entity(viewer).with_children(|builder| {
        builder
            .spawn_bundle(ShapeBundle {
                visibility: Visibility { is_visible: false },
                ..GeometryBuilder::new().build(
                    DrawMode::Stroke(StrokeMode::new(Color::GREEN, 1.0)),
                    Transform::from_translation(Vec3::new(0., 0., ZLevel::UnitDecorations.into())),
                )
            })
            .insert(DragSelectionBox::default());
    });
}

pub fn update_drag_selection(
    windows: Res<Windows>,
    camera_transform_query: Query<(&Camera, &Transform), With<PixelProjection>>,
    viewer_query: Query<(&CursorDragSelect, ChangeTrackers<CursorDragSelect>), With<Viewer>>,
    mut drag_selection_box: Query<(&mut Path, &mut Visibility), With<DragSelectionBox>>,
) {
    let (mut path, mut visibility) = drag_selection_box.single_mut();
    let (cursor_drag_select, cursor_drag_select_tracker) = viewer_query.single();
    if let CursorDragSelect(CursorDragSelectType::Dragging(drag_anchor_position, _, _)) =
        cursor_drag_select
    {
        let window = windows.get_primary().unwrap();
        let (camera, camera_transform) = camera_transform_query.single();
        if let Some(pixel_position) =
            camera::camera_position_to_pixel_position(window, camera, camera_transform)
        {
            let polygon = Polygon {
                points: vec![
                    pixel_position,
                    Vec2::new(pixel_position.x, drag_anchor_position.y),
                    *drag_anchor_position,
                    Vec2::new(drag_anchor_position.x, pixel_position.y),
                ],
                closed: true,
            };
            *path = ShapePath::build_as(&polygon);
            visibility.is_visible = true;
        }
    } else if cursor_drag_select_tracker.is_changed() {
        *path = ShapePath::new().build();
        visibility.is_visible = false;
    }
}
