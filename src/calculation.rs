use crate::earth::EARTH_DIAMETER;
use bevy::prelude::*;
use std::f32::consts::E;

pub fn get_current_atmospheric_density(surface_altitude: f32) -> f32 {
    let (air_temp, air_pressure) = if surface_altitude >= 25000. {
        let temp = -131.21 + 0.00299 * surface_altitude;
        let pressure = 2.488 * ((temp + 273.1) / 216.6).powf(-11.388);
        (temp, pressure)
    } else if 11000. < surface_altitude && surface_altitude < 25000. {
        let temp = -56.46;
        let pressure = 22.65 * E.powf(1.73 - 0.000157 * surface_altitude);
        (temp, pressure)
    } else {
        let temp = 15.04 - 0.00649 * surface_altitude;
        let pressure = 101.29 * ((temp + 273.1) / 288.08).powf(5.256);
        (temp, pressure)
    };

    let air_density = air_pressure / (0.2869 * (air_temp + 273.1));

    air_density
}

pub fn get_current_altitude(current_position: Vec3) -> f32 {
    let vector_magnitude =
        (current_position.x.powi(2) + current_position.y.powi(2) + current_position.z.powi(2))
            .sqrt();
    let relative_altitude = (vector_magnitude - (EARTH_DIAMETER / 2.)).abs();

    relative_altitude
}

fn calculate_drag_acceleration(
    air_density: f32,
    coefficient_of_drag: f32,
    cross_sectional_area: f32,
    vehicle_mass: f32,
    velocity: f32,
) -> f32 {
    let acceleration: f32 =
        (coefficient_of_drag * air_density * velocity.powi(2) * cross_sectional_area)
            / (2. * vehicle_mass);

    acceleration
}

fn calculate_gravitational_acceleration(position: Vec3, vehicle_mass: f32) -> Vec3 {
    let gravitational_constant: f32 = 6.6743 * 10_f32.powi(-11); // [Nm^2/kg^2]
    let earth_mass: f32 = 5.97219 * 10_f32.powi(24); // [kg]

    let position_magnitude: f32 =
        (position.x.powi(2) + position.y.powi(2) + position.z.powi(3)).sqrt();
    let position_unit_vector = Vec3::new(
        position.x / position_magnitude,
        position.y / position_magnitude,
        position.z / position_magnitude,
    );

    // Calculate acceleration based off it's total distance
    let acceleration =
        (gravitational_constant * earth_mass * vehicle_mass) / position_magnitude.powi(2);

    // Give contributions of total acceleration back to unit vector
    let acceleration = Vec3::new(
        position_unit_vector.x * acceleration,
        position_unit_vector.y * acceleration,
        position_unit_vector.z * acceleration,
    );

    acceleration
}

pub fn calculate_cumulative_acceleration(
    air_density: f32,
    drag_coefficient: f32,
    cross_sectional_area: f32,
    object_mass: f32,
    velocity: Vec3,
    position: Vec3,
) -> Vec3 {
    let acceleration_drag = Vec3::new(
        calculate_drag_acceleration(
            air_density,
            drag_coefficient,
            cross_sectional_area,
            object_mass,
            velocity.x,
        ),
        calculate_drag_acceleration(
            air_density,
            drag_coefficient,
            cross_sectional_area,
            object_mass,
            velocity.y,
        ),
        calculate_drag_acceleration(
            air_density,
            drag_coefficient,
            cross_sectional_area,
            object_mass,
            velocity.z,
        ),
    );
    let acceleration_gravity = calculate_gravitational_acceleration(position, object_mass);

    let acceleration_total = Vec3::new(
        acceleration_drag.x + acceleration_gravity.x,
        acceleration_drag.y + acceleration_gravity.y,
        acceleration_drag.z + acceleration_gravity.z,
    );

    acceleration_total
}
