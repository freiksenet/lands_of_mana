use bevy::math::vec2;
use bevy_pixel_camera::PixelProjection;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*, shapes::Polygon};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

use crate::{
    config::Direction,
    prelude::*,
    render::z_level::ZLevel,
    ui::{
        camera, CursorDragSelect, CursorDragSelectType, CursorSelectionTarget, CursorTargetTime,
        Selectable, Selected, Viewer,
    },
};

pub struct RenderSelectionPlugin {}

impl Plugin for RenderSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ShapePlugin)
            .add_enter_system(config::EngineState::LoadingGraphics, setup_drag_selection)
            .add_system_set_to_stage(
                config::Stage::UiSync,
                ConditionSet::new()
                    .label_and_after(config::UiSyncLabel::Sync)
                    .run_in_state(config::EngineState::InGame)
                    .with_system(run_add_new_selection_box)
                    .with_system(run_selection_box_display_type)
                    .into(),
            )
            .add_system_set_to_stage(
                config::Stage::UiSync,
                ConditionSet::new()
                    .label_and_after(config::UiSyncLabel::Update)
                    .run_in_state(config::EngineState::InGame)
                    .with_system(run_update_selection_box)
                    .with_system(update_drag_selection)
                    .into(),
            );
    }
}

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
    pub fn box_type_and_color(&self) -> (SelectionBoxType, IndicatorColor) {
        match self {
            SelectionDisplayType::Selected => (SelectionBoxType::Dotted, IndicatorColor::Green),
            SelectionDisplayType::Selecting => (SelectionBoxType::Brackets, IndicatorColor::Green),
            SelectionDisplayType::SelectedAndSelecting => {
                (SelectionBoxType::Solid, IndicatorColor::Green)
            }
            _ => (SelectionBoxType::default(), IndicatorColor::default()),
        }
    }
}

fn run_selection_box_display_type(
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

#[derive(Clone, Copy, Debug, EnumString, EnumIter, Default)]
pub enum IndicatorColor {
    #[default]
    White,
    Black,
    Red,
    Green,
}

impl IndicatorColor {
    pub fn selection_color_offset(&self) -> usize {
        match self {
            IndicatorColor::White => 0,
            IndicatorColor::Black => 2,
            IndicatorColor::Red => 4,
            IndicatorColor::Green => 6,
        }
    }

    pub fn direction_color_offset(&self) -> usize {
        match self {
            IndicatorColor::White => 0,
            IndicatorColor::Black => 1,
            IndicatorColor::Red => 2,
            IndicatorColor::Green => 3,
        }
    }
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

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SelectionBoxCorner(DirectionCorners);

impl SelectionBoxCorner {
    pub fn texture_offset(&self) -> usize {
        match self.0 {
            DirectionCorners::NorthEast => 1,
            DirectionCorners::SouthEast => 9,
            DirectionCorners::SouthWest => 8,
            DirectionCorners::NorthWest => 0,
        }
    }
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
        selection_color: IndicatorColor,
    ) -> SelectionBoxCornerBundle {
        let scale_vec = Vec3::new(1., 1., 0.);
        SelectionBoxCornerBundle {
            corner,
            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: corner.texture_offset()
                        + selection_type as usize
                        + selection_color.selection_color_offset(),
                    anchor: bevy::sprite::Anchor::Center,
                    ..Default::default()
                },
                texture_atlas: selection_atlas,
                transform: Transform::from_scale(scale_vec).with_translation(match corner {
                    SelectionBoxCorner(DirectionCorners::NorthWest) => {
                        Vec3::new(0., 16., ZLevel::UnitDecorations.into())
                    }
                    SelectionBoxCorner(DirectionCorners::NorthEast) => {
                        Vec3::new(16., 16., ZLevel::UnitDecorations.into())
                    }
                    SelectionBoxCorner(DirectionCorners::SouthWest) => {
                        Vec3::new(0., 0., ZLevel::UnitDecorations.into())
                    }
                    SelectionBoxCorner(DirectionCorners::SouthEast) => {
                        Vec3::new(16., 0., ZLevel::UnitDecorations.into())
                    }
                }),
                visibility: Visibility { is_visible: false },
                ..Default::default()
            },
        }
    }
}

fn run_add_new_selection_box(
    mut commands: Commands,
    ui_assets: Res<assets::UiAssets>,
    selectable_query: Query<Entity, (With<Selectable>, Without<WithSelectionBox>)>,
) {
    for selected in selectable_query.iter() {
        let selection_type = SelectionBoxType::Solid;
        let selection_color = IndicatorColor::default();
        let selection_box = commands
            .spawn()
            .insert_bundle(SelectionBoxBundle::default())
            .with_children(|builder| {
                for corner in DirectionCorners::iter() {
                    builder.spawn_bundle(SelectionBoxCornerBundle::new(
                        ui_assets.selectors.clone(),
                        SelectionBoxCorner(corner),
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

fn run_update_selection_box(
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

                    atlas_sprite.index = corner.texture_offset()
                        + selection_type as usize
                        + selection_color.selection_color_offset();
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

fn setup_drag_selection(mut commands: Commands, viewer_query: Query<Entity, With<Viewer>>) {
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

fn update_drag_selection(
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
        if let Some(pixel_position) = camera::cursor_to_world(window, camera, camera_transform) {
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

#[derive(Component, Debug)]
pub struct WithDirectionIndicator {
    pub indicator: Entity,
    pub indicator_type: IndicatorType,
    pub direction: Direction,
    pub color: IndicatorColor,
}

#[derive(Component, Debug, Default)]
pub struct DirectionIndicator {}

#[derive(Debug, Default)]
pub enum IndicatorType {
    #[default]
    Triangle = 0,
    TriangleOutline = 4,
    Arrow = 8,
    ArrowOutline = 12,
}

#[derive(Bundle, Default)]
pub struct DirectionIndicatorBundle {
    indicator: DirectionIndicator,
    #[bundle]
    transform: TransformBundle,
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
}

impl DirectionIndicatorBundle {
    pub fn new(
        selection_atlas: Handle<TextureAtlas>,
        indicator_type: IndicatorType,
        direction: Direction,
        color: IndicatorColor,
    ) -> Self {
        let color_offset = color.direction_color_offset();
        let (direction_offset, translation, rotation) = match direction {
            Direction::North => (32, vec2(0., 0.), Quat::IDENTITY),
            Direction::NorthEast => (
                32,
                vec2(0., 0.),
                Quat::from_rotation_x(std::f32::consts::FRAC_PI_4),
            ),
            Direction::East => (48, vec2(0., 0.), Quat::IDENTITY),
            Direction::SouthEast => (
                48,
                vec2(0., 0.),
                Quat::from_rotation_x(std::f32::consts::FRAC_PI_4),
            ),
            Direction::South => (0, vec2(0., 0.), Quat::IDENTITY),
            Direction::SouthWest => (
                0,
                vec2(0., 0.),
                Quat::from_rotation_x(std::f32::consts::FRAC_PI_4),
            ),
            Direction::West => (16, vec2(0., 0.), Quat::IDENTITY),
            Direction::NorthWest => (
                16,
                vec2(0., 0.),
                Quat::from_rotation_x(std::f32::consts::FRAC_PI_4),
            ),
        };
        DirectionIndicatorBundle {
            transform: TransformBundle {
                local: Transform::from_translation(
                    translation.extend(ZLevel::OrderDirections.into()),
                )
                .with_rotation(rotation),
                ..Default::default()
            },
            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: color_offset + direction_offset + indicator_type as usize,
                    ..Default::default()
                },
                texture_atlas: selection_atlas,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
