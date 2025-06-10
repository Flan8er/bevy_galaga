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

pub fn run_reentry() -> App {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::srgb(0., 0., 0.)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy_canvas".into()),
                transparent: true,
                decorations: false,
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(AppPlugins);
    app
}
