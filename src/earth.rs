use bevy::prelude::*;

use crate::movement::Rotatable;

use crate::asset_loader::SceneAssets;

pub const EARTH_DIAMETER: f32 = 12756274.; // [m]

pub fn spawn_earth(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        SceneRoot(scene_assets.earth.clone()),
        // Rotatable { speed: -0.005, axis: "y".to_string() },
        Transform {
            ..Default::default()
        },
        GlobalTransform::default(),
        Name::new("Earth"),
    ));
}
