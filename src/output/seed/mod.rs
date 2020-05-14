//! All the different strategies for generating music

use crate::theory::{key::Key, piano_key::PianoKey};

mod linear_seed;
mod random_seed;

pub use linear_seed::LinearSeed;
pub use random_seed::RandomSeed;

/// To be a seed, types simply must implement this single method
pub trait MusicSeed: Send {
    fn get_note(&mut self, key: Key) -> PianoKey;
}
