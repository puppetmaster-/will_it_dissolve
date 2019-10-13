use crate::models::level::{Level,load_level};

pub struct Levelmanager{
	current_level: usize,
	levels: Vec<Level>,
}

impl Levelmanager{
	pub fn new(starting_level: usize) -> tetra::Result<Levelmanager>{
		Ok(Levelmanager{
			current_level: starting_level,
			levels: load_levels(),
		})
	}

	pub fn get_current_level(&self)-> &Level{
		&self.levels[self.current_level-1]
	}

	pub fn advance_next_level(&mut self)-> bool{
		if self.current_level < self.levels.len(){
			self.current_level +=1;
			return true;
		}
		false
	}
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