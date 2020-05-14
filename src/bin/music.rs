use music::{
    output::MusicMaker,
    theory::{piano_key::PianoKey, pitch::Pitch, scale::Scale},
};
use rodio::{default_output_device, source::SineWave, Sink};
use structopt::StructOpt;

/// music is a procedural single-tone melody generator
#[derive(StructOpt, Debug)]
#[structopt(name = "music")]
struct Opt {
    /// Single-pitch mode
    #[structopt(short, long)]
    pitch_mode: bool,
    /// The base note to calculate the scale from
    #[structopt(short, long, default_value = "C4")]
    base_note: PianoKey,
    /// The series of intervals from the base note to use per octave
    #[structopt(short, long, default_value = "Ionian")]
    scale: Scale,
    /// Number of octaves over which to range, anything over 8 gets parsed as 8
    #[structopt(short, long, default_value = "1")]
    octaves: u8,
}

/// Displayed in the CLI each run
const GREETING: &str = ".: Cool Tunes :.";

fn main() {
    // Read arguments, greet user
    let opt = Opt::from_args();
    println!("{}", GREETING);

    // Set up audio playback
    let device = default_output_device().unwrap();
    let sink = Sink::new(&device);

    // Define music source from Opt
    if opt.pitch_mode {
        let wave = SineWave::from(Pitch::from(opt.base_note));
        println!("Playing single tone {}", opt.base_note);
        // Play sine wave
        sink.append(wave);
    } else {
        // Init procedural generator
        let music = MusicMaker::new(opt.base_note, opt.scale, opt.octaves);
        println!("{}", music);
        // Play random melody
        sink.append(music);
    };
    // Sleep thread to allow music to play infinitely
    sink.sleep_until_end();
}
