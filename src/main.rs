#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate log;
extern crate simple_logger;

mod scenes;
mod models;
mod utils;
mod assets;
mod button;
mod tile;
mod constants;

use tetra::ContextBuilder;
use std::rc::Rc;
use std::cell::{RefCell};

use crate::scenes::manager::SceneManager;
use crate::scenes::title::TitleScene;
use crate::assets::Assets;
use crate::models::config::{load_config};

fn main() -> tetra::Result {
	color_backtrace::install();
	simple_logger::init().unwrap();
	let config = Rc::new(load_config(include_str!("../assets/config/config.ron")));
	let version = config.version();
	ContextBuilder::new(format!("{} v{}", config.titel, version).as_str(), config.window_width, config.window_height)
		.window_scale(config.window_scale)
		.maximized(config.maximized)
		.fullscreen(config.fullscreen)
		.resizable(config.resizable)
		.scaling(config.scaling)
		.vsync(config.vsync)
		.show_mouse(config.show_mouse)
		.quit_on_escape(config.quit_on_escape)
	.build()?
		.run_with(|ctx| {
			let assets = Rc::new(RefCell::new(Assets::init(ctx)?));
			let scene = TitleScene::new(ctx,config,assets)?;
			Ok(SceneManager::new(Box::new(scene)))
		})
}

