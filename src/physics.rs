use std::cell::RefCell;

use iter_tools::Itertools;

use macroquad::time;

use rayon::prelude::*;

use crate::particle::Particle;

/// executes the physics loop `steps_per` times.
/// This allows you to have multiple physics simulations per graphical frame
pub fn physics_loop(bodies: &mut Vec<RefCell<&mut Particle>>, steps_per: u16, time_scaling: f32) {
	let dt = time::get_frame_time() / (steps_per as f32) * time_scaling; // time increment per simulation

	for _ in 0..steps_per {
		gravity(bodies);
		move_bodies(bodies, dt);
		collisions(bodies);
	}
}

/// calculate gravity
pub fn gravity(bodies: &[RefCell<&mut Particle>]) {
	bodies.iter().combinations(2).for_each(|pair| {
		Particle::grav(pair[0], pair[1]);
	});
}

/// calculate collisions
pub fn collisions(bodies: &mut Vec<RefCell<&mut Particle>>) {
	bodies.iter().combinations(2).for_each(|pair| {
		Particle::check_collision(pair[0], pair[1]);
	});

	bodies.retain(|b| !b.borrow().collided);
}

/// move bodies
pub fn move_bodies(bodies: &mut Vec<RefCell<&mut Particle>>, dt: f32) {
	bodies.par_iter_mut().for_each(|b| b.borrow_mut().step(dt));
}