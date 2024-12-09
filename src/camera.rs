use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use crate::earth::EARTH_DIAMETER;

// const CAMERA_DISTANCE: f32 = EARTH_DIAMETER + 1000.;
const CAMERA_DISTANCE: f32 = 10.0;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: std::f32::consts::FRAC_PI_4,
            near: 0.1,
            far: 1000.0,
            aspect_ratio: 1.77778,
        }),
        Transform::from_xyz(CAMERA_DISTANCE, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        PanOrbitCamera::default(),
        Name::new("Main Camera"),
    ));
}

pub fn update_camera_focus() {}
