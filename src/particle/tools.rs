use std::f32::consts::TAU; // 2*pi
use std::ops::{Range, RangeInclusive};

use macroquad::math;
use macroquad::math::Vec2;

use serde::{Serialize, Deserialize};

use rand::Rng;
use rand::rngs::ThreadRng;

/// generates a random angle between the given bounds in radians using the given `ThreadRng` generator.
pub fn random_angle(rng: &mut ThreadRng, r: MinMax) -> f32 {
	r.exc_rand(rng)
}

const FULL_CIRCLE: MinMax = MinMax::new_unchecked(0.0, TAU);

/// generates a random angle from 0 to 2pi radians
pub fn random_angle_full_circle(rng: &mut ThreadRng) -> f32 {
	random_angle(rng, FULL_CIRCLE)
}

/// generates a random `Vec2` using polar bounds
pub fn random_vec(
	rng: &mut ThreadRng,
	radius: MinMax, // you can spread parameter definitions across multiple lines
	angle: MinMax
) -> Vec2 {
	let angle = random_angle(rng, angle);
	let radius = radius.inc_rand(rng);
	
	math::polar_to_cartesian(radius, angle)
}

/// generates a random `Vec2` using polar bounds with an angle from 0 to 2pi radians
pub fn random_vec_full_circle(rng: &mut ThreadRng, radius: MinMax) -> Vec2 {
	let angle = random_angle_full_circle(rng);
	let radius = radius.inc_rand(rng);
	
	math::polar_to_cartesian(radius, angle)
}


/// a structure to represent the min/max bounds of random generation
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct MinMax {
	pub min: f32,
	pub max: f32,
}

impl MinMax {
	/// create a new `Self` w/o checking that min < max
	/// this allows this function to be a constant function
	pub const fn new_unchecked(min: f32, max: f32) -> Self {
		Self {
			min,
			max,
		}
	}

	pub fn new(min: f32, max: f32) -> Self {
		assert!(min <= max);
		Self::new_unchecked(min, max)
	}

	/// return self but in radians
	pub fn radians(&self) -> Self {
		Self {
			min: self.min.to_radians(),
			max: self.min.to_radians()
		}
	}

	/// exclusive range, equivelent to `self.min..self.max`
	pub fn range_exc(&self) -> Range<f32> {
		self.min..self.max
	}

	/// inclusive range, equivelent to `self.min..=self.max`
	pub fn range_inc(&self) -> RangeInclusive<f32> {
		self.min..=self.max
	}

	/// generate a random number, exclusive
	pub fn exc_rand(&self, rng: &mut ThreadRng) -> f32 {
		if self.min == self.max { // if the range specifies a specific number
			return self.max;
		}
		rng.gen_range(self.range_exc())
	}

	/// generate a random number, inclusive
	pub fn inc_rand(&self, rng: &mut ThreadRng) -> f32 {
		rng.gen_range(self.range_inc())
	}

	/// return `self` with x added to both min and max
	pub fn plus(&self, x: f32) -> Self {
		Self {
			min: self.min + x,
			max: self.max + x,
		}
	}

	/// return `self` with x subtracted from both min and max
	pub fn minus(&self, x: f32) -> Self {
		Self {
			min: self.min - x,
			max: self.max - x,
		}
	}
}
