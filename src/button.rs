use std::rc::Rc;
use std::cell::{RefCell};

use tetra::graphics::{Drawable, DrawParams, Vec2, Rectangle};
use tetra::input::{self, MouseButton};
use tetra::{Context, glm};
use crate::assets::{Assets, TextureName, AnimationName};

pub struct Button{
	assets: Rc<RefCell<Assets>>,
	position: Vec2,
	state: State,
	touch_area: Rectangle,
	pressed: bool,
	texture_name: TextureName,
	animation_name: AnimationName,
}

#[allow(dead_code)]
impl Button {
	pub fn new(assets: Rc<RefCell<Assets>>, position: Vec2, touch_area: Rectangle, button_type: ButtonType) -> tetra::Result<Button>{
		let animation_name = match button_type{
			ButtonType::Future => AnimationName::Future,
			ButtonType::Back => AnimationName::Back,
			ButtonType::Next => AnimationName::Next,
		};
		let texture_name = match button_type{
			ButtonType::Future => TextureName::Future,
			ButtonType::Back => TextureName::Back,
			ButtonType::Next => TextureName::Next,
		};
		Ok(Button{
			assets,
			position,
			state: State::Normal,
			touch_area,
			pressed: false,
			texture_name,
			animation_name,
		})
	}

	pub fn change_type_to(&mut self, button_type: ButtonType){
		self.animation_name = match button_type{
			ButtonType::Future => AnimationName::Future,
			ButtonType::Back => AnimationName::Back,
			ButtonType::Next => AnimationName::Next,
		};
		self.texture_name = match button_type{
			ButtonType::Future => TextureName::Future,
			ButtonType::Back => TextureName::Back,
			ButtonType::Next => TextureName::Next,
		};
	}

	pub fn disable(&mut self){
		self.state = State::Disabled;
	}

	pub fn enable(&mut self){
		self.state = State::Normal;
	}

	pub fn update(&mut self, ctx: &mut Context){
		if self.state != State::Disabled{
			let mouse_position = glm::round(&input::get_mouse_position(ctx));
			if is_inside_hover_area(self.position, self.touch_area, mouse_position) {
				if input::is_mouse_button_pressed(ctx, MouseButton::Left) {
					self.pressed = true;
					self.state = State::Hover;
				}else{
					self.state = State::Hover;
				}
			}else if self.pressed{
				self.state = State::Hover;
			}else{
				self.state = State::Normal;
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

impl Drawable for Button {
	fn draw<P>(&self, ctx: &mut Context, params: P)
		where
			P: Into<DrawParams>,
	{
		let params = params.into();
		let new_params = DrawParams::new()
			.position(Vec2::new(self.position.x + params.position.x,self.position.y + params.position.y))
			.scale(params.scale);
		if self.state == State::Hover || self.pressed{
			self.assets.borrow().get_animation(&self.animation_name).draw(ctx,new_params);
		}else{
			self.assets.borrow().get_texture(&self.texture_name).draw(ctx,new_params);
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum State {
	Normal,
	Hover,
	Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ButtonType {
	Future,
	Back,
	Next,
}
