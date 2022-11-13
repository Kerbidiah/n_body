use std::f32::consts::TAU; // 2*pi

use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;

use rayon::prelude::*;

use macroquad::math;
// use macroquad::math::Vec2;

use super::Particle; // super refers to the parent of this file which would be particles.rs


/// structure to represent settings for randomly generating evently distibuted `Particle`s in a circular region.
#[derive(Debug)]
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
	pub fn gen(&self) -> Particle {
		let mut rng = thread_rng(); // setup the random number generator
		
		// generate random angles and radii for pos and vel (allows us to easily cap the magnitude)
		let pos_angle = random_angle(&mut rng); // &mut means we are passing a mutable reference
		let pos_rad = rng.gen_range(0.0..=self.max_pos); // "..=" is an inclusive range
		let vel_angle = random_angle(&mut rng);
		let vel_rad = rng.gen_range(0.0..=self.max_vel);
		
		let pos = math::polar_to_cartesian(pos_rad, pos_angle);
		let vel = math::polar_to_cartesian(vel_rad, vel_angle);
		let mass = rng.gen_range(self.min_mass..=self.max_mass);
		
		Particle::new(pos, vel, mass) // btw no semicolon means value is returned
	}
	
	/// generate many `Particle`s with the given settings
	// usize could be either a u32 or u64, whichever bit archetecture the code is compiled for
	pub fn gen_multi(&self, count: usize) -> Vec<Particle> {

		(0..count) // range to iterate through
			.par_bridge() // convert to a parrallel iterator
			.map(|_| { // _ means ignore the numbers we are iterating through
				let mut rng = thread_rng(); // setup the random number generator
				let pos_angle = random_angle(&mut rng); // &mut means we are passing a mutable reference
				let pos_rad = rng.gen_range(0.0..=self.max_pos); // "..=" is an inclusive range
				let vel_angle = random_angle(&mut rng);
				let vel_rad = rng.gen_range(0.0..=self.max_vel);
		
				let pos = math::polar_to_cartesian(pos_rad, pos_angle);
				let vel = math::polar_to_cartesian(vel_rad, vel_angle);
				let mass = rng.gen_range(self.min_mass..=self.max_mass);
		
				Particle::new(pos, vel, mass)
			})
			.collect() // collect converts the iterator into a vector
	}
}

/// generate a random angle in radians using the given `ThreadRng` generator
fn random_angle(rng: &mut ThreadRng) -> f32 {
	rng.gen_range(0.0..TAU)	// ".." is an exclusive range (exclusive of right side)
}
