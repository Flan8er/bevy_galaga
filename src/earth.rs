use bevy::prelude::*;

use crate::asset_loader::SceneAssets;

const EARTH_DIAMETER: f64 = 1.;

pub fn spawn_earth(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        SceneRoot(scene_assets.earth.clone()),
        Transform {
            scale: Vec3::splat(EARTH_DIAMETER as f32),
            ..Default::default()
        },
        GlobalTransform::default(),
        Name::new("Earth"),
    ));
}
