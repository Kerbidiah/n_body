use std::path::PathBuf;

use anyhow::{self, Result, Ok};

use egui_macroquad;
use egui_macroquad::egui;
use macroquad::prelude::*;

use crate::config::{Settings, DistributionMethod};
use crate::particle::RandomParticleGen;

mod credits;

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
		egui_macroquad::ui(|egui_ctx| {
			egui::Window::new("Welcome to n_body")
				.show(egui_ctx, |ui| {
					Link::disp_vec(&links, ui);
					credits::author(ui);
					ui.vertical_centered(|ui| {
						if ui.button("continue").clicked() {
							stay = false;
						}
					});
				});
		});

		egui_macroquad::draw();
		next_frame().await
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
	let mut settings = Settings::load(settings_path).unwrap();

	// load random particle distribution method
	let mut rgs = DistributionMethod::load(method_path).unwrap();

	// loop {


	// 	next_frame().await
	// }

	(settings, rgs)
}