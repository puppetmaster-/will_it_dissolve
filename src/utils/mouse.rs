use std::rc::Rc;
use std::cell::{RefCell};

use tetra::graphics::{self, Drawable, DrawParams};
use tetra::{Context, input, glm};

use crate::assets::{Assets, AnimationName};

pub struct Mouse{
	assets: Rc<RefCell<Assets>>,
	visible: bool,
}

impl Mouse{
	pub fn new(assets: Rc<RefCell<Assets>>) -> tetra::Result<Mouse>{
		Ok(Mouse{
			assets,
			visible: true,
		})
	}

	#[allow(dead_code)]
	pub fn set_visible(&mut self, visible: bool){
		self.visible = visible;
	}

}

impl Drawable for Mouse {
	fn draw<P>(&self, ctx: &mut Context, params: P)
		where
			P: Into<DrawParams>,
	{
		if self.visible{
			let mut params = params.into();
			params.position = glm::round(&input::get_mouse_position(ctx));
			graphics::draw(ctx,self.assets.borrow().get_animation(&AnimationName::Action),params);
		}
	}
}