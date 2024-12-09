use bevy::prelude::*;

use crate::asset_loader::SceneAssets;

const CAPSULE_SCALE: f32 = 1.; //0.05;
pub const INITIAL_POSITION: Vec3 = Vec3::new(200_000., 0., 0.);
pub const INITIAL_VELOCITY: Vec3 = Vec3::new(7555., 0., 0.);

#[derive(Resource, Debug, Clone)]
pub struct Capsule {
    position: Vec3,
    velocity: Vec3,
}

impl Capsule {
    pub fn new(position: Vec3, velocity: Vec3) -> Self {
        Self {
            position: position,
            velocity: velocity,
        }
    }
}

pub fn spawn_capsule(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        SceneRoot(scene_assets.capsule.clone()),
        Transform {
            scale: Vec3::splat(CAPSULE_SCALE as f32),
            translation: Vec3::new(400., 400., 0.), // Replace with initial position
            ..Default::default()
        },
        GlobalTransform::default(),
        Name::new("Capsule"),
    ));
}

pub fn update_capsule_position(mut previous_state: ResMut<Capsule>) {
    // let previous_position = previous_state.position;
    // let previous_velocity = previous_state.velocity;

    // let mut updated_position = Vec3::ZERO;
    // let mut updated_velocity = Vec3::ZERO;

    // println!("{:#?}", previous_position);
    // println!("{:#?}", previous_velocity);

    // // Update the position from new calculated value.
    // updated_position.x = previous_position.x + 1.;
    // previous_state.position = updated_position;

    // // Update the position from new calculated value.
    // updated_velocity.x = previous_velocity.x + 1.;
    // previous_state.velocity = updated_velocity;
}
