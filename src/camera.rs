use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 3000.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

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
        Name::new("Main Camera"),
    ));
}
