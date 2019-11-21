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
const STANDARD_PITCH: Hertz = 440;

const SCALE_SIZE: usize = 7;

#[derive(Debug, Clone, Copy)]
enum Interval {
    Half = 1,
    Whole = 2,
}

type ScaleIntervals = [Interval; SCALE_SIZE];

const MajorScale: ScaleIntervals = [
    Interval::Whole,
    Interval::Whole,
    Interval::Half,
    Interval::Whole,
    Interval::Whole,
    Interval::Whole,
    Interval::Half,
];

#[derive(Debug)]
enum Mode {
    Aeolian,
    Ionian,
}

impl Mode {
    fn get_interval(&self, n: u8) -> Interval {
        let idx = n as usize % SCALE_SIZE;
        MajorScale[idx]
    }
}

fn main() {
    let mut rands = RandomInput::new();
    loop {
        println!("{}", rands.next().unwrap());
    }
}
