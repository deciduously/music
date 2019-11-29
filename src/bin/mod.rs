use lazy_static::lazy_static;
use music::*;
use rodio::{default_output_device, Sink};
use std::str::FromStr;

lazy_static! {
// TODO OPT
}

fn main() {
    println!("{}", GREETING);
    let circle = Scale::circle_of_fifths();

    for scale in circle {
        println!("{}", scale);
    }
    //let device = default_output_device().unwrap();
    //let sink = Sink::new(&device);
    //let music = MusicMaker::new();
    //sink.append(music);
    //sink.sleep_until_end();
}
