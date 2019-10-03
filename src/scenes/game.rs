use std::rc::Rc;
use std::cell::{RefCell};
use std::collections::HashSet;

use tetra::graphics::{self, DrawParams, Drawable, Rectangle};
use tetra::input::{self, Key};
use tetra::{Context};
use tetra::glm::Vec2;

use rand::prelude::*;

use crate::scenes::manager::{Scene, Transition};
use crate::assets::{Assets, SymbolName};
use crate::models::config::Config;
use crate::tile::{Tile};
use crate::button::{Button,ButtonType};
use crate::models::level::{Level,load_level};
use crate::constants::*;
use crate::utils::particle::{Particle};

#[allow(dead_code)]
pub struct GameScene {
	config: Rc<Config>,
	assets: Rc<RefCell<Assets>>,
	state: GameState,
	tiles: Vec<Tile>,
	levels: Vec<Level>,
	level: usize,
	click: u8,
	btn_future: Button,
	btn_back: Button,
	particles: Vec<Particle>,
	randomizer: ThreadRng,
}

impl GameScene {
	pub fn new(_ctx: &mut Context,config: Rc<Config>, assets: Rc<RefCell<Assets>>) -> tetra::Result<GameScene> {
		let level = 10;
		let levels = load_levels();
		let randomizer = rand::thread_rng();
		Ok(GameScene {
			config,
			state: GameState::Running,
			tiles: build_buttons(Rc::clone(&assets), &levels[level-1])?,
			btn_future: Button::new(Rc::clone(&assets), GET_POSITION_FUTURE_BUTTON(), GET_TOUCH_AREA_BUTTON(), ButtonType::Future)?,
			btn_back: Button::new(Rc::clone(&assets), GET_POSITION_BACK_BUTTON(), GET_TOUCH_AREA_BUTTON(), ButtonType::Back)?,
			assets,
			levels,
			level,
			click: 0,
			particles: vec![],
			randomizer,
		}.init())
	}
	
	fn init(mut self) -> Self{
		self.init_level();
		self
	}
	
	fn init_level(&mut self){
		self.click = self.levels[self.level-1].moves;
		for (i,b) in self.tiles.iter_mut().enumerate(){
			b.number(self.levels[self.level-1].values[i]);
			b.set_enabled(self.levels[self.level-1].states[i]);
			b.mark(self.levels[self.level-1].flagged[i]);
		}
		self.state = GameState::Running;
		self.btn_future.change_type_to(ButtonType::Future);
	}
	
	pub fn next_level(&mut self){
		if self.level < self.levels.len(){
			self.level +=1;
			self.init_level();
		}
	}
	
	fn change_tile_effect(&mut self, tile_index: HashSet<usize>, round: usize){
		let directions = vec![Vec2::new(1.0,0.0),Vec2::new(0.0,1.0),Vec2::new(-1.0,0.0),Vec2::new(0.0,-1.0),Vec2::new(1.0,0.0)];
		for n in tile_index{
			self.tiles[n].enable();
			let particle = Particle::new(self.tiles[n].position,directions[round])
				.set_aging(self.randomizer.gen_range(0.002,0.004))
				.set_texture_name(self.tiles[n].get_texture_name());
			self.particles.push(particle);
		}
	}
	
	fn after_work(&mut self, numbers: Vec<u8>) -> HashSet<usize>{
		let mut tile_index = HashSet::new();
		for i in 0..3{
			if numbers[i*3] == numbers[i*3+1] && numbers[i*3] == numbers[i*3+2] && numbers[i*3] != 0{
				tile_index.insert(i*3);
				tile_index.insert(i*3+1);
				tile_index.insert(i*3+2);
			}
			if numbers[i] == numbers[i+3] && numbers[i] == numbers[i+6] && numbers[i] != 0{
				tile_index.insert(i);
				tile_index.insert(i+3);
				tile_index.insert(i+6);
			}
		}
		tile_index
	}
	
