#![warn(clippy::pedantic)]

use clap::Parser;
use music::{
	output::MusicMaker,
	theory::{piano_key::PianoKey, pitch::Pitch, scale::Scale},
};
use rodio::{source::SineWave, OutputStream, Sink};

/// `music` is a procedural single-tone melody generator.
#[derive(clap::Parser, Debug)]
#[command(
    about = env!("CARGO_PKG_DESCRIPTION"),
	long_version = env!("CARGO_PKG_VERSION"),
	name = env!("CARGO_CRATE_NAME"),
	verbatim_doc_comment,
	version = env!("CARGO_PKG_VERSION"),
)]
struct Args {
	/// Single-pitch mode
	#[arg(short, long)]
	pitch_mode: bool,
	/// The base note to calculate the scale from
	#[arg(short, long, default_value = "C4")]
	base_note: PianoKey,
	/// The series of intervals from the base note to use per octave
	#[arg(short, long, default_value = "Ionian")]
	scale: Scale,
	/// Number of octaves over which to range, anything over 8 gets parsed as 8
	#[arg(short, long, default_value = "1")]
	octaves: u8,
}

/// Displayed in the CLI each run
const GREETING: &str = ".: Cool Tunes :.";

fn main() {
	// Read arguments, greet user
	let opt = Args::parse();
	println!("{GREETING}");

	// Set up audio playback
	let (_stream, handle) = OutputStream::try_default().unwrap();
	let sink = Sink::try_new(&handle).expect("Could not create sink");

	// Define music source from Opt
	if opt.pitch_mode {
		let wave = SineWave::from(Pitch::from(opt.base_note));
		println!("Playing single tone {}", opt.base_note);
		// Play sine wave
		sink.append(wave);
	} else {
		// Init procedural generator
		let music = MusicMaker::new(opt.base_note, opt.scale, opt.octaves);
		println!("{music}");
		// Play random melody
		sink.append(music);
	};
	// Sleep thread to allow music to play infinitely
	sink.sleep_until_end();
}
