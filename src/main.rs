use core::time::Duration;
use rand::random;
use rodio::{default_output_device, source::SineWave, Sink};
use std::{
    f32,
    f64::consts::PI,
    fmt,
    ops::{AddAssign, Div},
};

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
const MIDDLE_C: Hertz = 261.626;
const SAMPLE_RATE: Hertz = 44_100.0;

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
enum NoteLetter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Default for NoteLetter {
    fn default() -> Self {
        NoteLetter::C
    }
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Default, Debug, Clone, Copy)]
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

#[derive(Default, Debug, Clone, Copy)]
struct StandardPitch {
    note: Note,
    octave: u8,
}

impl StandardPitch {
    fn new(s: &str) -> Self {
        Self::default()
    }
    //fn get_offset()
}

impl fmt::Display for StandardPitch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.note, self.octave)
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
        let mut ret = Pitch::default();
        // TODO
        ret
    }
}

impl From<Pitch> for StandardPitch {
    fn from(p: Pitch) -> Self {
        let mut ret = StandardPitch::default();
        //let (interval, octaves) = ret.get_offset()
        unimplemented!()
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

#[derive(Debug, Clone, Copy)]
enum ScaleLength {
    Pentatonic = 5,
    Heptatonic = 7,
    Chromatic = 12,
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
    fn base_intervals() -> &'static [Interval] {
        use Interval::*;
        &[Maj2, Maj2, Min2, Maj2, Maj2, Maj2, Min2]
    }
    //fn get_intervals(&self) -> impl Iterator {
    //    let intervals = Mode::base_intervals();
    //    Mode::base_intervals().skip(self as u8 as usize).cycle()
    //}
}

#[derive(Debug, Clone, Copy)]
enum Scale {
    //Chromatic,
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
            //Chromatic => StandardPitch::all_pitches(),
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
    let device = default_output_device().unwrap();
    let sink = Sink::new(&device);
    let source = SineWave::from(Pitch::default());
    sink.append(source);
}
