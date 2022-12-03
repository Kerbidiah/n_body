use std::{path::PathBuf, fmt::Debug};
use std::fs;

use anyhow::Ok;

use rand::rngs::ThreadRng;
use rand::thread_rng;

use rayon::prelude::*;

use ron::ser;

use super::Particle;

use crate::config::{DistributionMethod, my_config};


/// a trait to define what a random particle generator is
pub trait RandomParticleGen: Sync + Debug {
	/// generate a random particle with given settings
	fn gen(&self, rng: &mut ThreadRng) -> Particle;

	/// convert to `DistributionMethod`
	fn get_enum(&self) -> DistributionMethod;

	/// generate many `Particle`s with the given settings
	/// this function is automatically written by rust for anything structure that implements this trait
	fn gen_multi(&self, count: u16) -> Vec<Particle> {
		(0..count) // range to iterate through
			.par_bridge() // converts a normal iterator to a parrallel one
			.map(|_| { // _ means ignore the numbers we are iterating through
				// setup the random number generator (I think this is only done once per thread)
				let mut rng = thread_rng();

				self.gen(&mut rng)
			})
			.collect() // collect converts the iterator into a vector
	}

	/// deserializes a `RandomParticleGen` from the specified .ron file
	fn write(&self, path: PathBuf) -> anyhow::Result<()> {
		let contents = ser::to_string_pretty(&self.get_enum(), my_config())?;
		fs::write(path, contents)?;

		Ok(())
	}
}
