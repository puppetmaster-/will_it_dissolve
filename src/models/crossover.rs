use tetra::graphics::{Shader, DrawParams};
use crate::utils::timer::Timer;
use std::cell::RefCell;
use crate::assets::{TextureName, Assets, ShaderName};
use tetra::{graphics, Context};
use std::rc::Rc;

pub struct Crossover {
	shader: Shader,
	assets: Rc<RefCell<Assets>>,
	timer: Timer,
	active: bool,
	texture_name: TextureName,
}

impl Crossover {
	pub fn new(assets: Rc<RefCell<Assets>>) -> tetra::Result<Crossover>{
		Ok(Crossover {
			shader: assets.borrow_mut().get_shader(ShaderName::LevelTransition),
			assets: Rc::clone(&assets),
			texture_name: TextureName::Black,
			timer: Timer::new(1),
			active: false,
		})
	}
	pub fn play(&mut self){
		self.active = true;
		self.timer.restart();
	}

	pub fn update(&mut self){
		if self.active{
			self.timer.update();
			self.active = !self.timer.finished;
		}
	}
	
	pub fn draw(&mut self, ctx: &mut Context){
		if self.active {
			graphics::set_shader(ctx, &self.shader);
			self.shader.set_uniform(ctx, "u_progress", self.timer.get_value());
			self.shader.set_uniform(ctx, "u_size", 10.0);
			graphics::draw(ctx, self.assets.borrow().get_texture(&self.texture_name), DrawParams::default());
			graphics::reset_shader(ctx);
		}
	}
}