use std::rc::Rc;
use std::cell::{RefCell};
use serde::{Serialize, Deserialize};

use tetra::graphics::{Drawable, DrawParams, Vec2, Rectangle};
use tetra::input::{self, MouseButton};
use tetra::{Context, glm};
use crate::assets::{Assets, TextureName, AnimationName};

const MAXNUMBER: u8 = 4;

pub struct Tile{
	assets: Rc<RefCell<Assets>>,
	pub position: Vec2,
	state: TileState,
	touch_area: Rectangle,
	pressed: bool,
	pub number: u8,
	disabled: bool,
}

#[allow(dead_code)]
impl Tile {
	pub fn new(assets: Rc<RefCell<Assets>>, position: Vec2, touch_area: Rectangle, number: u8) -> tetra::Result<Tile>{
		Ok(Tile{
			assets,
			position,
			state: TileState::Normal,
			touch_area,
			pressed: false,
			number,
			disabled: false,
		})
	}

	pub fn number(&mut self, number: u8){
		self.number = number;
	}

	pub fn subtract(&mut self){
		self.state = TileState::Normal;
		if self.number > 1{
			self.number -=1;
		}else{
			self.number = MAXNUMBER;
		}
	}
	
	pub fn add_up(&mut self){
		self.state = TileState::Normal;
		if self.number < MAXNUMBER{
			self.number +=1;
		}else{
			self.number = 1;
		}
	}

	pub fn is_marked(&self) -> bool{
		(self.state == TileState::Plus || self.state == TileState::Minus)
	}
	
	pub fn go_future(&mut self){
		match self.state{
			TileState::Plus => self.add_up(),
			TileState::Minus => self.subtract(),
			_ => ()
		}
	}
	
	pub fn set_enabled(&mut self, enabled: bool){
		if enabled { self.enable() } else { self.disable() }
	}

	pub fn disable(&mut self){
		self.disabled = true;
	}

	pub fn enable(&mut self){
		self.disabled = false;
	}

	pub fn update(&mut self, ctx: &mut Context){
		if !self.disabled && self.number > 0 {
			let mouse_position = glm::round(&input::get_mouse_position(ctx));
			if is_inside_hover_area(self.position, self.touch_area, mouse_position) {
				if input::is_mouse_button_down(ctx, MouseButton::Left) && !self.pressed{
					self.pressed = true;
					if self.state != TileState::Plus{
						self.state = TileState::Plus;
						
					}else{
						self.state = TileState::Normal;
					}
				}
				if input::is_mouse_button_down(ctx, MouseButton::Right) && !self.pressed{
					self.pressed = true;
					if self.state != TileState::Minus{
						self.state = TileState::Minus;
					}else{
						self.state = TileState::Normal;
					}
				}
				if input::is_mouse_button_released(ctx, MouseButton::Left) ||
					input::is_mouse_button_released(ctx, MouseButton::Right){
					self.pressed = false;
				}
			}
		}
	}

	pub fn get_animation_name(&self) -> Option<AnimationName>{
		match self.state{
			TileState::Plus => Some(AnimationName::Plus),
			TileState::Minus => Some(AnimationName::Minus),
			_ => None,
		}
	}

	pub fn get_texture_name(&self) -> TextureName{
		match self.number{
			0 => {
					if self.disabled{
						TextureName::Pic0Off
					}else{
						TextureName::Pic0On
					}
				},
			1 => {
					if self.disabled{
						TextureName::Pic1Off
					}else{
						TextureName::Pic1On
					}
				},
			2 => {
					if self.disabled{
						TextureName::Pic2Off
					}else{
						TextureName::Pic2On
					}
				},
			3 => {
				if self.disabled{
					TextureName::Pic3Off
				}else{
					TextureName::Pic3On
				}
			},
			4 => {
				if self.disabled{
					TextureName::Pic4Off
				}else{
					TextureName::Pic4On
				}
			},
			_ => {
				if self.disabled{
					TextureName::Pic1Off
				}else{
					TextureName::Pic1On
				}
			}
		}
	}

	pub fn is_pressed(&self) -> bool{
		self.pressed
	}

	pub fn get_pressed(&mut self) -> bool{
		if self.pressed{
			self.pressed = false;
			return true;
		}
		false
	}
}

impl Drawable for Tile {
	fn draw<P>(&self, ctx: &mut Context, _params: P)
		where
			P: Into<DrawParams>,
	{
		//background
		let texture_name = self.get_texture_name();
		self.assets.borrow().get_texture(&texture_name).draw(ctx,DrawParams::new()
			.position(Vec2::new(self.position.x,self.position.y))
		);
		//foreground
		let animation_name = self.get_animation_name();
		if let Some(a) = animation_name{
			self.assets.borrow().get_animation(&a).draw(
			ctx,DrawParams::new().position(Vec2::new(self.position.x,self.position.y))) 
		}
	}
}

fn is_inside_hover_area(draw_position: Vec2, area: Rectangle, position: Vec2) -> bool{
	let pos_x = draw_position.x;
	let pos_y = draw_position.y;
	!(position.x < area.x + pos_x ||
		position.y < area.y + pos_y ||
		position.x > area.x + pos_x + area.width ||
		position.y > area.y + pos_y + area.height
	)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash,Serialize, Deserialize)]
pub enum TileState {
	Normal,
	Minus,
	Plus,
}
