//! A Hertz measures the rate of a recurring phenomena in cycles per second
//! This module also houses several standardized constants in Hertz

use std::ops::{MulAssign, Sub};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Hertz(f64);

impl Hertz {
    /// Get the absolute value of a Hertz
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }
}

impl Sub for Hertz {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(f64::from(self) - f64::from(rhs))
    }
}

impl From<Hertz> for f64 {
    fn from(h: Hertz) -> Self {
        h.0
    }
}

impl From<f64> for Hertz {
    fn from(f: f64) -> Self {
        Self(f)
    }
}

impl MulAssign<f64> for Hertz {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
    }
}

/// The standard tuning pitch, per ISO 16
pub const STANDARD_PITCH: Hertz = Hertz(440.0);

/// C4 on a piano is also standardized
pub const MIDDLE_C: Hertz = Hertz(261.626);

/// C0 - the lowest supported note
pub const C_ZERO: Hertz = Hertz(16.352);

/// The sample rate used for the analog-to-digital conversion
pub const SAMPLE_RATE: Hertz = Hertz(48_000.0);
