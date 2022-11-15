use std::cell::RefCell;

use rayon::prelude::*;

use macroquad::camera::Camera2D;
use macroquad::color::colors;
use macroquad::prelude::*; // import everything in the macroquad prelude (stuff that is frequently used)

use crate::controls;
use crate::particle;
use crate::physics;
use crate::config;

use particle::Particle;


pub async fn game_loop(bodies: &mut Vec<Particle>, cam: &mut Camera2D, s: config::Settings) {
	// convert from Vec<Particle> to Vec<RefCell<&mut Particle>>
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
		controls::zoom(cam);
		controls::fix_aspect_ratio(cam);

		physics::physics_loop(&mut bodies, s.sims_per_frame, s.dt_multiplier);
		
		// lock the "star" in place
		// bodies[0].borrow_mut().pos = Vec2::ZERO;
		// bodies[0].borrow_mut().vel = Vec2::ZERO;
		
		// kill particles that have gone too far
		let prev = bodies.len();
		let star = bodies[0].borrow().clone();
		bodies.retain(|b| b.borrow().dist_sqrd(&star) <= 10_000.0); // TODO: kill dist
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
