use bevy::prelude::*;
use std::f32::consts::PI;

use crate::{
    asset_loader::SceneAssets, 
    earth::EARTH_DIAMETER, 
    movement::Rotatable, 
    calculation::{
        get_current_atmospheric_density, 
        get_current_altitude,
        calculate_cumulative_acceleration
    }
};

const CAPSULE_SCALE: f32 = 1.;
// pub const INITIAL_POSITION: Vec3 = Vec3::new((EARTH_DIAMETER / 2.) + 200_000., 0., 0.);
pub const INITIAL_POSITION: Vec3 = Vec3::new((EARTH_DIAMETER / 2.) + 125_000., 0., 0.);
pub const INITIAL_VELOCITY: Vec3 = Vec3::new(7555., 0., 0.);
pub const CAPSULE_MASS: f32 = 2.; // [kg]
pub const CAPSULE_RADIUS: f32 = 0.125; // [m]
pub const CAPSULE_DRAG_COEFFICIENT: f32 = 0.42; // Standard C_d for a half-sphere

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
            translation: INITIAL_POSITION, // Replace with initial position
            rotation: Quat::from_axis_angle(Vec3::new(1., 0., 0.), (-90. * std::f32::consts::PI) / 180.0),
            ..Default::default()
        },
        GlobalTransform::default(),
        Rotatable { speed: 0.05, axis: "y".to_string() },
        Name::new("Capsule"),
    ));
}

pub fn update_capsule_position(mut previous_state: ResMut<Capsule>, time: Res<Time>) {
    let delta_time: f32 = time.delta().as_secs() as f32;
    let cross_sectional_area: f32 = PI * CAPSULE_RADIUS.powi(2);

    // let position = previous_state.position;
    let velocity = previous_state.velocity;

    let altitude = get_current_altitude(position);

    // Don't calculate change if vehicle is on the ground.
    if altitude <= 0.0 {
        return
    }

    let air_density = get_current_atmospheric_density(altitude);

    let acceleration_total = calculate_cumulative_acceleration(
        air_density, 
        CAPSULE_DRAG_COEFFICIENT, 
        cross_sectional_area,
        CAPSULE_MASS,
        velocity,
        position
    );

    // New velocity is the velocity plus the change in velocity (acceleration times change in time).
    let update_velocity 

    // let mut updated_position = Vec3::ZERO;
    // let mut updated_velocity = Vec3::ZERO;

    // // Update the position from new calculated value.
    // updated_position.x = previous_position.x + 1.;
    // previous_state.position = updated_position;

    // // Update the position from new calculated value.
    // updated_velocity.x = previous_velocity.x + 1.;
    // previous_state.velocity = updated_velocity;
}

pub fn update_capsule_orientation() {}

// Schedule: Update capsule position, update capsule orientation, update camera focus
