// #![windows_subsystem = "windows"] // make the terimnal not show up

use std::path::PathBuf;

use anyhow;

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

#[macroquad::main("n-body simulation")] // macroquad entry point, also title of window
async fn main() -> anyhow::Result<()> {
	let settings_path = PathBuf::from("settings.ron");
	let method_path = PathBuf::from("belt.ron");

	let (s, rgs) = start_screen::start_screen(settings_path, method_path).await?;


	// setup the camera
	let mut cam = Camera2D::default();
	cam.zoom *= 0.025;
	set_camera(&cam); // the & means we are passing cam as a reference, so we keep ownership of cam
	

	// generate the particles
	let mut bodies = rgs.gen_multi(s.count);
	bodies[0].pos = Vec2::ZERO; // move the first particle to the center
	bodies[0].vel = Vec2::ZERO; // set vel to 0
	bodies[0].mass = 50.0; // make it much heavier (kinda like a star)
	bodies[0].radius(); // recompute raidus since we changed mass

	game_loop::game_loop(&mut bodies, &mut cam, s).await;

	Ok(())
}
