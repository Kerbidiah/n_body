use std::cell::RefCell;

use macroquad::prelude::*; // import everything in the macroquad prelude (stuff that is frequently used)
use macroquad::color;
use macroquad::camera::Camera2D;
use macroquad::math::Vec2;

use rayon::prelude::*;

pub mod particle; // lets rust know that particle.rs is part of our code and imports it
pub mod controls;
pub mod physics;


/// how many times faster simulation time is than real time
pub const DT_MULTIPLIER: f32 = 2000.0;
pub const SIMS_PER_FRAME: usize = 4;
pub const COUNT: usize = 200;

#[macroquad::main("n-body simulation")] // macroquad entry point, also title of window
async fn main() {
	// setup the camera
	let mut cam = Camera2D::default();
	cam.zoom *= 0.025;
	set_camera(&cam); // the & means we are passing cam as a reference which means we keep ownership of cam

	// generate random particles
	let rand_gen_settings = particle::PlainRandomGen {
		max_pos: 10.0,
		max_vel: 0.0,
		min_mass: 0.1,
		max_mass: 7.0
	};

	let mut bodies = rand_gen_settings.gen_multi(COUNT);
	bodies[0].pos = Vec2::ZERO;
	bodies[0].vel = Vec2::ZERO;
	bodies[0].mass = 50.0;
	bodies[0].radius(); // recompute raidus since we changed mass
		
	
	// convert from vec<bodies> to vec<&mut RefCell<bodies>>
	let mut bodies: Vec<RefCell<&mut particle::Particle>> = bodies
		.par_iter_mut()
		.map(|each| {
			// each.vel *= 0.0;
			RefCell::new(each)
		})
		.collect();
	
	// run collisions to get rid of all overlaping particles
	physics::collisions(&mut bodies);

	// setup frame and time stuff	
	let mut frame_counter: u64 = 0;
	loop {
		// controls
		controls::zoom(&mut cam);
		
		physics::physics_loop(&mut bodies, SIMS_PER_FRAME, DT_MULTIPLIER);
				
		bodies.iter().for_each(|b| {
			b.borrow().draw(color::colors::WHITE);
		});

		// print debug info every 30 frames
		if frame_counter % 30 == 0 {
			dbg!(get_fps());
			dbg!(bodies.len());
		}
		
		frame_counter += 1;
		
		// advance to the next frame after 1/60th of a second has elapsed since previous frame
		// note: if you're screen has a higher refreshrate (like my laptop, 240Hz) it will instead
		// be 1/refresh_rate seconds, so 1/240th of a second for my laptop
		next_frame().await
	}
}
