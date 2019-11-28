use music::*;
use rodio::{default_output_device, Sink};

fn main() {
    println!("{}", GREETING);
    let device = default_output_device().unwrap();
    let sink = Sink::new(&device);
    let music = MusicMaker::new();
    sink.append(music);
    sink.sleep_until_end();
}
