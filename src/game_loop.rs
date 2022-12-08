use std::cell::RefCell;

use rayon::prelude::*;

use macroquad::camera::Camera2D;
use macroquad::color::colors;
use macroquad::prelude::*;

use egui_macroquad;
use egui_macroquad::egui;
use egui::{Pos2, widgets};

use crate::controls;
use crate::particle;
use crate::physics;
use crate::config;
use crate::mov_avg::MovingAverage;

use particle::Particle;


/// does all the physics and UI stuff after the settings and config stuff is done
pub async fn game_loop(bodies: &mut Vec<Particle>, cam: &mut Camera2D, s: &mut config::Settings) {
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
	let mut fps_history = MovingAverage::new();
	let mut frame_time_history = MovingAverage::new();
	let mut killed_by_dist: usize = 0;
	let mut fps_goal: i32 = 60;
	loop {
		clear_background(BLACK);

		// controls
		controls::zoom(cam);
		controls::fix_aspect_ratio(cam);

		physics::physics_loop(&mut bodies, s.sims_per_frame, s.dt_multiplier);
		
		// lock the "star" in place
		// bodies[0].borrow_mut().pos = Vec2::ZERO;
		// bodies[0].borrow_mut().vel = Vec2::ZERO;
		
		// kill particles that have gone too far if kill distance is enabled
		let prev = bodies.len();
		if let Some(distance) = s.kill_dist {
			let star = bodies[0].borrow().clone();
			bodies.retain(|b| b.borrow().dist_sqrd(&star) <= distance.powi(2));
		}
		killed_by_dist += prev - bodies.len();
		
		bodies.iter().for_each(|b| {
			b.borrow().draw(colors::WHITE);
		});

		fps_history.insert_i32(get_fps());
		frame_time_history.insert(get_frame_time());

		// ui for onscreen info
		egui_macroquad::ui(|egui_ctx| {
			egui::Window::new("Info")
				.default_pos(Pos2::ZERO)
				.show(egui_ctx, |ui| {
					widgets::global_dark_light_mode_buttons(ui);
					
					// fps goal buttons
					ui.horizontal(|ui| {
						ui.label("desired fps");
						ui.add_space(20.0);
						ui.selectable_value(&mut fps_goal, 15, "15");
						ui.selectable_value(&mut fps_goal, 30, "30");
						ui.selectable_value(&mut fps_goal, 60, "60");
						ui.selectable_value(&mut fps_goal, 120, "120");
						ui.selectable_value(&mut fps_goal, 240, "240");
					});

					// stats on simulation
					egui::Grid::new("info table")
						.num_columns(2)
						.show(ui, |ui| {
							ui.label("fps");
							ui.label(fps_history.avg().to_string());
							ui.end_row();

							ui.label("frame time (ms)");
							ui.label((frame_time_history.avg() * 1000.0).to_string());
							ui.end_row();
							
							ui.label("sims per frame");
							ui.label(s.sims_per_frame.to_string());
							ui.end_row();

							ui.label("# bodies");
							ui.label(bodies.len().to_string());
							ui.end_row();

							ui.label("# killed by distance");
							ui.label(killed_by_dist.to_string());
							ui.end_row();
						});
				});
		});

		egui_macroquad::draw(); // draw UI
		
		adaptive_simrate(s, fps_goal);

		// advance to the next frame after 1/60th of a second has elapsed since previous frame
		// note: if you're screen has a higher refresh rate (like my laptop, 240Hz) it will instead
		// be 1/refresh_rate seconds, so 1/240th of a second for my laptop
		next_frame().await
	}
}

/// adjust sims per frame to maintain a `goal` fps
/// this uses a closed loop control
pub fn adaptive_simrate(s: &mut config::Settings, goal: i32) {
	// variables to make the code more concise
	let fps = get_fps() as f32;
	let g = goal as f32;
	let spf = s.sims_per_frame as f32;
	
	// calculate thresholds for incrasing/decresasing sims per frame
	let up_pct = (spf + 1.0) / spf;
	let dn_pct = (spf - 1.0) / spf;
	
	// raise or lower sim rate as needed
	if (s.sims_per_frame < u16::MAX) && (g * up_pct < fps) {
		s.sims_per_frame += 1;
	} else if (s.sims_per_frame > 1) && (g * dn_pct) > fps {
		s.sims_per_frame -= 1;
	}
}
