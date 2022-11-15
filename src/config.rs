use std::path::PathBuf;
use std::fs;

use anyhow::{Result, Ok};

use serde::{Deserialize, Serialize};
use ron;
use ron::ser::PrettyConfig;

use crate::particle::{RandomParticleGen, PlainRandomGen, BeltRandomGen};


#[derive(Debug, Serialize, Deserialize)]
pub enum DistributionMethod {
	Plain(PlainRandomGen),
	Belt(BeltRandomGen),
}

/// return a custom serilization configuration
pub fn my_config() -> PrettyConfig {
	PrettyConfig::new()
		.struct_names(true)
		.indentor("\t".to_owned())
		.new_line("\n".to_owned())
}

impl DistributionMethod {
	/// deserializes a `RandomParticleGen` from the specified .ron file
	pub fn load(path: PathBuf) -> Result<Box<dyn RandomParticleGen>> {
		// read a .ron file and deserialize the contents to a `DistributionMethod` enum
		let file_bytes = fs::read(path)?;
		let contents: DistributionMethod = ron::de::from_bytes(&file_bytes)?;
	
		let method: Box<dyn RandomParticleGen> = match contents {
			DistributionMethod::Plain(x) => Box::new(x),
			DistributionMethod::Belt(x) => Box::new(x),
		};
	
		Ok(method)
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
	/// how many times faster simulation time is than real time
	pub dt_multiplier: f32,
	/// how many physics loops are run per frame displayed
	pub sims_per_frame: usize,
	/// how many particles are generated
	pub count: u16,
	/// how far can particles go before they are deleted
	pub kill_dist: Option<f32>,
}

impl Settings {
	/// if there was an issue reading the settings file,
	/// this const can be used as an alternative
	pub const FALLBACK: Self = Settings {
		dt_multiplier: 20.0,
		sims_per_frame: 4,
		count: 1500,
		kill_dist: Some(100.0),
	};

	/// deserialize the given file into `Settings`
	pub fn load(path: PathBuf) -> Result<Self> {
		// read a .ron file and deserialize the contents
		let file_bytes = fs::read(path)?;
		Ok(ron::de::from_bytes(&file_bytes)?)
	}

	/// serialize `self` to the given file
	pub fn write(&self, path: PathBuf) -> Result<()> {
		let contents = ron::ser::to_string_pretty(self, my_config())?;
		fs::write(path, contents)?;

		Ok(())
	}
}


#[cfg(test)]
mod tests {
	use std::path::PathBuf;
	use super::*;

	#[ignore]
	#[test]
	fn write_settings() {
		let s = Settings {
			dt_multiplier: 20.0,
			sims_per_frame: 4,
			count: 1500,
			kill_dist: Some(100.0),
		};

		let fname = PathBuf::from("settings.ron");
		s.write(fname).unwrap();
	}

	#[test]
	#[allow(unused_must_use)]
	fn test_load_settings() {
		let fname = PathBuf::from("settings.ron");
		let s = Settings::load(fname);
		dbg!(s);
	}
}