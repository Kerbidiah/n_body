use std::f32::consts::FRAC_PI_2; // pi / 2

use serde::{Deserialize, Serialize};

use rand::rngs::ThreadRng;

use macroquad::math::Vec2;

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
	pub center: Option<[f32; 2]>,
	/// distance from center
	pub radius: MinMax,
	/// speed
	pub vel: MinMax,
	/// tangent angle of velocity to position in **degrees**
	pub vel_angle: MinMax,
	/// direction of orbit
	pub direction: Direction,
	/// a negative mass might cause some interesting results...
	pub mass: MinMax,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Direction {
	/// clockwise
	CW,
	/// counter clockwise
	CCW,
}

impl BeltRandomGen {
	pub fn new(
		offset: Option<Vec2>,
		radius: MinMax,
		vel: MinMax,
		mass: MinMax,
		vel_angle: MinMax,
	) -> Self {
		Self {
			center: offset.map(|x| x.to_array()),
			radius,
			vel,
			vel_angle,
			direction: Direction::CCW,
			mass,
		}
	}

	/// returns the center offset as a `Vec2`
	fn offset(&self) -> Vec2 {
		self.center.map_or(Vec2::ZERO, |c| Vec2::from_slice(&c))
	}
}

impl RandomParticleGen for BeltRandomGen {
	fn gen(&self, rng: &mut ThreadRng) -> Particle {
		let pos = random_vec_full_circle(rng, self.radius) + self.offset();

		// find angle perpendicular to position
		let mut theta = Vec2::X.angle_between(pos);
		theta += match self.direction {
			Direction::CCW => FRAC_PI_2,
			Direction::CW => -1.0 * FRAC_PI_2,
		};

		// use angle to make velocity vector
		let vel = random_vec(rng, self.vel, self.vel_angle.radians().plus(theta));

		let mass = self.mass.inc_rand(rng);

		Particle::new(pos, vel, mass)
	}

	fn get_enum(&self) -> DistributionMethod {
		DistributionMethod::Belt(self.clone())
	}
}
