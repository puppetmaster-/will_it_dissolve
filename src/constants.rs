#![allow(non_snake_case)]

use tetra::glm::Vec2;
use tetra::graphics::{Color, Rectangle};

pub const TILE_SIZE: i32 = 16;
pub const TILE_GAP: i32 = 1;
pub const TOP_BORDER_SPACING: i32 = 12;
pub const LEFT_BORDER_SPACING: i32 = 24;

pub const X_POSITION_MOVES_SYMBOLE: u8 = 36;
pub const Y_POSITION_MOVES_SYMBOLE: u8 = 68;


pub fn GET_TOUCH_AREA_BUTTON() ->Rectangle{
	Rectangle::new(0.0,0.0,32.0,16.0)
}

pub fn GET_POSITION_FUTURE_BUTTON() -> Vec2{
	Vec2::new(60.0,80.0)
}

pub fn GET_POSITION_BACK_BUTTON() -> Vec2{
	Vec2::new(10.0,80.0)
}

pub fn GET_JAMLOGO_POSITION() -> Vec2{
	Vec2::new(10.0,26.0)
}

pub fn GET_LOGO_POSITION() -> Vec2{
	Vec2::new(16.0,30.0)
}

pub fn GET_FUTURE_COLOR() -> Color{
	Color::rgb8(60,67,101)
}
