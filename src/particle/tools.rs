use std::f32::consts::TAU; // 2*pi

use macroquad::math;

use rand::Rng;
use rand::rngs::ThreadRng;

/// generates a random angle between the given bounds in radians using the given `ThreadRng` generator
pub fn random_angle(rng: &mut ThreadRng, lo: f32, hi: f32) -> f32 {
	rng.gen_range(lo..hi) // ".." is an exclusive range (exclusive of right side)
}

/// generates a random angle from 0 to 2pi radians
pub fn random_angle_full_circle(rng: &mut ThreadRng) -> f32 {
	random_angle(rng, 0.0, TAU)
}

/// generates a random `Vec2` using polar bounds
pub fn random_vec(
	rng: &mut ThreadRng,
	min_radius: f32, max_radius: f32, // you can spread parameter definitions across multiple lines
	min_angle: f32, max_angle: f32
) -> math::Vec2 {
	let angle = random_angle(rng, min_angle, max_angle);
	let radius = rng.gen_range(min_radius..=max_radius);
	
	math::polar_to_cartesian(radius, angle)
}

/// generates a random `Vec2` using polar bounds with an angle from 0 to 2pi radians
pub fn random_vec_full_circle(
	rng: &mut ThreadRng,
	min_radius: f32, max_radius: f32
) -> math::Vec2 {
	let angle = random_angle_full_circle(rng);
	let radius = rng.gen_range(min_radius..=max_radius);
	
	math::polar_to_cartesian(radius, angle)
}