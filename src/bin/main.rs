use music::Semitones;

fn main() {
    //let device = default_output_device().unwrap();
    //let sink = Sink::new(&device);
    //let source = SineWave::from(Pitch::default());
    //sink.append(source);
    println!("{:?}", Semitones::from(4) + Semitones::from(5));
}
