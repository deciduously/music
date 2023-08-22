//! All the different strategies for generating music.

use crate::theory::{key::Key, piano_key::PianoKey};

mod linear;
mod random;

pub use linear::Linear;
pub use random::Random;

/// To be a seed, types simply must implement this single method
pub trait Seed: Send {
	fn get_note(&mut self, key: Key) -> PianoKey;
}
