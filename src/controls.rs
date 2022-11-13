use macroquad::camera::Camera2D;
use macroquad::input;
use macroquad::prelude::set_camera;
use macroquad::prelude::KeyCode;
// use macroquad::Window;

const ZOOM_FACTOR: f32 = 1.5;

/// controls the zoom level of the camera
pub fn zoom(cam: &mut Camera2D) {
    
    // zoom in (equal key is same button as +)
    if input::is_key_pressed(KeyCode::Equal) {
        cam.zoom *= self::ZOOM_FACTOR;
    } else if input::is_key_pressed(KeyCode::Minus) {
        cam.zoom /= self::ZOOM_FACTOR;
    }

    set_camera(cam);
}

// const SENSITIVITY: f32 = 0.01;
