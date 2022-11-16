use std::path::PathBuf;

use anyhow::{self, Result, Ok};

use egui_macroquad;
use egui_macroquad::egui;
use egui::{Align2, Vec2}; // this Vec2 is an egui_macroquad::egui::Vec2 not a macroquad::math::Vec2

use macroquad::prelude::*;

use crate::config::prelude::*;
use crate::particle::RandomParticleGen;

pub mod credits;
pub mod param_edit;

use param_edit::Persistance;

/// display the splash screen then display the configuration screen
pub async fn start_screen(
	settings_path: PathBuf,
	method_path: PathBuf
) -> Result<(Settings, Box<dyn RandomParticleGen>)> {
	splash_screen().await;
	
	Ok(config_screen(settings_path, method_path).await)
}

pub async fn splash_screen() {
	use credits::Link;
	let links = Link::source_list();
	let mut stay = true;
	
	while stay {
		clear_background(BLACK);

		egui_macroquad::ui(|egui_ctx| {
			egui::Window::new("Welcome to n body")
				.anchor(Align2::CENTER_CENTER, Vec2::ZERO)
				.collapsible(false)
				.resizable(false)
				.show(egui_ctx, |ui| {
					Link::disp_vec(&links, ui); // display all the sources I used
					// TODO: display the libraries I used????
					credits::info(ui); // author and version section

					// button to continue
					ui.vertical_centered(|ui| {
						if ui.button("continue").clicked() {
							stay = false;
						}
					});
				});
		});

		egui_macroquad::draw();
		next_frame().await;
	}
}

/// display configuration screen
/// allows the user to select their random particle distribution method
/// and modify their settings
async fn config_screen(
	settings_path: PathBuf,
	method_path: PathBuf
) -> (Settings, Box<dyn RandomParticleGen>) {

	// load settings
	let mut settings = Settings::load(settings_path).unwrap_or_default();

	// load random particle distribution method
	let mut rgs = DistributionMethod::load(method_path).unwrap_or_default();
	let mut p = Persistance::new(&rgs, &settings);

	let mut stay = true;
	while stay {
		clear_background(BLACK);

		egui_macroquad::ui(|egui_ctx| {
			egui::Window::new("Settings and configuration")
				.anchor(Align2::CENTER_CENTER, Vec2::ZERO)
				.collapsible(false)
				.resizable(false)
				.show(egui_ctx, |ui| {
					p.param_edit(
						ui,
						&mut settings,
						&mut rgs
					);

					// button to continue to simulation
					ui.vertical_centered(|ui| {
						ui.separator();
						if ui.button("simulate").clicked() {
							stay = false;

							// insert a loading spinner thing so the user knows the app is working and not stuck
							ui.spinner();
						}
					});
				});
		});

		// if random generation method was changed, reset RGS to default
		if !rgs.is_same(p.method) {
			rgs = p.method.corresponding_default();
		}

		egui_macroquad::draw();
		next_frame().await;
	}

	(settings, rgs.strip_enum())
}