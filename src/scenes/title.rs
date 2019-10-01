use std::rc::Rc;
use std::cell::{RefCell};

use tetra::{Context, input};
use tetra::input::{Key};
use tetra::graphics::{self, Color, DrawParams};

use crate::assets::{Assets, TextureName};
use crate::constants::*;
use crate::scenes::manager::{Scene, Transition};
use crate::scenes::menu::MenuScene;
use crate::models::config::Config;
use crate::utils::{tween::TweenColor};

pub struct TitleScene {
	config: Rc<Config>,
	assets: Rc<RefCell<Assets>>,
	tween: TweenColor,
}

impl TitleScene {
	pub fn new(_ctx: &mut Context,config: Rc<Config>, assets: Rc<RefCell<Assets>>) -> tetra::Result<TitleScene> {
		Ok(TitleScene {
			config,
			assets,
			tween: TweenColor::new(Color::rgba(1.0,1.0,1.0,1.0),8),
		})
	}
}

impl Scene for TitleScene {
	fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
		self.tween.update();

		if self.tween.finished || input::is_key_released(ctx, Key::Escape) || input::is_key_released(ctx, Key::Return) || input::is_key_released(ctx, Key::Space) {
			Ok(Transition::Push(Box::new(MenuScene::new(ctx, Rc::clone(&self.config), Rc::clone(&self.assets))?)))
		}else{
			Ok(Transition::None)
		}
	}

	fn draw(&mut self, ctx: &mut Context, _dt: f64) -> tetra::Result<Transition> {
		graphics::clear(ctx, self.config.clear_color);
		graphics::draw(ctx, self.assets.borrow().get_texture(&TextureName::JamLogo),DrawParams::new()
			.position(GET_JAMLOGO_POSITION())
			.color(self.tween.get_color())
			);

		Ok(Transition::None)
	}
}
