use super::*;
use pretty_assertions::assert_eq;

#[test]
fn cool_greeting() {
    assert_eq!(GREETING, "Cool Tunes (tm)");
}

#[test]
fn test_semitones_to_cents() {
    assert_eq!(Cents::from(Semitones(1)), Cents(100.0));
    assert_eq!(Cents::from(Semitones(12)), Cents(1200.0));
}

#[test]
fn test_add_interval() {
    use Interval::*;
    assert_eq!(Unison + Unison, Unison);
    assert_eq!(Unison + Maj3, Maj3);
    assert_eq!(Octave + Octave, Octave);
    assert_eq!(Tritone + Tritone, Octave);
    assert_eq!(Maj7 + Min3, Min2);
}
