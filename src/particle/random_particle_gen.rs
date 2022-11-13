use rand::thread_rng;
use rand::rngs::ThreadRng;

use rayon::prelude::*;

use super::Particle;


/// a trait to define what a random particle generator is
pub trait RandomParticleGen: Sync {

	/// generate a random particle with given settings
	fn gen(&self, rng: &mut ThreadRng) -> Particle;

	/// generate many `Particle`s with the given settings
	/// this function is automatically written by rust for anything structure that implements this trait
	// usize could be either a u32 or u64, whichever bit archetecture the code is compiled for
	fn gen_multi(&self, count: usize) -> Vec<Particle> {
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