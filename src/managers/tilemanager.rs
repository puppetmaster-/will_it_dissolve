use crate::tile::Tile;
use std::cell::RefCell;
use std::rc::Rc;
use crate::assets::Assets;
use crate::models::level::Level;
use tetra::graphics::{Rectangle, Vec2, DrawParams, Drawable};
use crate::constants::{TILE_SIZE, LEFT_BORDER_SPACING, TOP_BORDER_SPACING, TILE_GAP};
use tetra::{Context, graphics};
use crate::utils::particle::Particle;
use rand::prelude::ThreadRng;
use std::collections::HashSet;
use rand::Rng;
use crate::utils::timer::Timer;

pub struct Tilemanager{
	tiles: Vec<Tile>,
	particles: Vec<Particle>,
	randomizer: ThreadRng,
	assets: Rc<RefCell<Assets>>,
	ready_timer: Timer,
}

impl Tilemanager{
	pub fn new(assets: Rc<RefCell<Assets>>) -> tetra::Result<Tilemanager>{
		let randomizer = rand::thread_rng();
		Ok(Tilemanager{
			tiles: vec![],
			assets,
			particles: vec![],
			randomizer,
			ready_timer: Timer::new(1),
		}.init()?)
	}

	pub fn init(mut self)-> tetra::Result<Self>{
		let touch_area = Rectangle::new(0.0,0.0,16.0,16.0);
		let positions = get_positions();
		for i in 0..9{
			self.tiles.push(Tile::new(Rc::clone(&self.assets), positions[i], touch_area, 0)?)
		}
		Ok(self)
	}

	pub fn init_level(&mut self,level: &Level){
		for (i,b) in self.tiles.iter_mut().enumerate(){
			b.number(level.values[i]);
			b.set_enabled(level.states[i]);
			b.mark(level.flagged[i]);
		}
	}

	pub fn go_future(&mut self) -> u8{
		self.ready_timer.restart();
		for tile in self.tiles.iter_mut(){
			if tile.is_marked(){
				tile.enable();
				let particle = Particle::new(tile.position,Vec2::new(0.0,-1.0))
					.set_aging(self.randomizer.gen_range(0.002,0.004))
					.set_texture_name(tile.get_texture_name());
				self.particles.push(particle);
				tile.go_future();
			}
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
		sum
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

	pub fn is_ready(&self) -> bool{
		self.ready_timer.finished
	}

	pub fn draw(&self, ctx: &mut Context){
		for b in self.tiles.iter(){
			b.draw(ctx, DrawParams::default());
		}

		for p in self.particles.iter().rev() {
			graphics::draw(ctx, self.assets.borrow().get_texture(&p.get_texture_name()), DrawParams::new()
				.position(p.get_position())
				.color(p.get_color())
			);
		}
	}

	pub fn update(&mut self, ctx: &mut Context, actions: &mut u8){
		// timer update
		self.ready_timer.update();

		for b in self.tiles.iter_mut(){
			if b.is_marked(){
				b.update(ctx);
				if !b.is_marked(){
					*actions +=1;
				}
			}else if actions > &mut 0{
				b.update(ctx);
				if b.is_marked(){
					*actions -=1;
				}
			}
		}

		// particle update
		self.particles.retain(|p| !p.is_dead());
		for p in self.particles.iter_mut(){
			p.update();
		}
	}
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