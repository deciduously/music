//! The `RandomSeed` will produce notes randomly.

use super::Seed;
use crate::theory::{key::Key, piano_key::PianoKey};
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};

pub struct Random(SmallRng);

impl Default for Random {
	fn default() -> Self {
		Self(SmallRng::from_entropy())
	}
}

impl Seed for Random {
	fn get_note(&mut self, key: Key) -> PianoKey {
		let keys = key.all_keys();
		*keys.choose(&mut self.0).unwrap()
	}
}
