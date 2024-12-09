use bevy::prelude::*;
use std::f32::consts::TAU;

use crate::asset_loader::SceneAssets;

pub const EARTH_DIAMETER: f32 = 12756274.;

#[derive(Component)]
pub struct Rotatable {
    speed: f32,
}


pub fn spawn_earth(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        SceneRoot(scene_assets.earth.clone()),
        Rotatable { speed: -0.005 },
        Transform {
            ..Default::default()
        },
        GlobalTransform::default(),
        Name::new("Earth"),
    ));
}

pub fn rotate_earth(mut cubes: Query<(&mut Transform, &Rotatable)>, timer: Res<Time>) {
    for (mut transform, cube) in &mut cubes {
        transform.rotate_y(cube.speed * TAU * timer.delta_secs());
    }
}

