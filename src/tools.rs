use macroquad::math::Vec2;
use macroquad::window;

pub fn screen_center() -> Vec2 {
	Vec2::new(window::screen_width() / 2., window::screen_height() / 2.)
}