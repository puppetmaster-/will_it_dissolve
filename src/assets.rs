use tetra::Context;
use tetra::graphics::{Texture, Rectangle, Shader};
use tetra::graphics::shader::DEFAULT_VERTEX_SHADER;
use std::collections::HashMap;
use crate::utils::animation::Animation;

type TextureHashmap = HashMap<TextureName, Texture>;
type AnimationHashmap = HashMap<AnimationName, Animation>;
type ShaderHashmap = HashMap<ShaderName, Shader>;
//type SymbolsHashmap = HashMap<SymbolName, Texture>;

pub struct Assets{
	textures: TextureHashmap,
	animations: AnimationHashmap,
	shaders: ShaderHashmap,
	//symbols: SymbolsHashmap,
}

impl Assets{
	pub fn init(ctx: &mut Context) -> tetra::Result<Self>{
		Ok(Assets{
			textures: build_textures(ctx)?,
			animations: build_animations(ctx)?,
			shaders: build_shaders(ctx)?,
			//symbols: build_symbols(ctx)?,
		})
	}

	pub fn get_texture(&self, name: &TextureName) -> &Texture{
		&self.textures[&name]
	}
	
	pub fn get_animation(&self, name: &AnimationName) -> &Animation{
		&self.animations[&name]
	}

	pub fn get_shader(&self, name: ShaderName) -> Shader{
		self.shaders[&name].clone()
	}

	/*
	pub fn get_symbol(&self, name: &SymbolName) -> &Texture{
		&self.symbols[&name]
	}*/
	
	pub fn update(&mut self){
		for animation in self.animations.values_mut(){
			animation.tick();
		}
	}
	
}

