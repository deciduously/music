use core::time::Duration;
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};
use rodio::source::{SineWave, Source};
use std::{
    f32::consts::PI,
    fmt, io,
    ops::{Add, AddAssign, Div, MulAssign, Sub},
    str::FromStr,
};

#[cfg(test)]
mod test;

pub const GREETING: &str = ".: Cool Tunes :.";
pub const STANDARD_PITCH: Hertz = Hertz(440.0);
pub const MIDDLE_C: Hertz = Hertz(261.626);
const SEMITONE_CENTS: Cents = Cents(100.0);
pub const C_ZERO: Hertz = Hertz(16.352);
pub const SAMPLE_RATE: Hertz = Hertz(48_000.0);

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Hertz(f64);

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

impl From<Pitch> for SineWave {
    fn from(p: Pitch) -> Self {
        SineWave::new(f64::from(p) as u32)
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

impl NoteLetter {
    fn inc(self) -> Self {
        use NoteLetter::*;
        match self {
            C => D,
            D => E,
            E => F,
            F => G,
            G => A,
            A => B,
            B => C,
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

impl FromStr for Accidental {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "b" | "♭" => Ok(Accidental::Flat),
            "#" => Ok(Accidental::Sharp),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} is not a valid accidental", s),
            )),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Note {
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

fn char_strs<'a>(s: &'a str) -> Vec<&'a str> {
    s.split("")
        .skip(1)
        .take_while(|c| *c != "")
        .collect::<Vec<&str>>()
}

impl FromStr for Note {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let char_strs = char_strs(s);
        let mut char_strs = char_strs.iter();
        // note will be first
        if let Some(letter) = char_strs.next() {
            let letter = NoteLetter::from_str(letter)?;
            if let Some(accidental) = char_strs.next() {
                // check if it's valid
                let accidental = Accidental::from_str(accidental)?;
                return Ok(Self {
                    letter,
                    accidental: Some(accidental),
                });
            } else {
                return Ok(Self {
                    letter,
                    accidental: None,
                });
            }
        }
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} is not a valid note", s),
        ))
    }
}

impl Note {
    fn interval_from_c(self) -> Interval {
        use Accidental::*;
        let ret = self.letter.interval_from_c();
        if let Some(acc) = self.accidental {
            match acc {
                Flat => return Interval::from(Semitones::from(i8::from(Semitones::from(ret)) - 1)),
                Sharp => return ret + Interval::Min2,
            }
        };
        ret
    }
    fn get_offset(self, other: Self) -> Interval {
        let self_interval_from_c = self.interval_from_c();
        let other_interval_from_c = other.interval_from_c();
        self_interval_from_c - other_interval_from_c
    }
    fn inc(&mut self) {
        use Accidental::*;
        use NoteLetter::*;
        if let Some(acc) = self.accidental {
            self.accidental = None;
            match acc {
                Sharp => {
                    self.letter = self.letter.inc();
                }
                Flat => {}
            }
        } else {
            // check for special cases
            if self.letter == B || self.letter == E {
                self.letter = self.letter.inc();
            } else {
                self.accidental = Some(Sharp);
            }
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct PianoKey {
    note: Note,
    octave: u8,
}

impl fmt::Display for PianoKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.note, self.octave)
    }
}

impl FromStr for PianoKey {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // It makes sense to get the letter to Intervals
        if let Some(octave) = char_strs(s).last() {
            if let Ok(octave) = octave.parse::<u8>() {
                let note = Note::from_str(&s[0..s.len() - 1])?;
                if octave <= 8 {
                    Ok(Self { note, octave })
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("{} is too high!", octave),
                    ))
                }
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("{} is too high for this keyboard", octave),
                ))
            }
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} is not a valid note", s),
            ))
        }
    }
}

impl PianoKey {
    pub fn new(s: &str) -> Result<Self, io::Error> {
        Self::from_str(s)
    }
    fn max_octave() -> u8 {
        8
    }
}

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

impl NoteLetter {
    fn interval_from_c(self) -> Interval {
        use Interval::Unison;
        Scale::default()
            .get_intervals()
            .iter()
            .take(self as usize)
            .fold(Unison, |acc, i| acc + *i)
    }
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

impl Add for Interval {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Interval::from(Semitones(
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
        Interval::from(Semitones(delta))
    }
}

impl AddAssign for Interval {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl From<Interval> for Note {
    // Take an interval from C
    fn from(i: Interval) -> Self {
        use Interval::*;
        let mut offset = Unison;
        // That's a series of Min2
        let scale = Scale::Chromatic.get_intervals();
        scale.iter().take(i as usize).for_each(|i| offset += *i);
        Note::default() + offset
    }
}

impl Add<Interval> for Note {
    type Output = Self;

