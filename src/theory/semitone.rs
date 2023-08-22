//! A Semitone divides an octave into 12 equal parts

use super::interval::Interval;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Semitones(i8);

impl From<i8> for Semitones {
	fn from(i: i8) -> Self {
		Self(i)
	}
}

impl From<Semitones> for i8 {
	fn from(s: Semitones) -> Self {
		s.0
	}
}

impl From<Interval> for Semitones {
	fn from(i: Interval) -> Self {
		Semitones(i as i8)
	}
}