	fn go_future(&mut self){
		self.state = GameState::Future;
		for b in self.tiles.iter_mut(){
			b.go_future();
		}
		
		let mut sum: u8 = self.tiles.iter().map(|t|t.number).sum();
		if sum != 0{
			for r in 0..4{
				let numbers = self.tiles.iter().map(|t|t.number).collect::<Vec<_>>();
				let tile_index = self.after_work(numbers);
				self.change_tile_effect(tile_index.clone(), r);
				for n in tile_index{
					let num = self.tiles[n].number-1;
					self.tiles[n].number(num);
				}
			}
			sum = self.tiles.iter().map(|t|t.number).sum();
		}

		if sum == 0{
			self.state = GameState::Win;
			self.btn_future.change_type_to(ButtonType::Next);
		}else{
			self.state = GameState::Lost;
		}
	}
}

impl Scene for GameScene {
	fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
		// particle update
		self.particles.retain(|p| !p.is_dead());
		for p in self.particles.iter_mut(){
			p.update();
		}
		
		// update animations
		self.assets.borrow_mut().update();
		
		//update tiles
		for b in self.tiles.iter_mut(){
			if b.is_marked(){
				b.update(ctx);
				if !b.is_marked(){
					self.click +=1;
				}
			}else if self.click > 0{
				b.update(ctx);
				if b.is_marked(){
					self.click -=1;
				}
			}
		}
		
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
		
		for b in self.tiles.iter(){
			b.draw(ctx, DrawParams::default());
		}
		
		for i in 0..self.click{
			graphics::draw(ctx,self.assets.borrow().get_symbol(&SymbolName::SymbolClick),
				Vec2::new(f32::from(i*10 + X_POSITION_MOVES_SYMBOLE), f32::from(Y_POSITION_MOVES_SYMBOLE)));
		}
		
		for p in self.particles.iter().rev() {
			graphics::draw(ctx, self.assets.borrow().get_texture(&p.get_texture_name()), DrawParams::new()
				.position(p.get_position())
				.color(p.get_color())
			);
		}


		if self.state == GameState::Lost{
			graphics::draw(ctx, &self.btn_back, DrawParams::default());
		}else if self.click == 0 || self.state == GameState::Win {
			graphics::draw(ctx, &self.btn_future, DrawParams::default());
		}
		
		Ok(Transition::None)
	}
}


fn build_buttons(assets: Rc<RefCell<Assets>>, level: &Level) -> tetra::Result<Vec<Tile>>{
	let touch_area = Rectangle::new(0.0,0.0,16.0,16.0);
	let positions = get_positions();
	let mut buttons: Vec<Tile> = vec![];
	for i in 0..9{
		buttons.push(Tile::new(Rc::clone(&assets), positions[i], touch_area, level.values[i])?)
	}
	Ok(buttons)
}

fn get_positions() -> Vec<Vec2>{
	let jump = TILE_SIZE + TILE_GAP;
	let mut positions = vec![];
	for i in 0..3{
		positions.push(Vec2::new((LEFT_BORDER_SPACING+i*jump) as f32,TOP_BORDER_SPACING as f32));
		positions.push(Vec2::new((LEFT_BORDER_SPACING+i*jump) as f32,(TOP_BORDER_SPACING + jump) as f32));
		positions.push(Vec2::new((LEFT_BORDER_SPACING+i*jump) as f32,(TOP_BORDER_SPACING + 2*jump) as f32));
	}
	positions
}

fn load_levels() ->Vec<Level>{
	vec![
		load_level(include_str!("../../assets/levels/level_1.ron")),
		load_level(include_str!("../../assets/levels/level_2.ron")),
		load_level(include_str!("../../assets/levels/level_3.ron")),
		load_level(include_str!("../../assets/levels/level_4.ron")),
		load_level(include_str!("../../assets/levels/level_5.ron")),
		load_level(include_str!("../../assets/levels/level_6.ron")),
		load_level(include_str!("../../assets/levels/level_7.ron")),
		load_level(include_str!("../../assets/levels/level_8.ron")),
		load_level(include_str!("../../assets/levels/level_9.ron")),
		load_level(include_str!("../../assets/levels/level_10.ron")),
		load_level(include_str!("../../assets/levels/level_11.ron")),
		load_level(include_str!("../../assets/levels/level_12.ron")),
		load_level(include_str!("../../assets/levels/level_13.ron")),
		]
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
	Running,
	Win,
	Lost,
	Future,
}