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
/// stores the state of the UI and options and stuff so it can persist between frames
pub struct Persistance {
	kill_dist: f32,
	kill_dist_enabled: bool,
	/// this is the current rand distribution settings
	pub rgs: Option<DM>,
	/// path to rand distribution settings
	path: PathBuf,
	/// cache of the file specified in `self.path`
	/// this exists so we can show the file's contents in the UI w/o
	/// having to load it from disk every single frame
	rgs_cache: String,
	/// how many frames untill files are refreshed
	file_refresh_timer: u16,
	/// list of all `.ron` files that aren't `settings.ron`
	files: Vec<PathBuf>,
}

impl Persistance {
	/// maximum value for kill distance
	fn max_kill_dist() -> f32 {
		MAX.sqrt()
	}

	/// create a new persistance struct with a bunch of default values.
	/// `init_path` is the initial path to the `DistributionMethod`	(aka `DM`)
	pub fn new(init_path: PathBuf, s: &Settings) -> Self {
		Self {
			kill_dist: s.kill_dist.unwrap_or_default(),
			kill_dist_enabled: s.kill_dist.is_some(),
			rgs: DM::load(init_path.clone()).ok(), // `.ok()` converts from `Result` to `Option`
			path: init_path.clone(),
			rgs_cache: String::new(),
			file_refresh_timer: 0,
			files: Vec::new(),
		}
	}

	/// check to see if files should be refreshed yet, if so, refresh them, otherwise, wait
	/// we don't check every single frame because system calls are slow
	/// 
	/// NOTE: file refresh rate is linked to fps.
	/// 240 fps would refresh every 1/3rd of a second roughly.
	/// 60 fps would take more than a second between refreshes.
	/// 
	/// Could I fix this? yes. Will I fix this??? no.
	pub fn refresh_files(&mut self) -> anyhow::Result<()> {
		if self.file_refresh_timer == 0 {
			self.file_refresh_timer = 100;

			let pwd = env::current_dir()?;
			self.files = fs::read_dir(pwd)?
				.map(|f| {
					f.expect("error reading specific directory").path()
				})
				.collect();
			
			// only .ron files that aren't settings.ron
			self.files.retain(|f| f.extension().unwrap_or_default() == "ron");
			self.files.retain(|f| f.file_name().unwrap_or_default() != "settings.ron");
			
			self.update_rgs_and_cache();
		} else {
			self.file_refresh_timer -= 1;
		}

		Ok(())
	}

	/// update the cache and rgs
	fn update_rgs_and_cache(&mut self) {
		self.rgs_cache = fs::read_to_string(&self.path).unwrap_or("ERROR reading file".to_string());
		self.rgs = DM::load_from_string(&self.rgs_cache).ok();
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
		ui.horizontal(|ui| {
			ui.heading("Simulation Settings");
			ui.add_space(120.0);
			widgets::global_dark_light_mode_buttons(ui);
		});
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
			ui.group(|ui| {
				self.file_picker(ui);
			});
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
					self.path = each;
					self.update_rgs_and_cache();
				}
			}
		});
	}
	
	/// display the selected distribuion method
	fn disp(&mut self, ui: &mut Ui) {
		ui.vertical(|ui| {
			ui.label(format!("selected file: {}", self.path_name()));
			if self.rgs.is_some() { // check to make sure rgs is valid
				ui.add(
					egui::TextEdit::multiline(&mut self.rgs_cache)
					.font(egui::TextStyle::Monospace)
					.interactive(false)
				);
			} else {
				ui.separator();
				ui.colored_label(egui::Color32::LIGHT_RED, "ERROR: not valid file");
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