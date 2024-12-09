use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use crate::capsule::INITIAL_POSITION;

const CAMERA_DISTANCE: f32 = 1.;
const CAMERA_FOV: f32 = 100.; // [deg]

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: (CAMERA_FOV * std::f32::consts::PI) / 180.0, // [rad]
            near: 0.1,
            far: 30.0,
            aspect_ratio: 1.77778,
        }),
        Transform::from_xyz(INITIAL_POSITION.x + CAMERA_DISTANCE, INITIAL_POSITION.y, INITIAL_POSITION.z).looking_at(INITIAL_POSITION, Vec3::Y),
        GlobalTransform::default(),
        PanOrbitCamera::default(),
        Name::new("Main Camera"),
    ));
}

pub fn update_camera_focus() {}
