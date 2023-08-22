//! An Interval represents a ratio between two Notes

use super::semitone::Semitones;
use std::ops::{Add, AddAssign, Sub};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Interval {
	Unison = 0,
	Min2,
	Maj2,
	Min3,
	Maj3,
	Perfect4,
	Tritone,
	Perfect5,
	Min6,
	Maj6,
	Min7,
	Maj7,
	Octave,
}

impl From<Semitones> for Interval {
	fn from(s: Semitones) -> Self {
		use Interval::{
			Maj2, Maj3, Maj6, Maj7, Min2, Min3, Min6, Min7, Octave, Perfect4, Perfect5, Tritone,
			Unison,
		};
		let int_semitones = i8::from(s);
		match int_semitones {
			0 => Unison,
			1 => Min2,
			2 => Maj2,
			3 => Min3,
			4 => Maj3,
			5 => Perfect4,
			6 => Tritone,
			7 => Perfect5,
			8 => Min6,
			9 => Maj6,
			10 => Min7,
			11 => Maj7,
			// If 12 or higher, will wrap around - intervals are octave-agnostic
			_ => Interval::from(Semitones::from(int_semitones % Octave as i8)),
		}
	}
}

impl From<Interval> for i8 {
	fn from(i: Interval) -> Self {
		Semitones::from(i).into()
	}
}

impl Add for Interval {
	type Output = Self;
	fn add(self, rhs: Self) -> Self {
		Interval::from(Semitones::from(
			i8::from(self) + i8::from(rhs) % Interval::Octave as i8,
		))
	}
}

impl Sub for Interval {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self {
		let mut delta = i8::from(self) - i8::from(rhs);
		if delta < 0 {
			delta += Interval::Octave as i8;
		};
		Interval::from(Semitones::from(delta))
	}
}

impl AddAssign for Interval {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}
