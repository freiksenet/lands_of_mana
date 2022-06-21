use bevy::{
    ecs::reflect::ReflectComponent,
    math::Mat4,
    prelude::{Bundle, Component, GlobalTransform, Transform},
    reflect::Reflect,
    render::{
        camera::{Camera, CameraProjection, DepthCalculation, ScalingMode, WindowOrigin},
        primitives::Frustum,
        view::VisibleEntities,
    },
};
use bevy_kayak_ui::CameraUiKayak;

#[derive(Bundle)]
pub struct UICameraBundle {
    pub camera: Camera,
    pub orthographic_projection: UIOrthographicProjection,
    pub visible_entities: VisibleEntities,
    pub frustum: Frustum,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub marker: CameraUiKayak,
}

impl UICameraBundle {
    pub fn new() -> Self {
        // we want 0 to be "closest" and +far to be "farthest" in 2d, so we offset
        // the camera's translation by far and use a right handed coordinate system
        let far = 1000.0;

        let orthographic_projection = UIOrthographicProjection {
            far,
            depth_calculation: DepthCalculation::ZDifference,
            window_origin: WindowOrigin::BottomLeft,
            ..Default::default()
        };

        let transform = Transform::from_xyz(0.0, 0.0, far - 0.1);

        let view_projection =
            orthographic_projection.get_projection_matrix() * transform.compute_matrix().inverse();
        let frustum = Frustum::from_view_projection(
            &view_projection,
            &transform.translation,
            &transform.back(),
            orthographic_projection.far(),
        );
        UICameraBundle {
            camera: Default::default(),
            orthographic_projection,
            frustum,
            visible_entities: VisibleEntities::default(),
            transform,
            global_transform: Default::default(),
            marker: CameraUiKayak,
        }
    }
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct UIOrthographicProjection {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
    pub window_origin: WindowOrigin,
    pub scaling_mode: ScalingMode,
    pub scale: f32,
    pub depth_calculation: DepthCalculation,
    pub zoom: i32,
}

impl CameraProjection for UIOrthographicProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh(
            self.left,
            self.right,
            self.bottom,
            self.top,
            // NOTE: near and far are swapped to invert the depth range from [0,1] to [1,0]
            // This is for interoperability with pipelines using infinite reverse perspective projections.
            self.far,
            self.near,
        )
    }

    fn update(&mut self, width: f32, height: f32) {
        let actual_width = width / (self.zoom as f32);
        let actual_height = height / (self.zoom as f32);

        self.left = 0.0;
        self.right = actual_width;
        self.bottom = actual_height;
        self.top = 0.0;
    }

    fn depth_calculation(&self) -> DepthCalculation {
        self.depth_calculation
    }

    fn far(&self) -> f32 {
        self.far
    }
}

impl Default for UIOrthographicProjection {
    fn default() -> Self {
        UIOrthographicProjection {
            left: -1.0,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,
            near: 0.0,
            far: 1000.0,
            window_origin: WindowOrigin::BottomLeft,
            scaling_mode: ScalingMode::None,
            scale: 1.,
            depth_calculation: DepthCalculation::ZDifference,
            zoom: 2,
        }
    }
}
