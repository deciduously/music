//! A Pitch is a domain-specific wrapper around Hertz, representing a musical pitch at a set frequency

use super::{cent::Cents, hertz::*, interval::Interval, piano_key::PianoKey, semitone::Semitones};
use rodio::source::SineWave;
use std::ops::AddAssign;

#[derive(Debug, Clone, Copy, PartialOrd)]
pub struct Pitch(Hertz);

impl Pitch {
    pub fn new(frequency: Hertz) -> Self {
        Self(frequency)
    }
}

impl Default for Pitch {
    fn default() -> Self {
        Self(STANDARD_PITCH)
    }
}

impl From<Pitch> for f64 {
    fn from(p: Pitch) -> Self {
        p.0.into()
    }
}

impl AddAssign<Cents> for Pitch {
    // Clippy notices you multiplying inside an AddAssign block and helpfully yells about it
    // You actually do mean it here, though
    #[allow(clippy::suspicious_op_assign_impl)]
    fn add_assign(&mut self, rhs: Cents) {
        self.0 *= 2.0f64.powf((rhs / Cents::from(Interval::Octave)).into())
    }
}

impl PartialEq for Pitch {
    fn eq(&self, other: &Pitch) -> bool {
        let tolerance = Hertz::from(0.1);
        let difference = (self.0 - other.0).abs();
        difference < tolerance
    }
}

impl AddAssign<Semitones> for Pitch {
    fn add_assign(&mut self, rhs: Semitones) {
        *self += Cents::from(rhs)
    }
}

impl From<Pitch> for SineWave {
    fn from(p: Pitch) -> Self {
        SineWave::new(f64::from(p) as u32)
    }
}

impl AddAssign<Interval> for Pitch {
    fn add_assign(&mut self, rhs: Interval) {
        *self += Cents::from(rhs)
    }
}

impl From<PianoKey> for Pitch {
    fn from(sp: PianoKey) -> Self {
        use Interval::*;
        let mut ret = Pitch::new(C_ZERO);
        // Add octaves
        for _ in 0..sp.octave {
            ret += Octave;
        }
        // Add note offset
        ret += sp.note.letter.interval_from_c();
        ret
    }
}
