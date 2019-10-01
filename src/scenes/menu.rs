use std::rc::Rc;
use std::cell::{RefCell};

use tetra::{Context, input, audio};
use tetra::input::{Key, MouseButton};
use tetra::graphics::{self,Vec2};
use tetra::audio::{Sound, SoundInstance};

use crate::assets::{Assets, TextureName};
use crate::scenes::manager::{Scene, Transition};
use crate::scenes::game::GameScene;
use crate::models::config::Config;
use crate::constants::*;
use crate::utils::tween::TweenPosition;

pub struct MenuScene {
	config: Rc<Config>,
	assets: Rc<RefCell<Assets>>,
	background_music_instance: SoundInstance,
	tween_logo: TweenPosition,
}

impl MenuScene {
	pub fn new(ctx: &mut Context,config: Rc<Config>, assets: Rc<RefCell<Assets>>) -> tetra::Result<MenuScene> {
		audio::set_master_volume(ctx, config.master_volume);
		let background_music = Sound::from_file_data(include_bytes!("../../assets/music/track.mp3"));
		let background_music_instance = background_music.spawn(ctx)?;
		background_music_instance.set_repeating(true);
		background_music_instance.play();
		background_music_instance.set_volume(0.2);
		Ok(MenuScene {
			config,
			assets,
			background_music_instance,
			tween_logo: TweenPosition::new(GET_LOGO_POSITION(), 0.1, 6, Vec2::new(0.0,1.0)),
		})
	}
}

impl Scene for MenuScene {
	fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
		self.tween_logo.update();
		
		if input::is_mouse_button_released(ctx, MouseButton::Left) || input::is_key_released(ctx, Key::Return) || input::is_key_released(ctx, Key::Space) {
			Ok(Transition::Push(Box::new(GameScene::new(ctx, Rc::clone(&self.config), Rc::clone(&self.assets))?)))
		}else if input::is_key_released(ctx, Key::Escape) || input::is_key_released(ctx, Key::Backspace){
			Ok(Transition::Quit)
		}else{
			Ok(Transition::None)
		}
	}

	fn draw(&mut self, ctx: &mut Context, _dt: f64) -> tetra::Result<Transition> {
		graphics::clear(ctx, self.config.clear_color);
		graphics::draw(ctx, self.assets.borrow().get_texture(&TextureName::Logo), self.tween_logo.get_position());

		Ok(Transition::None)
	}
}
