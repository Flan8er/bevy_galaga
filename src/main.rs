use bevy::prelude::*;

mod asset_loader;
mod calculation;
mod camera;
mod earth;

use asset_loader::AssetLoaderPlugin;
use camera::CameraPlugin;
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
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(EarthPlugin)
        .run();
}

// // Reentry starts at 200,000[m] at 7,555[m/s]
