// Get random number stream
// Map random data to u8 stream

// semitones : 0,2,4,5,7,9,11,12

//  the frequency in Hertz of a musical note with equal temperament: 440 * 2^(semitone distance / 12).
// 440 being A4

// inside a for loop (i = 0; i < 1; i += 0.0001)
// printf("%08X\n", 100*sin(1382*exp((a[$1 % 8]/12)*log(2))*i))

// this prints formatted 4-byte hex representing amplitute of the sound wave
// all is multiplied by 100 (scalar for volume control) - TODO structopt
// 1382 is ABOUT 440 * Pi - use RUST for this - constexpr??
// The bash verison uses 2^x = e^(x*ln(2)), we can just use 2^x
// 100 * sin((440*Pi) * (pick a random semitone / 12) * i)

// THEN instead of xxd, convert back into binary
// Use aplay to play actual sound - I bet there's a Pure Rust way to do this

// Minor scale

// THEN - author your own!

use lazy_static::lazy_static;
use rand::random;

#[derive(Default)]
struct RandomInput;

impl RandomInput {
    fn new() -> Self {
        Self::default()
    }
}

impl Iterator for RandomInput {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        Some(random::<Self::Item>())
    }
}

type Hertz = u32;

type Cents = f64;
const SEMITONE_CENTS: Cents = 100.0;
const OCTAVE_SEMITONES: u32 = 12;
const OCTAVE_CENTS: Cents = SEMITONE_CENTS * OCTAVE_SEMITONES as f64; // 1200.0

#[derive(Debug)]
struct Pitch {
    hertz: Hertz,
}

impl Pitch {
    fn new(hertz: Hertz) -> Self {
        Self { hertz }
    }
    fn add_cents(&mut self, cents: Cents) {
        self.hertz = (self.hertz as f64 * 2.0f64.powf(cents / OCTAVE_CENTS)) as u32;
    }
}

impl Default for Pitch {
    fn default() -> Self {
        Self { hertz: 440 }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Interval {
    Half = 1,
    Whole = 2,
}

type ScaleIntervals = Vec<Interval>;

lazy_static! {
    pub static ref MAJOR_SCALE: ScaleIntervals = vec![
        Interval::Whole,
        Interval::Whole,
        Interval::Half,
        Interval::Whole,
        Interval::Whole,
        Interval::Whole,
        Interval::Half,
    ];
}

#[derive(Debug)]
enum Mode {
    Aeolian,
    Ionian,
}

impl Mode {
    fn get_interval(&self, n: u8) -> Interval {
        use Mode::*;
        let start_note = match self {
            Aeolian => 5,
            Ionian => 0,
        };
        let idx = n as usize + start_note % MAJOR_SCALE.len();
        MAJOR_SCALE[idx]
    }
}

fn main() {
    //let mut rands = RandomInput::new();
    //loop {
    //    println!("{}", rands.next().unwrap());
    //}

    let mut pitch = Pitch::default();
    println!("{:?}", pitch);
    pitch.add_cents(2.0);
    println!("{:?}", pitch);
    pitch.add_cents(2.0);
    println!("{:?}", pitch);
}
