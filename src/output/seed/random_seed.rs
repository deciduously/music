//! The RandomSeed will produce notes randomly

use super::MusicSeed;
use crate::theory::{key::Key, piano_key::PianoKey};
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};

pub struct RandomSeed(SmallRng);

impl Default for RandomSeed {
    fn default() -> Self {
        Self(SmallRng::from_entropy())
    }
}

impl MusicSeed for RandomSeed {
    fn get_note(&mut self, key: Key) -> PianoKey {
        let keys = key.all_keys();
        *keys.choose(&mut self.0).unwrap()
    }
}
