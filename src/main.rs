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
const MIDDLE_C: Hertz = 261.626;

type Cents = f64;
const SEMITONE_CENTS: Cents = 100.0;
const OCTAVE_SEMITONES: i32 = 12;
const OCTAVE_CENTS: Cents = SEMITONE_CENTS * OCTAVE_SEMITONES as f64; // 1200.0

#[derive(Debug, Clone, Copy)]
struct PianoKey {
    note: u8,
    octave: u8,
}

impl PianoKey {
    fn new(note: u8, octave: u8) -> Self {
        PianoKey { note, octave }
    }
}

impl FromStr for PianoKey {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use io::{Error, ErrorKind::*};
        if s.len() != 2 {
            return Err(Error::new(InvalidInput, "Must be two characters long"));
        }
        let mut chars = s.chars();
        if let Some(note) = chars.next() {
            let char_note = note as char;
            if !char_note.is_uppercase() {
                Err(Error::new(InvalidData, "First character must be a letter"))
            } else if let Some(octave) = chars.next() {
                // Turn octave to integer
                let integer_octave = octave as u8 - b'0';
                if integer_octave > 8 {
                    return Err(Error::new(InvalidData, "Second character must be 0-8"));
                }
                // Turn note to integer
                let integer_note = char_note as u8 - b'A';
                // Make sure its a real note
                if integer_note <= 8 {
                    // Success!!
                    Ok(PianoKey::new(integer_note, integer_octave))
                } else {
                    Err(Error::new(InvalidData, "First character must be A-G"))
                }
            } else {
                Err(Error::new(InvalidInput, "Must be two characters long"))
            }
        } else {
            Err(Error::new(NotFound, "Input cannot be empty"))
        }
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
    fn add_semitones(&mut self, semitones: i32) {
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
#[repr(u8)]
enum Interval {
    Min2 = 1,
    Maj2 = 2,
}

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
    fn base_intervals() -> impl Iterator {
        use Interval::*;
        [Maj2, Maj2, Min2, Maj2, Maj2, Maj2, Min2].iter()
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
    fn get_intervals(self) -> impl Iterator {
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
    //let mut rands = RandomBytes::new();
    //loop {
    //    println!("{}", rands.next().unwrap());
    //}

    let mut pitch = Pitch::new(STANDARD_PITCH);
    println!("{:?}", pitch);
    pitch.add_semitones(-OCTAVE_SEMITONES);
    println!("{:?}", pitch);
}
