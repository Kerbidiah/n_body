use macroquad::prelude::set_camera;
use macroquad::camera::Camera2D;
use macroquad::input;
use macroquad::prelude::KeyCode;
// use macroquad::Window;

const ZOOM_FACTOR: f32 = 1.5;

/// controls the zoom level of the camera
pub fn zoom(cam: &mut Camera2D) {
	if input::is_key_pressed(KeyCode::Equal) { // zoom in (equal key is same button as +)
		cam.zoom *= self::ZOOM_FACTOR;
	} else if input::is_key_pressed(KeyCode::Minus) {
		cam.zoom /= self::ZOOM_FACTOR;
	}

	set_camera(cam);
}


// const SENSITIVITY: f32 = 0.01;