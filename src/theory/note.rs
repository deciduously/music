//! A Note is the high-level abstraction for working with specific pitches

use super::{char_strs, interval::Interval, scale::Scale, semitone::Semitones};
use std::{
	fmt, io,
	ops::{Add, AddAssign},
	str::FromStr,
};

/// Notes are one of these letters, with C as the base note
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Letter {
	#[default]
	C = 0,
	D,
	E,
	F,
	G,
	A,
	B,
}

impl FromStr for Letter {
	type Err = io::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_uppercase().as_str() {
			"A" => Ok(Letter::A),
			"B" => Ok(Letter::B),
			"C" => Ok(Letter::C),
			"D" => Ok(Letter::D),
			"E" => Ok(Letter::E),
			"F" => Ok(Letter::F),
			"G" => Ok(Letter::G),
			_ => Err(io::Error::new(
				io::ErrorKind::InvalidInput,
				format!("{s} is not a valid note"),
			)),
		}
	}
}

impl Letter {
	/// Get the next highest base note
	fn inc(self) -> Self {
		use Letter::{A, B, C, D, E, F, G};
		match self {
			C => D,
			D => E,
			E => F,
			F => G,
			G => A,
			A => B,
			B => C,
		}
	}

	/// Get the distance from the base - C - as an Interval
	#[must_use]
	pub fn interval_from_c(self) -> Interval {
		use Interval::Unison;
		Scale::default()
			.get_intervals()
			.iter()
			.take(self as usize)
			.fold(Unison, |acc, i| acc + *i)
	}
}

/// An Accidental adjust the tone of a note by one semitone.
/// Flats go down, sharps go up
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Accidental {
	Flat,
	Sharp,
}

impl fmt::Display for Accidental {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use Accidental::{Flat, Sharp};
		let acc_str = match self {
			Flat => "♭",
			Sharp => "#",
		};
		write!(f, "{acc_str}")
	}
}

impl FromStr for Accidental {
	type Err = io::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"b" | "♭" => Ok(Accidental::Flat),
			"#" => Ok(Accidental::Sharp),
			_ => Err(io::Error::new(
				io::ErrorKind::InvalidInput,
				format!("{s} is not a valid accidental"),
			)),
		}
	}
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Note {
	pub accidental: Option<Accidental>,
	pub letter: Letter,
}

impl Note {
	#[must_use]
	pub fn interval_from_c(self) -> Interval {
		use Accidental::{Flat, Sharp};
		let ret = self.letter.interval_from_c();
		if let Some(acc) = self.accidental {
			match acc {
				Flat => return Interval::from(Semitones::from(i8::from(Semitones::from(ret)) - 1)),
				Sharp => return ret + Interval::Min2,
			}
		};
		ret
	}
	#[must_use]
	pub fn get_offset(self, other: Self) -> Interval {
		let self_interval_from_c = self.interval_from_c();
		let other_interval_from_c = other.interval_from_c();
		self_interval_from_c - other_interval_from_c
	}
	fn inc(&mut self) {
		use Accidental::{Flat, Sharp};
		use Letter::{B, E};
		if let Some(acc) = self.accidental {
			self.accidental = None;
			match acc {
				Sharp => {
					self.letter = self.letter.inc();
				},
				Flat => {},
			}
		} else {
			// check for special cases
			if self.letter == B || self.letter == E {
				self.letter = self.letter.inc();
			} else {
				self.accidental = Some(Sharp);
			}
		}
	}
}

impl fmt::Display for Note {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let acc_str = if let Some(a) = self.accidental {
			format!("{a}")
		} else {
			String::new()
		};
		write!(f, "{:?}{}", self.letter, acc_str)
	}
}

impl FromStr for Note {
	type Err = io::Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut char_strs = char_strs(s);
		// note will be first
		if let Some(letter) = char_strs.next() {
			let letter = Letter::from_str(letter)?;
			if let Some(accidental) = char_strs.next() {
				// check if it's valid
				let accidental = Accidental::from_str(accidental)?;
				return Ok(Self {
					letter,
					accidental: Some(accidental),
				});
			}
			return Ok(Self {
				letter,
				accidental: None,
			});
		}
		Err(io::Error::new(
			io::ErrorKind::InvalidInput,
			format!("{s} is not a valid note"),
		))
	}
}

impl From<Interval> for Note {
	// Take an interval from C
	fn from(i: Interval) -> Self {
		use Interval::Unison;
		let mut offset = Unison;
		// That's a series of Min2
		let scale = Scale::Chromatic.get_intervals();
		scale.iter().take(i as usize).for_each(|i| offset += *i);
		Note::default() + offset
	}
}

impl Add<Interval> for Note {
	type Output = Self;

	fn add(self, rhs: Interval) -> Self {
		let semitones = Semitones::from(rhs);
		let mut ret = self;
		for _ in 0..i8::from(semitones) {
			ret.inc();
		}
		ret
	}
}

impl AddAssign<Interval> for Note {
	fn add_assign(&mut self, rhs: Interval) {
		*self = *self + rhs;
	}
}
