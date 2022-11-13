use serde::{Serialize, Deserialize};

use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;

use rayon::prelude::*;

use super::Particle; // super refers to the parent of this file which would be particles.rs
use super::tools::*;


/// structure to represent settings for randomly generating evently distibuted `Particle`s in a circular region.
#[derive(Debug, Serialize, Deserialize)]
pub struct PlainRandomGen {
	/// max distance from origin
	pub max_pos: f32,
	/// max speed
	pub max_vel: f32,
	/// setting this less than zero might cause some interesting results...
	pub min_mass: f32,
	pub max_mass: f32,
}


impl PlainRandomGen {
	/// make a new `PlainRandomGen` structure
    pub fn new(max_pos: f32, max_vel: f32, min_mass: f32, max_mass: f32) -> Self {
		Self {
			max_pos,
			max_vel,
			min_mass,
			max_mass
		}
	}

	/// generate a random particle with given settings
	pub fn gen(&self, rng: &mut ThreadRng) -> Particle {
		let pos = random_vec_full_circle(&mut rng, 0.0, self.max_pos); // &mut means we are passing a mutable reference
		let vel = random_vec_full_circle(&mut rng, 0.0, self.max_vel);
		let mass = rng.gen_range(self.min_mass..=self.max_mass);
		
		Particle::new(pos, vel, mass) // btw no semicolon means value is returned
	}
	
	/// generate many `Particle`s with the given settings
	// usize could be either a u32 or u64, whichever bit archetecture the code is compiled for
	pub fn gen_multi(&self, count: usize) -> Vec<Particle> {
		(0..count) // range to iterate through
			.par_bridge() // converts a normal iterator to a parrallel one
			.map(|_| { // _ means ignore the numbers we are iterating through
				// setup the random number generator (I think this is only done once per thread)
				let mut rng = thread_rng();

				self.gen(&mut rng)
			})
			.collect() // collect converts the iterator into a vector
	}
}