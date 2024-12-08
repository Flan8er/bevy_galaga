use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

const CAMERA_DISTANCE: f32 = 1500.0;
// const CAMERA_DISTANCE: f32 = 10.0;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: std::f32::consts::FRAC_PI_4,
            near: 0.1,
            far: 1000.0,
            aspect_ratio: 1.77778,
        }),
        Transform::from_xyz(0.0, CAMERA_DISTANCE, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        PanOrbitCamera::default(),
        Name::new("Main Camera"),
    ));
}

pub fn update_camera_focus() {}
