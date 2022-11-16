use std::f32::MAX;

use egui_macroquad;
use egui_macroquad::egui;
use egui_macroquad::egui::{widgets, Ui};

use crate::config::prelude::*;

#[derive(Debug, Clone)]
pub struct Persistance {
	kill_dist_enabled: bool,
	kill_dist: f32,
	pub method: DME
}

impl Persistance {
	/// maximum kill distance range
	fn max_kill_dist() -> f32 {
		MAX.sqrt()
	}

	pub fn new(rgs: &DM, s: &Settings) -> Self {
		Self {
			kill_dist_enabled: s.kill_dist.is_some(),
			kill_dist: s.kill_dist.unwrap_or_default(),
			method: rgs.corresponding(),
		}
	}

	pub fn update(&self, s: &mut Settings) {
		if self.kill_dist_enabled {
			s.kill_dist = Some(self.kill_dist);
		} else {
			s.kill_dist = None;
		}
	}

	pub fn param_edit(
		&mut self,
		ui: &mut Ui,
		s: &mut Settings,
		rgs: &mut DM
	) {
		ui.heading("Simulation Settings");
		egui::Grid::new("Settings Grid")
			.num_columns(2)
			.striped(true)
			.show(ui, |ui| {
				ui.label("time multiplier");
				ui.add(
					widgets::DragValue::new(&mut s.dt_multiplier)
						.clamp_range(0.0..=MAX)
						.speed(0.1)
				);
				ui.end_row();
			
				ui.label("simulations per frame");
				ui.add(
					widgets::DragValue::new(&mut s.sims_per_frame)
						.clamp_range(1..=std::u16::MAX)
						.speed(0.25)
				);
				ui.end_row();
			
				ui.label("# of particles");
				ui.add(
					widgets::DragValue::new(&mut s.count)
						.speed(0.0)
				);
				ui.end_row();
				
				ui.label("kill distance");
				ui.vertical(|ui| {
					ui.checkbox(&mut self.kill_dist_enabled, "enable");
					if self.kill_dist_enabled {
						ui.add(
							widgets::DragValue::new(&mut self.kill_dist)
								.clamp_range(0.0..=Self::max_kill_dist())
						);
					}
				});
			});
		
		ui.separator();
		ui.horizontal(|ui| {
			ui.label("particle distribution method: ");
			ui.selectable_value(&mut self.method, DME::Plain, "plain");
			ui.selectable_value(&mut self.method, DME::Belt, "belt");
		});

		self.update(s);
	}
}