    fn add(self, rhs: Interval) -> Self {
        let semitones = Semitones::from(rhs);
        let mut ret = self;
        for _ in 0..i8::from(semitones) {
            ret.inc();
        }
        ret
    }
}

impl AddAssign<Interval> for Note {
    fn add_assign(&mut self, rhs: Interval) {
        *self = *self + rhs;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Mode {
    Ionian = 0,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}

impl Mode {
    fn base_intervals() -> Vec<Interval> {
        use Interval::*;
        vec![Maj2, Maj2, Min2, Maj2, Maj2, Maj2, Min2]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScaleLength {
    Tetratonic = 4,
    Heptatonic = 7,
    Dodecatonic = 12,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Scale {
    Chromatic,
    Diatonic(Mode),
    Tetratonic,
}

impl Default for Scale {
    fn default() -> Self {
        Scale::Diatonic(Mode::Ionian)
    }
}

impl Scale {
    pub fn circle_of_fifths(mode: Mode) -> Vec<Key> {
        let mut ret = Vec::new();
        // Start with C
        let mut current_base = Note::default();
        // Increment by fifths and push to vector
        for _ in 0..ScaleLength::Dodecatonic as usize {
            ret.push(Key::new(
                Scale::Diatonic(mode),
                PianoKey::from_str(&format!("{}4", current_base)).unwrap(),
                1,
            ));
            current_base += Interval::Perfect5;
        }
        ret
    }
    fn get_intervals(self) -> Vec<Interval> {
        use Interval::*;
        use Scale::*;
        match self {
            Chromatic => [Min2]
                .iter()
                .cycle()
                .take(ScaleLength::Dodecatonic as usize)
                .copied()
                .collect::<Vec<Interval>>(),
            Diatonic(mode) => Mode::base_intervals()
                .iter()
                .cycle()
                .skip(mode as usize)
                .take(ScaleLength::Heptatonic as usize)
                .copied()
                .collect::<Vec<Interval>>(),
            Tetratonic => vec![Min2, Maj2, Maj3],
        }
    }
}

impl FromStr for Scale {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Mode::*;
        use Scale::*;
        match s.to_uppercase().as_str() {
            "IONIAN" | "MAJOR" => Ok(Diatonic(Ionian)),
            "DORIAN" => Ok(Diatonic(Dorian)),
            "PHRYGIAN" => Ok(Diatonic(Phrygian)),
            "LYDIAN" => Ok(Diatonic(Lydian)),
            "MIXOLYDIAN" => Ok(Diatonic(Mixolydian)),
            "AEOLIAN" | "MINOR" => Ok(Diatonic(Aeolian)),
            "LOCRIAN" => Ok(Diatonic(Locrian)),
            "CHROMATIC" => Ok(Chromatic),
            "TETRATONIC" => Ok(Tetratonic),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unknown scale")),
        }
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Scale::*;
        let s = match self {
            Chromatic | Tetratonic => format!("{:?} scale", self).to_lowercase(),
            Diatonic(mode) => {
                use Mode::*;
                match mode {
                    Aeolian => "minor scale".into(),
                    Ionian => "major scale".into(),
                    _ => format!("{:?} mode", mode),
                }
            }
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Key {
    base_note: PianoKey,
    octaves: u8,
    scale: Scale,
}

impl Key {
    pub fn new(scale: Scale, base_note: PianoKey, octaves: u8) -> Self {
        let octaves = if base_note.octave + octaves > 8 {
            PianoKey::max_octave() - base_note.octave
        } else {
            octaves
        };
        Self {
            base_note,
            octaves,
            scale,
        }
    }
    fn all_keys(self) -> Vec<PianoKey> {
        let notes = self.get_notes();
        let mut ret = Vec::new();
        for i in 0..self.octaves {
            notes.iter().for_each(|n| {
                ret.push(
                    PianoKey::from_str(&format!("{}{}", *n, i + self.base_note.octave))
                        .unwrap_or_else(|_| {
                            PianoKey::from_str(&format!("{}{}", *n, PianoKey::max_octave()))
                                .unwrap()
                        }),
                )
            });
        }
        ret
    }
    pub fn get_notes(self) -> Vec<Note> {
        let mut ret = vec![self.base_note.note];
        let mut offset = Interval::Unison;
        self.scale.get_intervals().iter().for_each(|i| {
            offset += *i;
            ret.push(self.base_note.note + offset)
        });
        ret
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let notes = self.get_notes();
        let mut ret = String::from("[ ");
        notes.iter().for_each(|n| ret.push_str(&format!("{} ", *n)));
        ret.push_str("]");
        write!(f, "{}", ret)
    }
}

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
        Cents(i8::from(s) as f64 * f64::from(SEMITONE_CENTS))
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

impl AddAssign<Cents> for Pitch {
    #[allow(clippy::suspicious_op_assign_impl)] // needed to stop clippy from yelling
    fn add_assign(&mut self, rhs: Cents) {
        self.0 *= 2.0f64.powf((rhs / Cents::from(Interval::Octave)).into())
    }
}

impl MulAssign<f64> for Hertz {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
    }
}

impl Hertz {
    fn abs(self) -> Self {
        Self(self.0.abs())
    }
}

impl PartialEq for Pitch {
    fn eq(&self, other: &Pitch) -> bool {
        let tolerance = Hertz(0.1);
        let difference = (self.0 - other.0).abs();
        difference < tolerance
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

//enum MusicSeed {
//    ByteStream(Box<dyn Iterator<Item = u8>>),
//    Linear,
//    Random(SmallRng),
//}

// Linear should implement Iterator, and get_note() will just call next()

trait MusicSeed: Send {
    fn get_note(&mut self, key: Key) -> PianoKey;
}

struct RandomSeed(SmallRng);

impl Default for RandomSeed {
    fn default() -> Self {
        Self(SmallRng::from_entropy())
    }
}

impl MusicSeed for RandomSeed {
    fn get_note(&mut self, key: Key) -> PianoKey {
        let keys = key.all_keys();
        *keys.choose(&mut self.0).unwrap()
    }
}

#[derive(Default)]
struct LinearSeed(u8);

impl Iterator for LinearSeed {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.0;
        self.0 += 1;
        Some(ret)
    }
}

impl MusicSeed for LinearSeed {
    fn get_note(&mut self, key: Key) -> PianoKey {
        unimplemented!()
    }
}

pub struct MusicMaker {
    key: Key,
    seed: Box<dyn MusicSeed>,
    current_note: PianoKey,
    current_sample: usize,
    sample_rate: Hertz,
    volume: f32,
}

impl Default for MusicMaker {
    fn default() -> Self {
        Self {
            key: Key::default(),
            seed: Box::new(RandomSeed::default()),
            current_note: PianoKey::from_str("C4").unwrap(),
            current_sample: usize::default(),
            sample_rate: SAMPLE_RATE,
            volume: 2.0,
        }
    }
}

pub type Sample = f32;

impl MusicMaker {
    pub fn new(base_note: PianoKey, scale: Scale, octaves: u8) -> Self {
        Self::default().set_key(base_note, scale, octaves)
    }
    fn get_frequency(&mut self) -> Sample {
        let pitch = Pitch::from(self.current_note);
        pitch.into()
    }
    fn new_note(&mut self) {
        let new_note = self.seed.get_note(self.key);
        //print!("{} ", new_note); TODO doenst work b/c sleep until end, i think?
        self.current_note = new_note;
    }
    pub fn set_key(mut self, base_note: PianoKey, scale: Scale, octaves: u8) -> Self {
        self.key = Key::new(scale, base_note, octaves);
        self
    }
}

impl Iterator for MusicMaker {
    type Item = Sample; // Sampled amplitude
    fn next(&mut self) -> Option<Self::Item> {
        self.current_sample = self.current_sample.wrapping_add(1); // will cycle

        let value = self.volume * PI * self.get_frequency() * self.current_sample as Sample
            / f64::from(self.sample_rate) as Sample;
        // when to switch notes?
        if self.current_sample as f64 >= f64::from(self.sample_rate) {
            self.current_sample = 0;
            self.new_note();
        }
        Some(value.sin())
    }
}

impl Source for MusicMaker {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        f64::from(self.sample_rate) as u32
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl fmt::Display for MusicMaker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let key = self.key;
        write!(
            f,
            "Generating music from the {} {}\nOctaves: {} - {}\n{}",
            key.base_note.note,
            key.scale,
            key.base_note.octave,
            key.base_note.octave + key.octaves,
            key
        )
    }
}

impl From<Pitch> for Sample {
    fn from(p: Pitch) -> Self {
        f64::from(p) as f32
    }
}
