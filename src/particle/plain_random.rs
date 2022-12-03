use rand::rngs::ThreadRng;

use serde::{Deserialize, Serialize};

use super::tools::*;
use super::Particle; // super refers to the parent of this file which would be particle.rs
use super::RandomParticleGen;

use crate::config::DistributionMethod;


/// structure to represent settings for randomly generating evently distibuted `Particle`s in a circular region.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PlainRandomGen {
	/// max distance from origin
	pub max_radius: f32,
	/// max speed
	pub max_vel: f32,
	pub mass: MinMax,
}

impl PlainRandomGen {
	/// make a new `PlainRandomGen` structure
	pub fn new(max_radius: f32, max_vel: f32, min_mass: f32, max_mass: f32) -> Self {
		Self {
			max_radius,
			max_vel,
			mass: MinMax::new(min_mass, max_mass),
		} // by the way, no semicolon means the value is returned
	}
}

// for documentation, see the `RandomParticleGen` trait documentation
impl RandomParticleGen for PlainRandomGen {
	fn gen(&self, rng: &mut ThreadRng) -> Particle {
		let pos = random_vec_full_circle(rng, MinMax::new(0.0, self.max_radius));
		let vel = random_vec_full_circle(rng, MinMax::new(0.0, self.max_vel));
		let mass = self.mass.inc_rand(rng);

		Particle::new(pos, vel, mass)
	}

	fn get_enum(&self) -> DistributionMethod {
		DistributionMethod::Plain(self.clone())
	}
}
