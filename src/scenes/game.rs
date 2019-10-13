use std::rc::Rc;
use std::cell::{RefCell};

use tetra::graphics::{self, DrawParams, Drawable};
use tetra::input::{self, Key};
use tetra::{Context};
use tetra::glm::Vec2;

use crate::scenes::manager::{Scene, Transition};
use crate::assets::{Assets, AnimationName, TextureName};
use crate::models::config::Config;
use crate::button::{Button,ButtonType};

use crate::constants::*;
use crate::utils::mouse::Mouse;
use crate::managers::tilemanager::Tilemanager;
use crate::managers::levelmanager::Levelmanager;

#[allow(dead_code)]
pub struct GameScene {
	config: Rc<Config>,
	assets: Rc<RefCell<Assets>>,
	state: GameState,
	tilemanager: Tilemanager,
	levelmanager: Levelmanager,
	actions: u8,
	btn_future: Button,
	btn_back: Button,
	mouse: Mouse,
}

impl GameScene {
	pub fn new(_ctx: &mut Context,config: Rc<Config>, assets: Rc<RefCell<Assets>>) -> tetra::Result<GameScene> {
		Ok(GameScene {
			config,
			state: GameState::Running,
			tilemanager: Tilemanager::new(Rc::clone(&assets))?,
			levelmanager: Levelmanager::new(1)?,
			btn_future: Button::new(Rc::clone(&assets), GET_POSITION_FUTURE_BUTTON(), GET_TOUCH_AREA_BUTTON(), ButtonType::Future)?,
			btn_back: Button::new(Rc::clone(&assets), GET_POSITION_BACK_BUTTON(), GET_TOUCH_AREA_BUTTON(), ButtonType::Back)?,
			mouse: Mouse::new(Rc::clone(&assets))?,
			assets,
			actions: 0,

		}.init())
	}
	
	fn init(mut self) -> Self{
		self.init_level();
		self
	}
	
	fn init_level(&mut self){
		self.actions = self.levelmanager.get_current_level().moves;
		self.tilemanager.init_level(self.levelmanager.get_current_level());
		self.state = GameState::Running;
		self.btn_future.change_type_to(ButtonType::Future);
	}
	
	pub fn next_level(&mut self){
		if self.levelmanager.advance_next_level(){
			self.init_level();
		}else{
			self.state = GameState::End;
		}
	}
	
	fn go_future(&mut self){
		self.state = GameState::Future;
		let number_of_visible_tiles = self.tilemanager.go_future();
		if number_of_visible_tiles == 0{
			self.state = GameState::Win;
			self.actions = 0;
			self.btn_future.change_type_to(ButtonType::Next);
		}else{
			self.state = GameState::Lost;
		}
	}
}

impl Scene for GameScene {
	fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
		// update animations
		self.assets.borrow_mut().update();

		// update tiles
		self.tilemanager.update(ctx, &mut self.actions);
		
		// back button
		if self.state == GameState::Lost{
			self.btn_back.update(ctx);
			if self.btn_back.is_pressed(){
				self.btn_back.get_pressed();
				self.init_level();
			}
		}else{
			// future button
			self.btn_future.update(ctx);
			if self.btn_future.is_pressed(){
				self.btn_future.get_pressed();
				if self.state == GameState::Win{
					self.next_level()
				}else{
					self.go_future();
				}
			}
		}
		
		// check keys
		if input::is_key_released(ctx, Key::Backspace){
			Ok(Transition::Pop)
		}else{
			Ok(Transition::None)
		}
	}

	fn draw(&mut self, ctx: &mut Context, _dt: f64) -> tetra::Result<Transition> {
		if self.state != GameState::Lost && self.state != GameState::Win{
			graphics::clear(ctx, self.config.clear_color);
		}else{
			graphics::clear(ctx, GET_FUTURE_COLOR());
		}
		//draw tiles
		self.tilemanager.draw(ctx);

		//draw actions symbol
		for i in 0..self.actions{
			graphics::draw(ctx,self.assets.borrow().get_animation(&AnimationName::Action),
				Vec2::new(f32::from(i*10 + X_POSITION_MOVES_SYMBOLE), f32::from(Y_POSITION_MOVES_SYMBOLE)));
		}

		// draw future, next or back buttons
		if self.state == GameState::Lost && self.tilemanager.is_ready(){
			graphics::draw(ctx, &self.btn_back, DrawParams::default());
		}else if self.state == GameState::End && self.tilemanager.is_ready() {
			graphics::draw(ctx, self.assets.borrow().get_texture(&TextureName::Thx), GET_THX_POSITION());
		}else if self.actions <= self.levelmanager.get_current_level().remaining_moves && self.tilemanager.is_ready() || self.state == GameState::Win && self.tilemanager.is_ready() {
			graphics::draw(ctx, &self.btn_future, DrawParams::default());
		}
		
		self.mouse.draw(ctx, DrawParams::default());
		Ok(Transition::None)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
	Running,
	Win,
	Lost,
	Future,
	End,
}