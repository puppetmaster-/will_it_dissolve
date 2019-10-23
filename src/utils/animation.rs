#![allow(dead_code)]
use tetra::graphics::{Texture, Rectangle, Drawable, DrawParams};
use tetra::Context;


#[derive(Debug, Clone)]
pub struct Animation {
	texture: Texture,
	frames: Vec<Rectangle>,
	frame_length: i32,
	current_frame: usize,
	timer: i32,
	active: bool,
	looping: bool,
}

impl Animation {
	pub fn new(texture: Texture, frames: Vec<Rectangle>, frame_length: i32) -> Animation {
		Animation {
			texture,
			frames,
			frame_length,

			current_frame: 0,
			timer: 0,
			active: true,
			looping: true,
		}
	}

	pub fn stop(mut self) ->Animation{
		self.active = false;
		self
	}

	pub fn tick(&mut self) {
		if self.active{
			self.timer += 1;
			if self.timer >= self.frame_length {
				let next_frame = self.current_frame+1;
				if !self.looping && next_frame == self.frames.len(){
					self.active = false;
				}else{
					self.current_frame = next_frame % self.frames.len();
				}
				self.timer = 0;
			}
		}

	}

	pub fn play_once(&mut self){
		self.looping = false;
		self.restart();
	}

	pub fn play(&mut self){
		self.looping = true;
		self.restart();
	}

	pub fn restart(&mut self) {
		self.current_frame = 0;
		self.timer = 0;
		self.active = true;
	}

	pub fn set_looping(&mut self, looping: bool){
		self.looping = looping;
	}

	pub fn is_active(&self)-> bool{
		self.active
	}

	pub fn texture(&self) -> &Texture {
		&self.texture
	}

	pub fn set_texture(&mut self, texture: Texture) {
		self.texture = texture;
	}

	pub fn frames(&self) -> &[Rectangle] {
		&self.frames
	}

	pub fn set_frames(&mut self, new_frames: Vec<Rectangle>) {
		self.frames = new_frames;
		self.restart();
	}

	pub fn frame_length(&self) -> i32 {
		self.frame_length
	}

	pub fn set_frame_length(&mut self, new_frame_length: i32) {
		self.frame_length = new_frame_length;
	}
}

impl Drawable for Animation {
	fn draw<P>(&self, ctx: &mut Context, params: P)
		where
			P: Into<DrawParams>,
	{
		let frame_clip = self.frames[self.current_frame];

		let mut params = params.into();

		params.clip = match params.clip {
			Some(mut clip) => {
				clip.x += frame_clip.x;
				clip.y += frame_clip.y;
				clip.width += frame_clip.width;
				clip.height += frame_clip.height;

				Some(clip)
			}
			None => Some(frame_clip),
		};

		self.texture.draw(ctx, params)
	}
}