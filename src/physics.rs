use std::cell::RefCell;

use rayon::prelude::*;
use iter_tools::Itertools;
use macroquad::time;

use crate::particle::Particle;

/// execute the physics loop `steps_per` times.
/// this allows you to have multiple physics frames per graphical frame
// [T] means a "slice of type T". A slice is basically a more generic vector
pub fn physics_loop(bodies: &mut Vec<RefCell<&mut Particle>>, steps_per: usize, time_scaling: f32) {
	let dt = time::get_frame_time() / (steps_per as f32) * time_scaling;
	
	for _ in 0..steps_per {
		// calc gravity between each pair
		gravity(bodies);
		
		// move bodies
		bodies.par_iter_mut().for_each(|b| {
			b.borrow_mut().step(dt)
		});
		
		// calc collisions
		collisions(bodies);
		
		bodies.retain(|b| !b.borrow().collided);
	}
}

/// calculate gravity
fn gravity(bodies: &[RefCell<&mut Particle>]) {
	bodies.iter().combinations(2).for_each(|pair| {
		Particle::grav(pair[0], pair[1]);
	});
}

/// calculate collisions
pub fn collisions(bodies: &[RefCell<&mut Particle>]) {
	dbg!();
	bodies.iter().combinations(2).for_each(|pair| {
		Particle::check_collision(pair[0], pair[1]);
	});
}
