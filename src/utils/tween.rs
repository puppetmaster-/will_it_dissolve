
use tetra::graphics::{Color,Vec2};
use crate::utils::timer::Timer;

pub struct TweenPosition{
	speed: f32,
	position: Vec2,
	start_count: i32,
	move_tick: f32,
	move_count: i32,
	move_direction: Vec2,
}

impl TweenPosition {
	pub fn new(position: Vec2, speed: f32, move_pixel: i32, moving_direction: Vec2) -> Self{
		 Self{
			 position,
			 speed,
			 start_count: move_pixel,
			 move_tick: 0.0,
			 move_count: move_pixel/2,
			 move_direction: moving_direction,
		 }
	}

	pub fn update(&mut self){
		if self.move_count != 0 {
			self.move_tick += self.speed;
			if self.move_tick >= 1.0 {
				self.move_tick = 0.0;
				self.position += self.move_direction;
				self.move_count -= 1;
				if self.move_count == 0 {
					self.move_count = self.start_count;
					self.move_direction = Vec2::new(self.move_direction.x * -1.0,self.move_direction.y * -1.0);
				}
			}
		}
	}

	pub fn get_position(&self) -> Vec2{
		self.position
	}
}

pub struct TweenColor{
	color: Color,
	timer: Timer,
	pub finished: bool,
	reverse: bool,
}

impl TweenColor{
	pub fn new(color: Color, duration: u64) -> TweenColor{
		Self{
			color,
			timer: Timer::new(duration/2),
			finished: false,
			reverse: false,
		}
	}
	pub fn update(&mut self){
		if !self.finished{
			self.timer.update();
			if self.timer.finished{
				if self.timer.counter == 0{
					self.reverse = true;
					self.timer.restart();
				}else{
					self.finished = true;
				}
			}
		}
	}
	
	pub fn get_color(&self) -> Color{
		if self.reverse{
			Color::rgba(self.color.r,self.color.g,self.color.b,1.0 - self.timer.get_value())
		}else{
			Color::rgba(self.color.r,self.color.g,self.color.b,self.timer.get_value())
		}
	}
}