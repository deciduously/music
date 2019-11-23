use rand::random;
use std::ops::{Add, AddAssign, Div, Mul};

#[derive(Default)]
struct RandomBytes;

impl RandomBytes {
    fn new() -> Self {
        Self::default()
    }
}

impl Iterator for RandomBytes {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        Some(random::<Self::Item>())
    }
}

type Hertz = f64;
const STANDARD_PITCH: Hertz = 440.0;
const C_ZERO: Hertz = 16.352;

struct Cents(f64);
struct Semitones(i8);
const SEMITONE_CENTS: Cents = Cents(100.0);

impl Div for Cents {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Cents(f64::from(self) / f64::from(rhs))
    }
}

impl From<Cents> for f64 {
    fn from(cents: Cents) -> Self {
        cents.0
    }
}

impl From<Semitones> for i8 {
    fn from(semitones: Semitones) -> Self {
        semitones.0
    }
}

impl From<Semitones> for Cents {
    fn from(semitones: Semitones) -> Self {
        Cents(i8::from(semitones) as f64 * f64::from(SEMITONE_CENTS))
    }
}

#[derive(Debug, Clone, Copy)]
enum Note {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Default for Note {
    fn default() -> Self {
        Note::C
    }
}

#[derive(Debug, Clone, Copy)]
enum Accidental {
    Flat,
    Sharp,
}

#[derive(Default, Debug, Clone, Copy)]
struct StandardPitch {
    accidental: Option<Accidental>,
    note: Note,
    octave: u8,
}

impl StandardPitch {
    fn new(s: &str) -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Copy)]
struct Pitch {
    frequency: Hertz,
}

impl Pitch {
    fn new(frequency: Hertz) -> Self {
        Self { frequency }
    }
}

impl Default for Pitch {
    fn default() -> Self {
        Self {
            frequency: STANDARD_PITCH,
        }
    }
}

impl AddAssign<Cents> for Pitch {
    fn add_assign(&mut self, cents: Cents) {
        self.frequency *= 2.0f64.powf((cents / Cents::from(Interval::Octave)).into())
    }
}

impl AddAssign<Semitones> for Pitch {
    fn add_assign(&mut self, semitones: Semitones) {
        *self += Cents::from(semitones)
    }
}

impl AddAssign<Interval> for Pitch {
    fn add_assign(&mut self, i: Interval) {
        *self += Cents::from(i)
    }
}

#[derive(Debug, Clone, Copy)]
enum Interval {
    Unison,
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

impl From<Interval> for Semitones {
    fn from(i: Interval) -> Self {
        use Interval::*;
        let x = match i {
            Unison => 0,
            Min2 => 1,
            Maj2 => 2,
            Min3 => 3,
            Maj3 => 4,
            Perfect4 => 5,
            Tritone => 6,
            Perfect5 => 7,
            Min6 => 8,
            Maj6 => 9,
            Min7 => 10,
            Maj7 => 11,
            Octave => 12,
        };
        Semitones(x)
    }
}

impl From<Interval> for Cents {
    fn from(i: Interval) -> Self {
        Semitones::from(i).into()
    }
}

type Octave = [Interval; 7];

#[derive(Debug, Clone, Copy)]
enum Mode {
    Ionian = 0,
    Dorian = 1,
    Phrygian = 2,
    Lydian = 3,
    Mixolydian = 4,
    Aeolian = 5,
    Locrian = 6,
}

impl Mode {
    fn base_intervals() -> Octave {
        use Interval::*;
        [Maj2, Maj2, Min2, Maj2, Maj2, Maj2, Min2]
    }
    //fn get_intervals(&self) -> impl Iterator {
    //    let intervals = Mode::base_intervals();
    //    Mode::base_intervals().skip(self as u8 as usize).cycle()
    //}
}

#[derive(Debug, Clone, Copy)]
enum Scale {
    Diatonic(Mode),
}

impl Scale {
    fn get_intervals(self) -> Octave {
        // TODO this needs to be a method, come here next!
        // have THIS calculate an impl Iterator (or impl Scale??)
        use Interval::*;
        use Scale::*;
        match self {
            Diatonic(_) => Mode::base_intervals(),
        }
    }
    //fn get_interval(&self, n: u8) -> Interval {
    //    use Scale::*;
    //    let offset = match self {
    //        Aeolian => 5,
    //        Ionian => 0,
    //    };
    //    let c = self.get_intervals();
    //    let idx = n as usize + offset % c.size_hint().0;
    //    c.nth(idx).unwrap()
    // }
}

fn main() {
    let mut pitch = Pitch::new(C_ZERO);
    println!("{:?}", pitch); // Pitch { frequency: 16.352 }
    for _ in 0..4 {
        pitch += Semitones::from(Interval::Octave);
    } // add 4 octaves - C0 -> C4
    println!("{:?}", pitch); // Pitch { frequency: 261.632 }
    pitch += Interval::Maj6; // C4 -> A4
    println!("{:?}", pitch); // Pitch { frequency: 440.010821831319 } // close enough
}
