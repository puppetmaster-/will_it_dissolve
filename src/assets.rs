use tetra::Context;
use tetra::graphics::{Texture, Rectangle, Animation};
use std::collections::HashMap;

type TextureHashmap = HashMap<TextureName, Texture>;
type AnimationHashmap = HashMap<AnimationName, Animation>;
//type SymbolsHashmap = HashMap<SymbolName, Texture>;

pub struct Assets{
	textures: TextureHashmap,
	animations: AnimationHashmap,
	//symbols: SymbolsHashmap,
}

impl Assets{
	pub fn init(ctx: &mut Context) -> tetra::Result<Self>{
		Ok(Assets{
			textures: build_textures(ctx)?,
			animations: build_animations(ctx)?,
			//symbols: build_symbols(ctx)?,
		})
	}

	pub fn get_texture(&self, name: &TextureName) -> &Texture{
		&self.textures[&name]
	}
	
	pub fn get_animation(&self, name: &AnimationName) -> &Animation{
		&self.animations[&name]
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
		(AnimationName::Plus, Animation::new(tileset.clone(),Rectangle::row(0.0, 16.0, 16.0, 16.0).take(2).collect(), 6)),
		(AnimationName::Minus, Animation::new(tileset.clone(),Rectangle::row(32.0, 16.0, 16.0, 16.0).take(2).collect(), 6)),
		(AnimationName::Future, Animation::new(button_tileset.clone(),Rectangle::row(0.0, 16.0, 32.0, 16.0).take(6).collect(), 6)),
		(AnimationName::Back, Animation::new(button_tileset.clone(),Rectangle::row(0.0, 32.0, 32.0, 16.0).take(6).collect(), 6)),
		(AnimationName::Next, Animation::new(button_tileset.clone(),Rectangle::row(0.0, 48.0, 32.0, 16.0).take(6).collect(), 6)),
		(AnimationName::Action, Animation::new(symbol_tileset.clone(),Rectangle::row(0.0, 0.0, 8.0, 8.0).take(4).collect(), 6)),
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
}
/*
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymbolName {
	SymbolClick,
}*/
