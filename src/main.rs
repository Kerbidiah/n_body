#![windows_subsystem = "windows"] // make the terminal not show up

use std::path::PathBuf;

use macroquad::camera::Camera2D;
use macroquad::math::Vec2;
use macroquad::prelude::*; // import everything in the macroquad prelude (stuff that is frequently used)


// let rust know that these "modules" are part of our code
pub mod controls;
pub mod particle;
pub mod physics;
pub mod config;
pub mod game_loop;
pub mod start_screen;
pub mod mov_avg;


/// macroquad configuration
pub fn window_conf() -> Conf {
	Conf {
		window_title: "n-body simulation".to_string(),
		window_width: 1920,
		window_height: 1080,
		high_dpi: true,
		..Default::default()
	}
}

/// main fucntion. where it all begins.
/// the `async` is required by macroquad
/// 
/// **ACADEMIC INTEGRITY STATEMENT**
/// I have not used source code obtained from any other unauthorized
/// source, either modified or unmodified.  Neither have I provided
/// access to my code to another. The project I am submitting
/// is my own original work.
#[macroquad::main(window_conf)] // macroquad entry point and configuration
async fn main() -> anyhow::Result<()> {
	let settings_path = PathBuf::from("settings.ron");
	let method_path = PathBuf::from("belt.ron");

	let (mut s, rgs) = start_screen::start_screen(settings_path, method_path).await;

	// setup the camera
	let mut cam = Camera2D::default();
	cam.zoom *= 0.025;
	set_camera(&cam); // the & means we are passing cam as a reference, so we keep ownership of cam

	// generate the particles
	let mut bodies = rgs.gen_multi(s.count);

	// modify the first particle to be like a star
	bodies[0].pos = Vec2::ZERO; // move to center
	bodies[0].vel = Vec2::ZERO;
	bodies[0].mass = 50.0;
	bodies[0].radius(); // recompute raidus since we changed mass

	game_loop::game_loop(&mut bodies, &mut cam, &mut s).await;

	Ok(())
}
