use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

mod asset_loader;
mod calculation;
mod camera;
mod capsule;
mod earth;

use asset_loader::AssetLoaderPlugin;
use camera::CameraPlugin;
use capsule::CapsulePlugin;
use earth::EarthPlugin;

fn main() {
    App::new()
        // Bevy built-ins.
        .insert_resource(ClearColor(Color::srgb(0., 0., 0.)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(EarthPlugin)
        .add_plugins(CapsulePlugin)
        .run();
}

// // Reentry starts at 200,000[m] at 7,555[m/s]
