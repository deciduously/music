use core::time::Duration;
use rand::random;
use rodio::{default_output_device, source::SineWave, Sink};
use std::{
    f32,
    f64::consts::PI,
    fmt, io,
    ops::{Add, AddAssign, Div},
    str::FromStr,
};

#[cfg(test)]
mod test;

pub const GREETING: &'static str = "Cool Tunes (tm)";

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

pub type Hertz = f64;
pub const STANDARD_PITCH: Hertz = 440.0;
pub const C_ZERO: Hertz = 16.352;
pub const MIDDLE_C: Hertz = 261.626;
pub const SAMPLE_RATE: Hertz = 44_100.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cents(f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Semitones(i8);

const SEMITONE_CENTS: Cents = Cents(100.0);

impl From<f64> for Cents {
    fn from(f: f64) -> Self {
        Cents(f)
    }
}

impl From<Cents> for f64 {
    fn from(cents: Cents) -> Self {
        cents.0
    }
}

impl From<i8> for Semitones {
    fn from(i: i8) -> Self {
        Semitones(i)
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

impl Add for Semitones {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Semitones(i8::from(self) + i8::from(other))
    }
}

impl Div for Cents {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Cents(f64::from(self) / f64::from(rhs))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum NoteLetter {
    C = 0,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl Default for NoteLetter {
    fn default() -> Self {
        NoteLetter::C
    }
}

impl FromStr for NoteLetter {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO this can probably get super concise and cool
        match s.to_uppercase().as_str() {
            "A" => Ok(NoteLetter::A),
            "B" => Ok(NoteLetter::B),
            "C" => Ok(NoteLetter::C),
            "D" => Ok(NoteLetter::D),
            "E" => Ok(NoteLetter::E),
            "F" => Ok(NoteLetter::F),
            "G" => Ok(NoteLetter::G),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} is not a valid note", s),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Accidental {
    Flat,
    Sharp,
}

impl fmt::Display for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Accidental::*;
        let acc_str = match self {
            Flat => "♭",
            Sharp => "#",
        };
        write!(f, "{}", acc_str)
    }
}

impl FromStr for NoteLetter {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use 
        match s {
            "b" | "♭" => Ok(Accidental::Flat),
            "#" => Ok(Accidental::Sharp),
            
        }
    }
}


#[derive(Default, Debug, Clone, Copy, PartialEq)]
struct Note {
    accidental: Option<Accidental>,
    letter: NoteLetter,
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let acc_str = if let Some(a) = self.accidental {
            format!("{}", a)
        } else {
            "".to_string()
        };
        write!(f, "{:?}{}", self.letter, acc_str)
    }
}

impl Note {
    fn get_interval(&self, other: Self) -> Interval {
        let int_self = self.letter as i8;
        let int_other = other.letter as i8;
        // TODO accidental?
        Interval::from(Semitones(
            (int_self - int_other).abs() % NoteLetter::B as i8,
        ))
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct StandardPitch {
    note: Note,
    octave: u8,
}

impl StandardPitch {
    fn new(s: &str) -> Self {
        Self::default()
    }
    //fn get_offset()
    fn all_pitches() -> &'static [Interval] {
        unimplemented!()
        // Need to do FromStr first
    }
}

impl fmt::Display for StandardPitch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.note, self.octave)
    }
}

impl FromStr for StandardPitch {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pitch {
    frequency: Hertz,
}

impl Pitch {
    pub fn new(frequency: Hertz) -> Self {
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

impl PartialEq for Pitch {
    fn eq(&self, other: &Pitch) -> bool {
        let tolerance = 0.1;
        let difference = (self.frequency - other.frequency).abs();
        difference < tolerance
    }
}

impl AddAssign<Cents> for Pitch {
    #[allow(clippy::suspicious_op_assign_impl)]
    fn add_assign(&mut self, rhs: Cents) {
        self.frequency *= 2.0f64.powf((rhs / Cents::from(Interval::Octave)).into())
    }
}

impl AddAssign<Semitones> for Pitch {
    fn add_assign(&mut self, rhs: Semitones) {
        *self += Cents::from(rhs)
    }
}

impl AddAssign<Interval> for Pitch {
    fn add_assign(&mut self, rhs: Interval) {
        *self += Cents::from(rhs)
    }
}

impl From<StandardPitch> for Pitch {
    fn from(sp: StandardPitch) -> Self {
        use Interval::*;
        let mut ret = Pitch::new(C_ZERO);
        // Add octaves
        for _ in 0..sp.octave {
            ret += Octave;
        }
        // TODO Add note offset
        ret
    }
}

impl From<Pitch> for f64 {
    fn from(pitch: Pitch) -> Self {
        pitch.frequency
    }
}

impl From<Pitch> for SineWave {
    fn from(p: Pitch) -> Self {
        SineWave::new(f64::from(p) as u32)
    }
}

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

impl From<Interval> for i8 {
    fn from(i: Interval) -> Self {
        Semitones::from(i).into()
    }
}

impl From<Semitones> for Interval {
    fn from(s: Semitones) -> Self {
        use Interval::*;
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
            12 | _ => Interval::from(Semitones(int_semitones % Octave as i8)),
        }
    }
}

impl From<Interval> for Semitones {
    fn from(i: Interval) -> Self {
        Semitones(i as i8)
    }
}

impl From<Interval> for Cents {
    fn from(i: Interval) -> Self {
        Semitones::from(i).into()
    }
}

impl Add for Interval {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Interval::from(Semitones(
            i8::from(self) + i8::from(rhs) % Interval::Octave as i8,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ScaleLength {
    Tetratonic = 4,
    Pentatonic = 5,
    Heptatonic = 7,
    Chromatic = 12,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Mode {
    Ionian = 0,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}

impl Mode {
    fn base_intervals() -> &'static [Interval] {
        use Interval::*;
        &[Maj2, Maj2, Min2, Maj2, Maj2, Maj2, Min2]
    }
    //fn get_intervals(&self) -> impl Iterator {
    //    let intervals = Mode::base_intervals();
    //    Mode::base_intervals().skip(self as u8 as usize).cycle()
    //}
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Scale {
    Chromatic,
    Diatonic(Mode),
}

impl Default for Scale {
    fn default() -> Self {
        Scale::Diatonic(Mode::Ionian)
    }
}

impl Scale {
    fn get_intervals(self) -> &'static [Interval] {
        // TODO this needs to be a method, come here next!
        // have THIS calculate an impl Iterator (or impl Scale??)
        use Scale::*;
        match self {
            Chromatic => StandardPitch::all_pitches(),
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
