use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

mod asset_loader;
mod calculation;
mod camera;
mod capsule;
mod earth;
mod movement;
mod plugins;

use plugins::AppPlugins;

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
        .add_plugins(AppPlugins)
        .run();
}
