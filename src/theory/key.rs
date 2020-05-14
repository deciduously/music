//! A Key dictates which notes are currently allowed to be played.
//! It is defined by a base note, a scale up to the next octave, and a number of octaves

use super::{interval::Interval, note::Note, piano_key::PianoKey, scale::Scale};
use std::{fmt, str::FromStr};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Key {
    pub base_note: PianoKey,
    pub octaves: u8,
    pub scale: Scale,
}

impl Key {
    pub fn new(scale: Scale, base_note: PianoKey, octaves: u8) -> Self {
        let octaves = if base_note.octave + octaves > 8 {
            PianoKey::max_octave() - base_note.octave
        } else {
            octaves
        };
        Self {
            base_note,
            octaves,
            scale,
        }
    }
    /// Get a listing of all possible PianoKeys in this Key
    pub fn all_keys(self) -> Vec<PianoKey> {
        let notes = self.get_notes();
        let mut ret = Vec::new();
        for i in 0..self.octaves {
            notes.iter().for_each(|n| {
                ret.push(
                    PianoKey::from_str(&format!("{}{}", *n, i + self.base_note.octave))
                        .unwrap_or_else(|_| {
                            PianoKey::from_str(&format!("{}{}", *n, PianoKey::max_octave()))
                                .unwrap()
                        }),
                )
            });
        }
        ret
    }

    pub fn get_notes(self) -> Vec<Note> {
        let mut ret = vec![self.base_note.note];
        let mut offset = Interval::Unison;
        self.scale.get_intervals().iter().for_each(|i| {
            offset += *i;
            ret.push(self.base_note.note + offset)
        });
        ret
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let notes = self.get_notes();
        let mut ret = String::from("[ ");
        notes.iter().for_each(|n| ret.push_str(&format!("{} ", *n)));
        ret.push_str("]");
        write!(f, "{}", ret)
    }
}
