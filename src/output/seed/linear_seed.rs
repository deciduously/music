//! The LinearSeed will ascend through a scale sequentially

use super::MusicSeed;
use crate::theory::{key::Key, piano_key::PianoKey};

#[derive(Default)]
pub struct LinearSeed(u8);

impl Iterator for LinearSeed {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.0;
        self.0 += 1;
        Some(ret)
    }
}

impl MusicSeed for LinearSeed {
    fn get_note(&mut self, _key: Key) -> PianoKey {
        unimplemented!()
    }
}
