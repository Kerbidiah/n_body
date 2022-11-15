// #![windows_subsystem = "windows"] // make the terimnal not show up

use std::cell::RefCell;
use std::path::PathBuf;

use anyhow::Result;

use rayon::prelude::*;

use macroquad::camera::Camera2D;
use macroquad::color::colors;
use macroquad::math::Vec2;
use macroquad::prelude::*; // import everything in the macroquad prelude (stuff that is frequently used)

// let rust know that these "modules" are part of our code
pub mod controls;
pub mod particle;
pub mod physics;
pub mod config;

use particle::MinMax;
use particle::RandomParticleGen;

/// how many times faster simulation time is than real time
pub const DT_MULTIPLIER: f32 = 20.0;
/// how many physics loops are run per frame displayed
pub const SIMS_PER_FRAME: usize = 4;
/// how many particles are generated
pub const COUNT: usize = 1500;

#[macroquad::main("n-body simulation")] // macroquad entry point, also title of window
async fn main() -> Result<()> {
	// setup the camera
	let mut cam = Camera2D::default();
	cam.zoom *= 0.025;
	set_camera(&cam); // the & means we are passing cam as a reference, so we keep ownership of cam
	
	// load random particle distribution method
	let fname = PathBuf::from("plzwork.ron");
	let rand_gen_settings = config::load_distribution_method(fname)?;

	// generate the particles
	let mut bodies = rand_gen_settings.gen_multi(COUNT);
	bodies[0].pos = Vec2::ZERO; // move the first particle to the center
	bodies[0].vel = Vec2::ZERO; // set vel to 0
	bodies[0].mass = 50.0; // make it much heavier (kinda like a star)
	bodies[0].radius(); // recompute raidus since we changed mass

	// convert from vec<Particle> to Vec<RefCell<&mut Particle>>
	let mut bodies: Vec<RefCell<&mut particle::Particle>> = bodies
		.par_iter_mut()
		.map(|each| {
			RefCell::new(each)
		})
		.collect();

	// run collisions to get rid of all overlaping particles
	physics::collisions(&mut bodies);
	dbg!(bodies.len());

	// setup frame and time stuff
	let mut frame_counter: u64 = 0;
	let mut killed_by_dist: usize = 0;
	loop {
		// controls
		controls::zoom(&mut cam);
		controls::fix_aspect_ratio(&mut cam);

		physics::physics_loop(&mut bodies, SIMS_PER_FRAME, DT_MULTIPLIER);
		
		// lock the "star" in place
		// bodies[0].borrow_mut().pos = Vec2::ZERO;
		// bodies[0].borrow_mut().vel = Vec2::ZERO;
		
		// kill particles that have gone too far
		let prev = bodies.len();
		let star = bodies[0].borrow().clone();
		bodies.retain(|b| b.borrow().dist_sqrd(&star) <= 10_000.0);
		killed_by_dist += prev - bodies.len();
		
		bodies.iter().for_each(|b| {
			b.borrow().draw(colors::WHITE);
		});
		
		// print debug info every 30 frames
		if frame_counter % 30 == 0 {
			dbg!(get_fps());
			dbg!(bodies.len());
			dbg!(killed_by_dist);
		}
		
		frame_counter += 1;
		
		// advance to the next frame after 1/60th of a second has elapsed since previous frame
		// note: if you're screen has a higher refreshrate (like my laptop, 240Hz) it will instead
		// be 1/refresh_rate seconds, so 1/240th of a second for my laptop
		next_frame().await
	}

}
