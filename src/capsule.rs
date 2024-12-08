use bevy::prelude::*;

use crate::asset_loader::SceneAssets;

const CAPSULE_SCALE: f32 = 1.; //0.05;

pub struct CapsulePlugin;

impl Plugin for CapsulePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_capsule)
            .add_systems(Update, update_capsule_position);
    }
}

pub fn spawn_capsule(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        SceneRoot(scene_assets.capsule.clone()),
        Transform {
            scale: Vec3::splat(CAPSULE_SCALE as f32),
            translation: Vec3::new(400., 400., 0.),
            ..Default::default()
        },
        GlobalTransform::default(),
        Name::new("Capsule"),
    ));
}

pub fn update_capsule_position() {}
