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
    assert_eq!(Unison + Maj7, Maj7);
    assert_eq!(Octave + Octave, Octave);
    assert_eq!(Maj7 + Tritone, Perfect4);
}

#[test]
fn test_standard_pitch_from_str() {
    use std::str::FromStr;
    assert_eq!(
        StandardPitch::from_str("A4"),
        StandardPitch {
            note: Note {
                accidental: None,
                letter: NoteLetter::A,
            },
            octave: 4
        }
    )
}
