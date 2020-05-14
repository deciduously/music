//! A Scale is a series of notes in a single octave, along a given set of intervals

use super::{interval::Interval, key::Key, note::Note, piano_key::PianoKey};
use std::{fmt, io, str::FromStr};

/// A Mode is an offset starting note along the line of Diatonic intervals
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Mode {
    Ionian = 0,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}

impl Mode {
    fn base_intervals() -> Vec<Interval> {
        use Interval::*;
        vec![Maj2, Maj2, Min2, Maj2, Maj2, Maj2, Min2]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScaleLength {
    Tetratonic = 4,
    Heptatonic = 7,
    Dodecatonic = 12,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Scale {
    Chromatic,
    Diatonic(Mode),
    Tetratonic,
}

impl Default for Scale {
    fn default() -> Self {
        Scale::Diatonic(Mode::Ionian)
    }
}

impl Scale {
    /// Produce a scale for each member of the circle of fifths
    pub fn circle_of_fifths(mode: Mode) -> Vec<Key> {
        let mut ret = Vec::new();
        // Start with C
        let mut current_base = Note::default();
        // Increment by fifths and push to vector
        for _ in 0..ScaleLength::Dodecatonic as usize {
            ret.push(Key::new(
                Scale::Diatonic(mode),
                PianoKey::from_str(&format!("{}4", current_base)).unwrap(),
                1,
            ));
            current_base += Interval::Perfect5;
        }
        ret
    }
    /// Produce the intervals that make up this scale
    pub fn get_intervals(self) -> Vec<Interval> {
        use Interval::*;
        use Scale::*;
        match self {
            Chromatic => [Min2]
                .iter()
                .cycle()
                .take(ScaleLength::Dodecatonic as usize)
                .copied()
                .collect::<Vec<Interval>>(),
            Diatonic(mode) => Mode::base_intervals()
                .iter()
                .cycle()
                .skip(mode as usize)
                .take(ScaleLength::Heptatonic as usize)
                .copied()
                .collect::<Vec<Interval>>(),
            Tetratonic => vec![Min2, Maj2, Maj3],
        }
    }
}

impl FromStr for Scale {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Mode::*;
        use Scale::*;
        match s.to_uppercase().as_str() {
            "IONIAN" | "MAJOR" => Ok(Diatonic(Ionian)),
            "DORIAN" => Ok(Diatonic(Dorian)),
            "PHRYGIAN" => Ok(Diatonic(Phrygian)),
            "LYDIAN" => Ok(Diatonic(Lydian)),
            "MIXOLYDIAN" => Ok(Diatonic(Mixolydian)),
            "AEOLIAN" | "MINOR" => Ok(Diatonic(Aeolian)),
            "LOCRIAN" => Ok(Diatonic(Locrian)),
            "CHROMATIC" => Ok(Chromatic),
            "TETRATONIC" => Ok(Tetratonic),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unknown scale")),
        }
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Scale::*;
        let s = match self {
            Chromatic | Tetratonic => format!("{:?} scale", self).to_lowercase(),
            Diatonic(mode) => {
                use Mode::*;
                match mode {
                    Aeolian => "minor scale".into(),
                    Ionian => "major scale".into(),
                    _ => format!("{:?} mode", mode),
                }
            }
        };
        write!(f, "{}", s)
    }
}
