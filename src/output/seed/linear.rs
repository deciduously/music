//! The `LinearSeed` will ascend through a scale sequentially.

use super::Seed;
use crate::theory::{key::Key, piano_key::PianoKey};

#[derive(Default)]
pub struct Linear(u8);

impl Iterator for Linear {
	type Item = u8;
	fn next(&mut self) -> Option<Self::Item> {
		let ret = self.0;
		self.0 += 1;
		Some(ret)
	}
}

impl Seed for Linear {
	fn get_note(&mut self, _key: Key) -> PianoKey {
		unimplemented!()
	}
}
