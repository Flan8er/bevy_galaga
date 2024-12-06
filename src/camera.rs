use bevy::prelude::*;
use bevy_third_person_camera::*;

const CAMERA_DISTANCE: f32 = 80.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

// fn spawn_camera(mut commands: Commands) {
//     commands.spawn((
//         Camera3d::default(),
//         Transform::from_xyz(0.0, CAMERA_DISTANCE, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
//     ));
// }

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        ThirdPersonCamera {
            aim_enabled: true,
            aim_speed: 3.0, // default
            aim_zoom: 0.7,  // default
            offset_enabled: true,
            offset_toggle_enabled: true,
            gamepad_settings: CustomGamepadSettings { ..default() },
            zoom_enabled: true,          // default
            zoom: Zoom::new(15.0, 50.0), // default
            ..default()
        },
        // ThirdPersonCamera::default(),
    ));
}
