use music::*;
use rodio::{default_output_device, Sink};
use structopt::StructOpt;

/// music is a procedural single-tone melody generator
#[derive(StructOpt, Debug)]
#[structopt(name = "music")]
struct Opt {
    /// The base note to calculate the scale from
    #[structopt(short, long, default_value = "C")]
    note: Note,
    /// The series of intervals from the base note to use per octave
    #[structopt(short, long, default_value = "Ionian")]
    scale: Scale,
}

fn main() {
    let opt = Opt::from_args();
    println!("{}", GREETING);

    let device = default_output_device().unwrap();
    let sink = Sink::new(&device);
    let music = MusicMaker::new(opt.note, opt.scale);
    println!("{}", music);
    sink.append(music);
    sink.sleep_until_end();
}
