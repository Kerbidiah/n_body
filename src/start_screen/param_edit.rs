use std::f32::MAX;
use std::path::PathBuf;
use std::fs;
use std::env;

use anyhow;
use anyhow::Ok;

use egui_macroquad;
use egui_macroquad::egui;
use egui_macroquad::egui::{widgets, Ui};

use crate::config::prelude::*;

#[derive(Debug, Clone)]
pub struct Persistance {
	kill_dist_enabled: bool,
	kill_dist: f32,
	/// this is the current rand distribution settings
	pub rgs: Option<DM>,
	/// path to rand distribution settings
	path: PathBuf,
	/// how many frames untill files are refreshed
	file_refresh_timer: u16,
	files: Vec<PathBuf>,
}

impl Persistance {
	/// maximum kill distance range
	#[inline] // this is a recomendation to the compiler to inline this function
	fn max_kill_dist() -> f32 {
		MAX.sqrt()
	}

	/// create a new persistance struct with a bunch of default values.
	/// `init_path` is the initial path to the `DistributionMethod`	
	pub fn new(init_path: PathBuf, s: &Settings) -> Self {
		let rgs = DistributionMethod::load(init_path.clone()).ok();
		Self {
			kill_dist_enabled: s.kill_dist.is_some(),
			kill_dist: s.kill_dist.unwrap_or_default(),
			rgs,
			path: init_path,
			file_refresh_timer: 0,
			files: Vec::new(),
		}
	}

	/// check to see if files should be refreshed yet, if so, refresh them, otherwise, wait
	/// we don't check every single frame because system calls are slow
	/// NOTE: file refresh rate is linked to fps
	// 240 fps would refresh every 1/3rd of a second roughly
	// 60 fps would take more than a second between refreshes.
	pub fn refresh_files(&mut self) -> anyhow::Result<()> {
		if self.file_refresh_timer == 0 {
			self.file_refresh_timer = 100;

			let pwd = env::current_dir()?;
			self.files = fs::read_dir(pwd)?
				.map(|f| {
					f.expect("error reading specific directory").path()
				})
				.collect();
		} else {
			self.file_refresh_timer -= 1;
		}

		Ok(())
	}

	/// update kill distance enable/disable
	pub fn update_settings(&self, s: &mut Settings) {
		if self.kill_dist_enabled {
			s.kill_dist = Some(self.kill_dist);
		} else {
			s.kill_dist = None;
		}
	}

	/// UI for editing settings
	pub fn param_edit(
		&mut self,
		ui: &mut Ui,
		s: &mut Settings,
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
		
		self.update_settings(s);
	}

	/// UI for selecting distribuiton method config
	pub fn rand_edit(
		&mut self,
		ui: &mut Ui,
	) {
		self.refresh_files().unwrap();

		ui.heading("Particle Distribution Settings");
		ui.horizontal(|ui| {
			self.file_picker(ui);
			ui.separator();
			self.disp(ui);
		});
	}

	/// file picker UI
	fn file_picker(&mut self, ui: &mut Ui) {
		ui.vertical(|ui| {
			for each in self.files.clone() {
				let f_name = each
					.file_name().unwrap()
					.to_os_string()
					.into_string().unwrap();

				if ui.button(f_name).clicked() {
					self.rgs = dbg!(DM::load(each.clone())).ok(); // converts from result to option
					self.path = each;
				}
			}
		});
	}

	
	/// display the selected distribuion method
	fn disp(&mut self, ui: &mut Ui) {
		ui.vertical(|ui| {
			ui.label(format!("selected file: {}", self.path_name()));
			ui.separator();
			if let Some(value) = &self.rgs {
				ui.label(format!("value: {:?}", value));
			} else {
				ui.colored_label(egui::Color32::RED, "ERROR: not valid file");
			}
		});
	}
	
	/// get the path file name as a plain old String
	fn path_name(&self) -> String {
		// all this craziness converts from an &OsStr to a String
		self.path
		.file_name().unwrap()
		.to_os_string()
		.into_string().unwrap()
	}

	/// button to continue to simulation
	pub fn simulate_button(&self, ui: &mut Ui, stay: &mut bool) {
		// only shows up if selected method is valid
		if self.rgs.is_some() {
			ui.separator();
			ui.vertical_centered(|ui| {
				if ui.button("simulate").clicked() {
					*stay = false;

					// insert a loading spinner thing so the user knows the app is working and not stuck
					ui.spinner();
				}
			});
		}
	}
}