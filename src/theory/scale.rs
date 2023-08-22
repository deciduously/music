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
		use Interval::{Maj2, Min2};
		vec![Maj2, Maj2, Min2, Maj2, Maj2, Maj2, Min2]
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Length {
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
	///
	/// # Panics
	///
	/// This function would panic if a bad value is passed to `PianoKey::from_str`.
	#[must_use]
	pub fn circle_of_fifths(mode: Mode) -> Vec<Key> {
		let mut ret = Vec::new();
		// Start with C
		let mut current_base = Note::default();
		// Increment by fifths and push to vector
		for _ in 0..Length::Dodecatonic as usize {
			ret.push(Key::new(
				Scale::Diatonic(mode),
				PianoKey::from_str(&format!("{current_base}4")).unwrap(),
				1,
			));
			current_base += Interval::Perfect5;
		}
		ret
	}
	/// Produce the intervals that make up this scale
	#[must_use]
	pub fn get_intervals(self) -> Vec<Interval> {
		use Interval::{Maj2, Maj3, Min2};
		use Scale::{Chromatic, Diatonic, Tetratonic};
		match self {
			Chromatic => [Min2]
				.iter()
				.cycle()
				.take(Length::Dodecatonic as usize)
				.copied()
				.collect::<Vec<Interval>>(),
			Diatonic(mode) => Mode::base_intervals()
				.iter()
				.cycle()
				.skip(mode as usize)
				.take(Length::Heptatonic as usize)
				.copied()
				.collect::<Vec<Interval>>(),
			Tetratonic => vec![Min2, Maj2, Maj3],
		}
	}
}

impl FromStr for Scale {
	type Err = io::Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use Mode::{Aeolian, Dorian, Ionian, Locrian, Lydian, Mixolydian, Phrygian};
		use Scale::{Chromatic, Diatonic, Tetratonic};
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
		use Scale::{Chromatic, Diatonic, Tetratonic};
		let s = match self {
			Chromatic | Tetratonic => format!("{self:?} scale").to_lowercase(),
			Diatonic(mode) => {
				use Mode::{Aeolian, Ionian};
				match mode {
					Aeolian => "minor scale".into(),
					Ionian => "major scale".into(),
					_ => format!("{mode:?} mode"),
				}
			},
		};
		write!(f, "{s}")
	}
}
