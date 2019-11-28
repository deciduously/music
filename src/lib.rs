use core::time::Duration;
use lazy_static::lazy_static;
use rand::{random, rngs::SmallRng, seq::IteratorRandom, Rng, SeedableRng};
use rodio::source::{SineWave, Source};
use std::{
    f32::consts::PI,
    fmt, io,
    ops::{Add, AddAssign, Div, MulAssign, Sub},
    str::FromStr,
};

#[cfg(test)]
mod test;

lazy_static! {
// TODO OPT
}

pub const GREETING: &str = "Cool Tunes (tm)";

#[derive(Default)]
struct RandomMelody;

impl RandomMelody {
    fn new() -> Self {
        Self::default()
    }
}

impl Iterator for RandomMelody {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        Some(random::<Self::Item>())
    }
}

//impl Source for RandomMelody {}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Hertz(f64);

pub const STANDARD_PITCH: Hertz = Hertz(440.0);
pub const C_ZERO: Hertz = Hertz(16.352);
pub const MIDDLE_C: Hertz = Hertz(261.626);
pub const SAMPLE_RATE: Hertz = Hertz(44_100.0);

impl Hertz {
    fn abs(self) -> Self {
        Self(self.0.abs())
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

impl Sub for Hertz {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl MulAssign<f64> for Hertz {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Cents(f64);

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Semitones(i8);

const SEMITONE_CENTS: Cents = Cents(100.0);

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

impl From<Semitones> for Cents {
    fn from(s: Semitones) -> Self {
        Cents(i8::from(s) as f64 * f64::from(SEMITONE_CENTS))
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

impl NoteLetter {
    fn all() -> &'static [Self] {
        use NoteLetter::*;
        &[C, D, E, F, G, A, B]
    }
    fn interval_from_c(self) -> Interval {
        use Interval::Unison;
        Scale::default()
            .get_intervals()
            .iter()
            .take(self as usize)
            .fold(Unison, |acc, i| acc + *i)
    }
    fn inc(self) -> Self {
        use NoteLetter::*;
        // TODO MAKE THIS COOLER
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
struct Note {
    accidental: Option<Accidental>,
    letter: NoteLetter,
}

impl Note {
    fn interval_from_c(self) -> Interval {
        use Accidental::*;
        let ret = self.letter.interval_from_c();
        if let Some(acc) = self.accidental {
            match acc {
                // TODO refactor
                Flat => return Interval::from(Semitones::from(i8::from(Semitones::from(ret)) - 1)),
                Sharp => return ret + Interval::Min2,
            }
        };
        ret
    }
    fn get_offset_from_interval(self, other: Interval) -> Interval {
        let self_interval_from_c = self.interval_from_c();
        self_interval_from_c - other
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

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct PianoKey {
    note: Note,
    octave: u8,
}

impl PianoKey {
    fn new(s: &str) -> Result<Self, io::Error> {
        Self::from_str(s)
    }
    //fn get_offset()
    fn all_pitches() -> Vec<PianoKey> {
        NoteLetter::all()
            .iter()
            .zip([0..8].iter())
            .map(|(letter, octave)| {
                PianoKey::from_str(&format!("{:?}{:?}", letter, octave)).unwrap()
            })
            .collect::<Vec<PianoKey>>()
    }
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

impl Add<Interval> for PianoKey {
    type Output = Self;

    fn add(self, rhs: Interval) -> Self {
        unimplemented!()
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

impl PartialEq for Pitch {
    fn eq(&self, other: &Pitch) -> bool {
        let tolerance = Hertz(0.1);
        let difference = (self.0 - other.0).abs();
        difference < tolerance
    }
}

impl AddAssign<Cents> for Pitch {
    #[allow(clippy::suspicious_op_assign_impl)]
    fn add_assign(&mut self, rhs: Cents) {
        self.0 *= 2.0f64.powf((rhs / Cents::from(Interval::Octave)).into())
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

impl From<Pitch> for Sample {
    fn from(p: Pitch) -> Self {
        f64::from(p) as f32
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScaleLength {
    Tetratonic = 4,
    Pentatonic = 5,
    Heptatonic = 7,
    Dodecatonic = 12,
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
    //fn get_intervals(&self) -> impl Iterator {
    //    let intervals = Mode::base_intervals();
    //    Mode::base_intervals().skip(self as u8 as usize).cycle()
    //}
}

// TODO - REMOVE - FOR DEMO

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct DiatonicScale {
    offset: Semitones,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Scale {
    Chromatic,
    Diatonic(Mode),
}

impl Default for Scale {
    fn default() -> Self {
        Scale::Diatonic(Mode::Ionian)
    }
}

impl Scale {
    fn circle_of_fifths() -> Vec<Vec<Interval>> {
        unimplemented!()
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
            Diatonic(_) => Mode::base_intervals(),
        }
    }
    fn get_notes(self, base_note: Note) -> Vec<Note> {
        let mut ret = vec![base_note];
        let mut offset = Interval::Unison;
        self.get_intervals().iter().for_each(|i| {
            offset += *i;
            ret.push(base_note + offset)
        });
        ret
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

impl fmt::Display for Scale {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Key {
    base_note: Note,
    scale: Scale,
}

impl Key {
    fn all_keys(self) -> Vec<PianoKey> {
        let notes = self.scale.get_notes(self.base_note);
        let mut ret = Vec::new();
        for i in 3..6 {
            notes
                .iter()
                .for_each(|n| ret.push(PianoKey::from_str(&format!("{}{}", *n, i)).unwrap()));
        }
        ret
    }
}

pub struct MusicMaker {
    key: Key,
    seed: SmallRng,
    current_note: PianoKey,
    current_sample: usize,
    sample_rate: Hertz,
    volume: f32,
}

impl Default for MusicMaker {
    fn default() -> Self {
        Self {
            key: Key::default(),
            seed: SmallRng::from_entropy(),
            current_note: PianoKey::from_str("C4").unwrap(),
            current_sample: usize::default(),
            sample_rate: SAMPLE_RATE,
            volume: 2.0,
        }
    }
}

type Sample = f32;

impl MusicMaker {
    pub fn new() -> Self {
        Self::default()
    }
    fn get_frequency(&mut self) -> Sample {
        let pitch = Pitch::from(self.current_note);
        println!("{:?}", pitch);
        pitch.into()
    }
    fn new_note(&mut self) {
        let keys = self.key.all_keys();
        self.current_note = *keys.iter().choose(&mut self.seed).unwrap();
    }
}

impl Iterator for MusicMaker {
    type Item = Sample; // Sampled amplitude
    fn next(&mut self) -> Option<Self::Item> {
        self.current_sample = self.current_sample.wrapping_add(1); // will cycle

        let value = self.volume
            * PI
            * Sample::from(Pitch::from(self.current_note))
            * self.current_sample as Sample
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
