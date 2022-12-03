use std::f32::consts::FRAC_PI_2; // pi/2

use macroquad::math::Vec2;

use rand::rngs::ThreadRng;

use serde::{Deserialize, Serialize};

use super::tools::*;
use super::Particle;
use super::RandomParticleGen;

use crate::config::DistributionMethod;


/// structure to represent settings for randomly generating `Particle`s in a belt like region.
/// The direction of their velocities is perpendicularly relative to their position
/// such that all the particles are travelling the same direction around the belt.
/// Its much like an asteriod belt.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BeltRandomGen {
	/// corrdinates of center of belt
	// can't use Vec2 because it doesn't derive `Serialize` and `Deserialize`
	pub center: [f32; 2],
	/// distance from center
	pub radius: MinMax,
	/// speed
	pub vel: MinMax,
	/// tangent angle of velocity to the circle around the `center` point in **degrees**
	pub vel_angle: MinMax,
	/// direction of orbit
	pub direction: Direction,
	pub mass: MinMax,
}

/// an enum to represent which direction the particles are traveling
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Direction {
	/// clockwise
	CW,
	/// counter clockwise
	CCW,
}

impl BeltRandomGen {
	/// create a new `BeltRandomGen` structure
	pub fn new(
		offset: Vec2,
		radius: MinMax,
		vel: MinMax,
		mass: MinMax,
		vel_angle: MinMax,
	) -> Self {
		Self {
			center: offset.to_array(),
			radius,
			vel,
			vel_angle,
			direction: Direction::CCW,
			mass,
		}
	}

	/// returns the center offset as a `Vec2`
	#[inline(always)] // strong *recomendation* to the compiler to inline this function
	fn offset(&self) -> Vec2 {
		Vec2::from_slice(&self.center)
	}
}

impl Default for Direction {
	/// counter clock-qise is the default
	fn default() -> Self {
		Self::CCW
	}
}

impl Direction {
	/// returns the angle between a tangential vector in the given direction and the radius in radians
	pub fn get_angle_offset(&self) -> f32 {
		match self {
			Direction::CCW => FRAC_PI_2, // FRAC_PI_2 is 90 degrees in radians
			Direction::CW => -1.0 * FRAC_PI_2,
		}
	}
}

impl RandomParticleGen for BeltRandomGen {
	fn gen(&self, rng: &mut ThreadRng) -> Particle {
		let pos = random_vec_full_circle(rng, self.radius) + self.offset();

		// find random angle perpendicular to position
		let mut theta = Vec2::X.angle_between(pos);
		theta += self.direction.get_angle_offset();

		// use angle to make velocity vector
		let vel = random_vec(rng, self.vel, self.vel_angle.radians().plus(theta));
		
		let mass = self.mass.inc_rand(rng);

		Particle::new(pos, vel, mass)
	}

	fn get_enum(&self) -> DistributionMethod {
		DistributionMethod::Belt(self.clone())
	}
}

// for documentation, see the `RandomParticleGen` trait documentation
impl Default for BeltRandomGen {
	/// implementation of the `Default` trait for `BeltRandomGen`
	fn default() -> Self {
		Self {
			center: [0.0, 0.0],
			radius: MinMax::new(3.0, 20.0),
			vel: MinMax::new(0.005, 0.1),
			vel_angle: MinMax::new(-5.0, 5.0),
			direction: Default::default(),
			mass: MinMax::new(0.01, 0.07)
		}
	}
}
