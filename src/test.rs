use super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_cool_greeting() {
    assert_eq!(GREETING, "Cool Tunes (tm)");
}

#[test]
fn test_new_pitch() {
    assert_eq!(Pitch::default(), Pitch(Hertz(440.0)));
    assert_eq!(Pitch::new(MIDDLE_C), Pitch(Hertz(261.621)));
}

#[test]
fn test_semitones_to_cents() {
    assert_eq!(Cents::from(Semitones(1)), Cents(100.0));
    assert_eq!(Cents::from(Semitones(12)), Cents(1200.0));
}

#[test]
fn test_interval_to_cents() {
    use Interval::*;
    assert_eq!(Cents::from(Unison), Cents(0.0));
    assert_eq!(Cents::from(Min2), Cents(100.0));
    assert_eq!(Cents::from(Octave), Cents(1200.0));
}

#[test]
fn test_add_semitones() {
    assert_eq!(Semitones(2) + Semitones(4), Semitones(6));
}

#[test]
fn test_add_interval() {
    use Interval::*;
    assert_eq!(Unison + Unison, Unison);
    assert_eq!(Unison + Maj3, Maj3);
    assert_eq!(Maj2 + Min3, Perfect4);
    assert_eq!(Octave + Octave, Unison);
    assert_eq!(Tritone + Tritone, Unison);
    assert_eq!(Maj7 + Min3, Maj2);
}

#[test]
fn test_add_cents_to_pitch() {
    let mut pitch = Pitch::default();
    pitch += Cents(3.9302);
    assert_eq!(pitch, Pitch::new(Hertz(441.0)));
}

#[test]
fn test_add_semitones_to_pitch() {
    use Interval::Octave;
    let mut pitch = Pitch::default();
    pitch += Semitones::from(Octave);
    assert_eq!(pitch, Pitch::new(Hertz(880.0)))
}

#[test]
fn test_add_interval_to_pitch() {
    use Interval::Min2;
    let mut pitch = Pitch::default();
    pitch += Min2;
    assert_eq!(pitch, Pitch::new(Hertz(466.1)))
}

#[test]
fn test_new_piano_key() {
    use Accidental::*;
    use NoteLetter::*;
    assert_eq!(
        PianoKey::default(),
        PianoKey {
            note: Note {
                letter: C,
                accidental: None
            },
            octave: 0
        }
    );
    assert_eq!(
        PianoKey::new("A4").unwrap(),
        PianoKey {
            note: Note {
                letter: A,
                accidental: None
            },
            octave: 4
        }
    );
    assert_eq!(
        PianoKey::new("G♭2").unwrap(),
        PianoKey {
            note: Note {
                letter: G,
                accidental: Some(Flat)
            },
            octave: 2
        }
    );
    assert_eq!(
        PianoKey::new("F#8").unwrap(),
        PianoKey {
            note: Note {
                letter: F,
                accidental: Some(Sharp)
            },
            octave: 8
        }
    );
}

#[test]
#[should_panic]
fn test_reject_piano_key_too_high() {
    assert_eq!(PianoKey::new("A9").unwrap(), PianoKey::default());
}

#[test]
#[should_panic]
fn test_reject_piano_key_invalid_letter() {
    assert_eq!(PianoKey::new("Q7").unwrap(), PianoKey::default());
}

#[test]
fn test_piano_key_to_pitch() {
    assert_eq!(Pitch::from(PianoKey::new("A4").unwrap()), Pitch::default());
    assert_eq!(Pitch::from(PianoKey::default()), Pitch::new(C_ZERO));
}

#[test]
fn test_note_interval() {
    // TODO
}

#[test]
fn test_standard_pitch_to_pitch() {
    // TODO
}

// TODO demonstrate above, then talk about FromStr
