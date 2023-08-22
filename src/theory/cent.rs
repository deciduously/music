//! A Cent subdivides a Semitone into 100 equal portions

use super::{interval::Interval, semitone::Semitones};
use std::ops::Div;

const SEMITONE_CENTS: Cents = Cents(100.0);

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Cents(f64);

impl From<f64> for Cents {
	fn from(f: f64) -> Self {
		Cents(f)
	}
}

impl From<Cents> for f64 {
	fn from(c: Cents) -> Self {
		c.0
	}
}

impl From<Semitones> for Cents {
	fn from(s: Semitones) -> Self {
		Cents(f64::from(i8::from(s)) * f64::from(SEMITONE_CENTS))
	}
}

impl From<Interval> for Cents {
	fn from(i: Interval) -> Self {
		Semitones::from(i).into()
	}
}

impl Div for Cents {
	type Output = Self;

	fn div(self, rhs: Self) -> Self {
		Cents(f64::from(self) / f64::from(rhs))
	}
}
