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
}

/// deserializes a `RandomParticleGen` from the specified .ron file
pub fn load_distribution_method(path: PathBuf) -> Result<Box<dyn RandomParticleGen>> {
	// read a .ron file and deserialize the contents to a `DistributionMethod` enum
	let file_bytes = fs::read(path)?;
	let contents: DistributionMethod = ron::de::from_bytes(&file_bytes)?;

	let method: Box<dyn RandomParticleGen> = match contents {
		DistributionMethod::Plain(x) => Box::new(x),
		DistributionMethod::Belt(x) => Box::new(x),
	};

	Ok(method)
}
