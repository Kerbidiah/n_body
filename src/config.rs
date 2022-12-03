use std::path::PathBuf;
use std::fs;

use anyhow::Ok;

use ron;
use ron::ser::PrettyConfig;

use serde::{Deserialize, Serialize};

use crate::particle::{RandomParticleGen, PlainRandomGen, BeltRandomGen};


/// an enum to represent all the different types of random particle generation we have
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DistributionMethod {
	Plain(PlainRandomGen),
	Belt(BeltRandomGen),
}

/// same as `DistributionMethod`, but empty (to be used for the UI)
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum DistributionMethodEmpty {
	Plain,
	#[default]
	Belt,
}

/// return my custom serilization configuration
pub fn my_config() -> PrettyConfig {
	PrettyConfig::new()
	.struct_names(true)
	.indentor("\t".to_owned()) // spaces are dumb
	.new_line("\n".to_owned())
}

use DistributionMethod as DM; // alias for DistributionMethod
impl DistributionMethod {
	/// deserializes a `RandomParticleGen` from the specified .ron file.
	/// `dyn` means the type is a type that is determined at runtime time.
	/// the `RandomParticleGen` trait limits the possible types/structs to those that impl `RandomParticleGen`
	/// `Box` is needed since the size of the 2 structures are different, so they need to be heap allocated
	pub fn load_remove_enum(path: PathBuf) -> anyhow::Result<Box<dyn RandomParticleGen>> {
		let contents = Self::load(path)?;
		
		Ok(contents.strip_enum())
	}
	
	/// read a .ron file from disk and deserialize the contents to a `DistributionMethod` enum
	pub fn load(path: PathBuf) -> anyhow::Result<Self> {
		let file_bytes = fs::read(path)?;
		let contents = ron::de::from_bytes(&file_bytes)?;
		
		Ok(contents)
	}

	/// read a `&str` with the same format as a .ron and deserialize the contents to a `DistributionMethod` enum
	pub fn load_from_string(s: &str) -> anyhow::Result<Self> {
		let contents = ron::de::from_str(s)?;
		
		Ok(contents)
	}
		
	/// return the structure inside of the enum
	pub fn strip_enum(&self) -> Box<dyn RandomParticleGen> {
		let method: Box<dyn RandomParticleGen> = match self {
			Self::Plain(x) => Box::new(x.clone()),
			Self::Belt(x) => Box::new(x.clone()),
		};
		
		method
	}
	
	/// get the corresponding `DistributionMethodEmpty`
	pub fn corresponding(&self) -> DistributionMethodEmpty {
		match self {
			Self::Plain(_) => DME::Plain,
			Self::Belt(_) => DME::Belt,
		}
	}
	
	/// checks if `self` corresponds to `other`
	pub fn is_same(&self, other: DistributionMethodEmpty) -> bool {
		self.corresponding() == other
	}

	/// serialize `self` to the given file
	pub fn write(&self, path: PathBuf) -> anyhow::Result<()> {
		let contents = ron::ser::to_string_pretty(self, my_config())?;
		fs::write(path, contents)?;
		
		Ok(())
	}
}

impl Default for DistributionMethod {
	fn default() -> Self {
		Self::Belt(BeltRandomGen::default())
	}
}

use DistributionMethodEmpty as DME; // alias for DistributionMethodEmpty
impl DistributionMethodEmpty {
	/// get the corresponding `DistributionMethodEmpty`
	pub fn corresponding_default(&self) -> DistributionMethod {
		match self {
			DME::Plain => DM::Plain(PlainRandomGen::default()),
			DME::Belt => DM::Belt(BeltRandomGen::default()),
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
	/// how many times faster simulation time is than real time
	pub dt_multiplier: f32,
	/// how many physics loops are run per frame displayed
	pub sims_per_frame: u16,
	/// how many particles are generated
	pub count: u16,
	/// how far can particles can go before they are deleted
	pub kill_dist: Option<f32>,
}

impl Settings {
	/// deserialize the given file
	pub fn load(path: PathBuf) -> anyhow::Result<Self> {
		// read a .ron file and deserialize the contents
		let file_bytes = fs::read(path)?;
		Ok(ron::de::from_bytes(&file_bytes)?)
	}
	
	/// serialize to the given file
	pub fn write(&self, path: PathBuf) -> anyhow::Result<()> {
		let contents = ron::ser::to_string_pretty(self, my_config())?;
		fs::write(path, contents)?;
		
		Ok(())
	}
}

impl Default for Settings {
	/// if there was an issue reading the settings file, this default can be used as an alternative
	fn default() -> Self {
		Self {
			dt_multiplier: 20.0,
			sims_per_frame: 1,
			count: 1500,
			kill_dist: Some(100.0),
		}
	}
}


/// shorthand stuff
pub mod prelude {
	pub use super::Settings;
	pub use super::DistributionMethod;
	pub use super::DistributionMethodEmpty;
	pub use DistributionMethod as DM;
	pub use DistributionMethodEmpty as DME;
}

#[cfg(test)]
mod tests {
	use std::path::PathBuf;

	use super::*;

	use crate::particle;
	
	#[ignore]
	#[test]
	fn write_settings() {
		let s = Settings::default();
		
		let fname = PathBuf::from("settings.ron");
		s.write(fname).unwrap();
	}
	
	#[test]
	fn write_belt() {
		let rgs = DM::Belt(particle::BeltRandomGen::default());
		
		let fname = PathBuf::from("belt.ron");
		rgs.write(fname).unwrap();
	}
	
	#[test]
	fn write_plain() {
		let rgs = DM::Plain(particle::PlainRandomGen::default());
		
		let fname = PathBuf::from("plain.ron");
		rgs.write(fname).unwrap();
	}

	#[test]
	#[allow(unused_must_use)]
	fn test_load_settings() {
		let fname = PathBuf::from("settings.ron");
		let s = Settings::load(fname);
		dbg!(s);
	}
}
