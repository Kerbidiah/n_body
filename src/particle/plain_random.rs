use std::f32::MAX;
use serde::{Deserialize, Serialize};

use rand::rngs::ThreadRng;

use egui_macroquad;
use egui_macroquad::egui;
use egui_macroquad::egui::{widgets, Ui};

use super::tools::*;
use super::Particle; // super refers to the parent of this file which would be particle.rs
use super::RandomParticleGen;

use crate::config::DistributionMethod;

/// structure to represent settings for randomly generating evently distibuted `Particle`s in a circular region.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlainRandomGen {
	/// max distance from origin
	pub max_radius: f32,
	/// max speed
	pub max_vel: f32,
	/// a negative mass might cause some interesting results...
	pub mass: MinMax,
}

impl PlainRandomGen {
	/// make a new `PlainRandomGen` structure
	pub fn new(max_radius: f32, max_vel: f32, min_mass: f32, max_mass: f32) -> Self {
		Self {
			max_radius,
			max_vel,
			mass: MinMax::new(min_mass, max_mass),
		}
	}
	
	pub fn draw_editor(&mut self, ui: &mut Ui) {
		egui::Grid::new("plain config grid")
			.num_columns(2)
			.striped(true)
			.show(ui, |ui| {
				ui.label("max radius");
				ui.add(
					widgets::DragValue::new(&mut self.max_radius)
						.clamp_range(0.0..=MAX)
				);
				ui.end_row();
				
				ui.label("max velocity");
				ui.add(
					widgets::DragValue::new(&mut self.max_vel)
						.clamp_range(0.0..=MAX)
				);
				ui.end_row();
				
				// TODO: add other feilds
			});
	}
}

impl RandomParticleGen for PlainRandomGen {
	/// generate a random particle with given settings
	fn gen(&self, rng: &mut ThreadRng) -> Particle {
		let pos = random_vec_full_circle(rng, MinMax::new(0.0, self.max_radius));
		let vel = random_vec_full_circle(rng, MinMax::new(0.0, self.max_vel));
		let mass = self.mass.inc_rand(rng);

		Particle::new(pos, vel, mass) // btw no semicolon means value is returned
	}

	fn get_enum(&self) -> DistributionMethod {
		DistributionMethod::Plain(self.clone())
	}
}

#[allow(clippy::derivable_impls)]
impl Default for PlainRandomGen {
	fn default() -> Self {
		Self {
			max_radius: Default::default(),
			max_vel: Default::default(),
			mass: Default::default()
		}
	}
}