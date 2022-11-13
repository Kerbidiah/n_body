use std::f32::consts::*;
use std::cell::RefCell;

use macroquad::math::Vec2; // import 2d vector from macroquad (note: macroquad is actually using glam)
use macroquad::{shapes, color};

pub mod random; // let rust know that particle/random.rs is related to this file
pub use random::PlainRandomGen; // any use of particle.rs will automatically use particle/random.rs aswell



/// a structure to represent a particle
#[derive(Debug, Clone)] // tell rust to write the code so we can print this structure for debugging purposes
pub struct Particle {
	pub pos: Vec2,
	pub vel: Vec2,
	accel: Vec2, // not having pub makes this feild private
	pub mass: f32,
	/// store radius to avoid recomputing it for every collison check
	/// it is only recomputed when mass changes
	radius: f32,
	/// allows delaying particle deletion untill a later point in the game loop
	pub collided: bool,
}

impl Particle { // functions for particles

	/// gravitational constant
	pub const G: f32 = 6.6743e-11;

	/// creates a new `Particle` w/ given parameters and `colided = false`
	pub fn new(pos: Vec2, vel: Vec2, mass: f32) -> Self {
		let mut x = Self {
			pos,
			vel,
			accel: Vec2::ZERO,
			mass,
			radius: 0.0,
			collided: false,
		};
		x.radius();
		x
	}

	
	/// add acceleration
	pub fn add_accel(&mut self, val: Vec2) {
		self.accel += val;
	}
	
	/// adjusts position and velocity according to the given acceleration and time step
	// "&mut self" means this function takes a mutable reference to a Particle
	pub fn step(&mut self, dt: f32) {
		self.pos += (0.5 * self.accel * dt.powi(2)) + (self.vel * dt);
		self.vel += self.accel * dt;
		
		// reset acceleration back to 0
		self.accel = Vec2::ZERO;
	}

	/// finds the difference in position between `self` and `other`
	pub fn diff(&self, other: &Self) -> Vec2 {
		other.pos - self.pos
	}

	/// this is probably faster than squaring then dividing
	pub fn diff_neg2pow(&self, other: &Self) -> Vec2 {
		self.diff(other).powf(-2.0)
	}
	
	/// finds the distance from `self` to `other`
	pub fn dist(&self, other: &Self) -> f32 {
		self.pos.distance(other.pos)
	}

	/// finds the distance squared from `self` to `other`. This is faster and more accurate than `dist`.
	pub fn dist_sqrd(&self, other: &Self) -> f32 {
		self.pos.distance_squared(other.pos)
	}
	
	/// find the momentum vector
	pub fn momentum(&self) -> Vec2 {
		self.vel * self.mass
	}

	/// apply gravitational acceleration to the given `Particles`
	pub fn grav(a: &RefCell<&mut Self>, b: &RefCell<&mut Self>) {
		let mag = Self::G / (a.borrow().dist_sqrd(&b.borrow()));
		let dirc = a.borrow().diff(&b.borrow()).normalize();
		a.borrow_mut().add_accel(b.borrow().mass * mag * dirc);
		b.borrow_mut().add_accel(a.borrow().mass * mag * dirc * -1.0);
	}
	
	/// check collision for 2 particles.
	/// if collision occurs, the following is done:
	/// New velocity is determined using the momentums of the 2 particles.
	/// New mass is sum of the masses of the particles
	/// `b.collided` is set to `true`
	pub fn check_collision(a: &RefCell<&mut Self>, b: &RefCell<&mut Self>) {
		let collision_dist = (a.borrow().radius + a.borrow().radius).powi(2); // we square because thats faster than doing a sqrt on distance
		
		if a.borrow().dist_sqrd(&b.borrow()) <= collision_dist {
			
		}
		
		let new_momentum = a.borrow().momentum() + b.borrow().momentum();
		a.borrow_mut().mass += b.borrow().mass;
		a.borrow_mut().radius();
		a.borrow_mut().vel = new_momentum / a.borrow().mass;

		b.borrow_mut().collided = true;
	}

	/// multiplier to adjust size of particles
	const SIZE_MULTIPLIER: f32 = 0.05;

	/// returns how big the object should be on screen and for collision detection.
	/// The radius is determined by finding the radius of a sphere with a volume of `mass`
	pub fn radius(&mut self) {
		// FRAC_1_PI is 1/pi, multiplying by that is faster than dividing by pi, and is accurate enough
		self.radius = (0.75 * self.mass * FRAC_1_PI * Self::SIZE_MULTIPLIER).cbrt(); // cbrt is cube root
	}
	
	/// draws particle on screen with given color relative to center of the screen.
	/// x and y axis are adjusted to work the same way they do in a typical 2d coordinate plane.
	pub fn draw(&self, color: color::Color) {
		shapes::draw_circle(self.pos.x, self.pos.y, self.radius, color);
	}
}

// unit testing, this is only compiled and run when "cargo test" is run in the terminal
#[cfg(test)]
mod tests {
	use super::*;

	fn grav_test(i: i32, dt: f32) -> (f32, f32) {
		let grav = Vec2::new(0.0, -9.81);
		let mut particle = Particle::new(Vec2::ZERO, Vec2::ZERO, 1.0);

		for _ in 0..i {
			particle.add_accel(grav);
			particle.step(dt);
		}

		return (particle.pos.y, particle.vel.y);
	}

	fn pct_error(measured: f32, actual: f32) -> f32 {
		(1.0 - (measured / actual)).abs()
	}

	#[test]
	fn grav_test_1() {
		let (pos, vel) = grav_test(50, 0.01);
		let correct_pos = -1.22625;
		let correct_vel = -4.905;
		let acceptable_error = 0.01;

		dbg!(pos, vel);
		dbg!(correct_pos, correct_vel);

		// test that velocity is correct
		assert!(pct_error(vel, correct_vel) <= acceptable_error);

		// test that position is correct
		assert!(pct_error(pos, correct_pos) <= acceptable_error);
	}

	#[test]
	fn grav_test_2() {
		let (pos, vel) = grav_test(50000, 0.01);
		let correct_pos = -1226250.0;
		let correct_vel = -4905.0;
		let acceptable_error = 0.01;

		dbg!(pos, vel);
		dbg!(correct_pos, correct_vel);

		// test that velocity is correct
		assert!(pct_error(vel, correct_vel) <= acceptable_error);

		// test that position is correct
		assert!(pct_error(pos, correct_pos) <= acceptable_error);
	}

}
