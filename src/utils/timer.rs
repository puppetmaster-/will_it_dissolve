
use std::time::{Duration,Instant};

#[derive(Debug)]
pub struct Timer{
	duration: Duration,
	pub finished: bool,
	start_time: Instant,
	pub counter: u32,
}

impl Timer{
	pub fn new(duration: u64)-> Timer{
		Timer{
			duration: Duration::from_secs(duration),
			finished: false,
			start_time: Instant::now(),
			counter: 0,
		}
	}
	
	pub fn update(&mut self){
		if !self.finished{
			let current_time = Instant::now();
			let elapsed = current_time - self.start_time;
			if elapsed >= self.duration{
				self.finished = true;
			}
		}
	}
	
	#[allow(dead_code)]
	pub fn set_duration(&mut self, duration: u64){
		self.duration = Duration::from_secs(duration);
	}
	
	pub fn restart(&mut self){
		self.start_time = Instant::now();
		self.finished = false;
		self.counter +=1;
	}
	
	pub fn get_value(&self)-> f32{
		let current_time = Instant::now();
		let elapsed = current_time - self.start_time;
		if elapsed < self.duration{
			1.0 * (100.0 / self.duration.as_millis() as f32 * elapsed.as_millis() as f32) / 100.0
		}else{
			1.0
		}
	}
}

