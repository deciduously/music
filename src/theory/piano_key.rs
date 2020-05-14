//! A PianoKey represents a single key on a piano (aptly named, huh?)

use super::{char_strs, note::Note};
use std::{fmt, io, str::FromStr};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct PianoKey {
    pub note: Note,
    pub octave: u8,
}

impl fmt::Display for PianoKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.note, self.octave)
    }
}

impl FromStr for PianoKey {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // It makes sense to get the letter to Intervals
        if let Some(octave) = char_strs(s).last() {
            if let Ok(octave) = octave.parse::<u8>() {
                let note = Note::from_str(&s[0..s.len() - 1])?;
                if octave <= 8 {
                    Ok(Self { note, octave })
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{} is too high!", octave),
                    ))
                }
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("{} is too high for this keyboard", octave),
                ))
            }
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} is not a valid note", s),
            ))
        }
    }
}

impl PianoKey {
    pub fn new(s: &str) -> Result<Self, io::Error> {
        Self::from_str(s)
    }
    /// Get the highest allowed octave - hard-coded to be 8
    pub fn max_octave() -> u8 {
        8
    }
}
