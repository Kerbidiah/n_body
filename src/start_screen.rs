use std::path::PathBuf;

use egui_macroquad;
use egui_macroquad::egui;
use egui::{Align2, Vec2}; // this Vec2 is an egui_macroquad::egui::Vec2 not a macroquad::math::Vec2

use macroquad::prelude::*;
use macroquad::camera::Camera2D;

use crate::config::prelude::*;
use crate::particle::RandomParticleGen;
use crate::controls;

pub mod credits;
pub mod param_edit;

use param_edit::Persistance;

/// display the splash screen then display the configuration screen
pub async fn start_screen(
	settings_path: PathBuf,
	method_path: PathBuf
) -> (Settings, Box<dyn RandomParticleGen>) {
	splash_screen().await;
	
	let mut cam = Camera2D::default();
	controls::fix_aspect_ratio(&mut cam); // do this now so the initial display later on is correct from frame 1
	
	config_screen(settings_path, method_path).await
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

/// display configuration screen.
/// Allows the user to select their random particle distribution method
/// and modify their settings.
async fn config_screen(
	settings_path: PathBuf,
	method_path: PathBuf
) -> (Settings, Box<dyn RandomParticleGen>) {

	// load settings
	let mut settings = Settings::load(settings_path).unwrap_or_default();

	// load random particle distribution method
	let mut p = Persistance::new(method_path, &settings);

	let mut stay = true;
	while stay {
		clear_background(BLACK);
		
		egui_macroquad::ui(|egui_ctx| {
			egui::Window::new("Settings and configuration")
				.anchor(Align2::CENTER_CENTER, Vec2::ZERO)
				.collapsible(false)
				.resizable(false)
				// .title_bar(false)
				.show(egui_ctx, |ui| {
					p.param_edit(ui, &mut settings);
					p.rand_edit(ui);
					p.simulate_button(ui, &mut stay);
				});
		});

		egui_macroquad::draw();
		next_frame().await;
	}

	(settings, p.rgs.unwrap().strip_enum())
}
