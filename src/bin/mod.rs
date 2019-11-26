use music::*;
use rodio::{default_output_device, source::SineWave, Sink};

fn main() {
    println!("{}", GREETING);
    let device = default_output_device().unwrap();
    let sink = Sink::new(&device);
    let source = SineWave::from(Pitch::default());
    sink.append(source);
    sink.sleep_until_end();
}
