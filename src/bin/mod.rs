use music::*;
use rodio::{Sink, source::SineWave, default_output_device};

fn main() {
    println!("{}", GREETING);
    let device = default_output_device().unwrap();
    let sink = Sink::new(&device);
    let source = SineWave::from(Pitch::default());
    sink.append(source);
    sink.sleep_until_end();
}