fn build_textures(ctx: &mut Context) ->tetra::Result<TextureHashmap>{
	Ok([
		(TextureName::Pic0On, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_14.png"))?),
		(TextureName::Pic0Off, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_15.png"))?),
		(TextureName::Pic1On, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_00.png"))?),
		(TextureName::Pic1Off, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_01.png"))?),
		(TextureName::Pic2On, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_02.png"))?),
		(TextureName::Pic2Off, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_03.png"))?),
		(TextureName::Pic3On, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_04.png"))?),
		(TextureName::Pic3Off, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_05.png"))?),
		(TextureName::Pic4On, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_06.png"))?),
		(TextureName::Pic4Off, Texture::from_file_data(ctx, include_bytes!("../assets/art/art_07.png"))?),
		(TextureName::Black, Texture::from_file_data(ctx, include_bytes!("../assets/art/black.png"))?),
		(TextureName::Future, Texture::from_file_data(ctx, include_bytes!("../assets/art/button_00.png"))?),
		(TextureName::Back, Texture::from_file_data(ctx, include_bytes!("../assets/art/button_01.png"))?),
		(TextureName::Next, Texture::from_file_data(ctx, include_bytes!("../assets/art/button_02.png"))?),
		(TextureName::JamLogo, Texture::from_file_data(ctx, include_bytes!("../assets/art/open-jam-logo.png"))?),
		(TextureName::Logo, Texture::from_file_data(ctx, include_bytes!("../assets/art/logo.png"))?),
		(TextureName::Thx, Texture::from_file_data(ctx, include_bytes!("../assets/art/thx.png"))?),
		].iter().cloned().collect()
	)
}

fn build_animations(ctx: &mut Context) ->tetra::Result<AnimationHashmap>{
	let tileset = Texture::from_file_data(ctx, include_bytes!("../assets/art/art.png"))?;
	let button_tileset = Texture::from_file_data(ctx, include_bytes!("../assets/art/button.png"))?;
	let symbol_tileset = Texture::from_file_data(ctx, include_bytes!("../assets/art/symbol.png"))?;
	Ok([
		(AnimationName::Plus, Animation::new(tileset.clone(),Rectangle::row(0.0, 32.0, 16.0, 16.0).take(4).collect(), 5)),
		(AnimationName::Minus, Animation::new(tileset.clone(),Rectangle::row(64.0, 32.0, 16.0, 16.0).take(4).collect(), 5)),
		(AnimationName::Tile43, Animation::new(tileset.clone(), Rectangle::row(0.0, 48.0, 16.0, 16.0).take(5).collect(), 5).stop()),
		(AnimationName::Tile32, Animation::new(tileset.clone(), Rectangle::row(0.0, 64.0, 16.0, 16.0).take(5).collect(), 5).stop()),
		(AnimationName::Tile21, Animation::new(tileset.clone(), Rectangle::row(0.0, 80.0, 16.0, 16.0).take(5).collect(), 5).stop()),
		(AnimationName::Tile10, Animation::new(tileset.clone(), Rectangle::row(0.0, 96.0, 16.0, 16.0).take(5).collect(), 5).stop()),
		(AnimationName::Tile41, Animation::new(tileset.clone(), Rectangle::row(0.0, 112.0, 16.0, 16.0).take(5).collect(), 5).stop()),
		(AnimationName::Tile14, Animation::new(tileset.clone(), Rectangle::row(0.0, 128.0, 16.0, 16.0).take(5).collect(), 5).stop()),
		(AnimationName::Future, Animation::new(button_tileset.clone(),Rectangle::row(0.0, 16.0, 32.0, 16.0).take(6).collect(), 6)),
		(AnimationName::Back, Animation::new(button_tileset.clone(),Rectangle::row(0.0, 32.0, 32.0, 16.0).take(6).collect(), 6)),
		(AnimationName::Next, Animation::new(button_tileset.clone(),Rectangle::row(0.0, 48.0, 32.0, 16.0).take(6).collect(), 6)),
		(AnimationName::Action, Animation::new(symbol_tileset.clone(),Rectangle::row(0.0, 0.0, 8.0, 8.0).take(4).collect(), 6)),
		].iter().cloned().collect()
	)
}

fn build_shaders(ctx: &mut Context) ->tetra::Result<ShaderHashmap>{
	Ok([
		//(ShaderName::LevelTransition, Shader::fragment(ctx,"D:/RustProjects/will_it_dissolve/assets/art/level_change.frag")?)
		(ShaderName::LevelTransition, Shader::from_string(ctx, DEFAULT_VERTEX_SHADER, include_str!("../assets/shader/level_change.frag"))?)
		].iter().cloned().collect()
	)
}
/*
fn build_symbols(ctx: &mut Context) ->tetra::Result<SymbolsHashmap>{
	Ok([
		(SymbolName::SymbolClick, Texture::from_file_data(ctx, include_bytes!("../assets/art/symbol_14.png"))?),
		].iter().cloned().collect()
	)
}*/


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextureName {
	Pic0On,
	Pic0Off,
	Pic1On,
	Pic1Off,
	Pic2On,
	Pic2Off,
	Pic3On,
	Pic3Off,
	Pic4On,
	Pic4Off,
	Black,
	Future,
	Back,
	Next,
	JamLogo,
	Logo,
	Thx,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnimationName {
	Plus,
	Minus,
	Back,
	Future,
	Next,
	Action,
	Tile43,
	Tile32,
	Tile21,
	Tile10,
	Tile14,
	Tile41,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShaderName{
	LevelTransition,
}

impl TextureName {
	pub fn from_str(s: &str) -> TextureName {
		match s {
			"Pic0On" => TextureName::Pic0On,
			"Pic0Off" => TextureName::Pic0Off,
			"Pic1On" => TextureName::Pic1On,
			"Pic1Off" => TextureName::Pic1Off,
			"Pic2On" => TextureName::Pic2On,
			"Pic2Off" => TextureName::Pic2Off,
			"Pic3On" => TextureName::Pic3On,
			"Pic3Off" => TextureName::Pic3Off,
			"Pic4On" => TextureName::Pic4On,
			"Pic4Off" => TextureName::Pic4Off,
			_ => TextureName::Pic0On,
		}
	}
	/*
	pub fn as_str(&self) -> &'static str {
		match s {
			TextureName::Pic0On => "Pic0On" ,
		}
	}*/

	/*
	#[derive(Debug, Clone, PartialEq, Eq, Hash)]
	pub enum SymbolName {
		SymbolClick,
	}*/
}