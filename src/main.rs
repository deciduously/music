use rand::random;
use std::{io, str::FromStr};

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

type Cents = f64;
type Semitones = i8;
const SEMITONE_CENTS: Cents = 100.0;
const OCTAVE_SEMITONES: Semitones = 12; // TODO use Interval::Octave
const OCTAVE_CENTS: Cents = SEMITONE_CENTS * OCTAVE_SEMITONES as f64; // 1200.0

#[derive(Debug)]
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

#[derive(Debug)]
enum Accidental {
    Flat,
    Sharp,
}

#[derive(Default, Debug)]
struct SPN {
    accidental: Option<Accidental>,
    note: Note,
    octave: u8,
}

impl SPN {
    fn new(s: &str) -> Self {
        Self::default()
    }
}

#[derive(Debug)]
struct Pitch {
    frequency: Hertz,
}

impl Pitch {
    fn new(frequency: Hertz) -> Self {
        Self { frequency }
    }
    fn add_cents(&mut self, cents: Cents) {
        self.frequency *= 2.0f64.powf(cents / OCTAVE_CENTS);
    }
    fn add_semitones(&mut self, semitones: Semitones) {
        self.add_cents(semitones as f64 * SEMITONE_CENTS);
    }
}

impl Default for Pitch {
    fn default() -> Self {
        Self {
            frequency: STANDARD_PITCH,
        }
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

impl Interval {
    fn cents(&self) -> Cents {
        self.semitones() as f64 * SEMITONE_CENTS
    }
    fn semitones(&self) -> Semitones {
        use Interval::*;
        match self {
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
        }
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
    pitch.add_semitones(OCTAVE_SEMITONES * 4); // add 4 octaves - C0 -> C4
    println!("{:?}", pitch); // Pitch { frequency: 261.632 }
    pitch.add_semitones(9); // C4 -> A4
    println!("{:?}", pitch); //
}
