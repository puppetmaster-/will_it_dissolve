use std::ops::Not;

pub mod timer;
pub mod tween;
pub mod particle;
pub mod mouse;
pub mod animation;


// just for fun and learning
#[derive(Clone,Copy,Debug)]
pub enum Disabled {
	On,
	Off,
}

impl From<Disabled> for bool {
	fn from(f: Disabled) -> bool {
		match f {
			Disabled::On => true,
			Disabled::Off => false,
		}
	}
}

impl Not for Disabled {
	type Output = Disabled;
	fn not(self) -> Self::Output {
		match self {
			Disabled::Off => Disabled::On,
			Disabled::On => Disabled::Off,
		}
	}
}