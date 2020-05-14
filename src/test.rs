use super::theory::{
    cent::Cents, hertz::*, interval::*, key::Key, note::*, piano_key::PianoKey, pitch::Pitch,
    scale::*, semitone::Semitones,
};
use pretty_assertions::assert_eq;
use std::str::FromStr;

#[test]
fn test_subtract_hertz() {
    assert_eq!(Hertz::from(440.0) - Hertz::from(1.0), Hertz::from(439.0))
}

#[test]
fn test_new_pitch() {
    assert_eq!(Pitch::default(), Pitch::new(Hertz::from(440.0)));
    assert_eq!(Pitch::new(MIDDLE_C), Pitch::new(Hertz::from(261.626)));
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
        PianoKey::new("Gb2").unwrap(),
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
fn test_sub_interval() {
    use Interval::*;
    assert_eq!(Unison - Unison, Unison);
    assert_eq!(Unison - Maj3, Min6);
    assert_eq!(Maj2 - Min3, Maj7);
    assert_eq!(Octave - Octave, Unison);
    assert_eq!(Tritone - Tritone, Unison);
    assert_eq!(Maj7 - Min3, Min6);
}

#[test]
fn test_note_letter_to_interval() {
    use Interval::*;
    use NoteLetter::*;
    assert_eq!(C.interval_from_c(), Unison);
    assert_eq!(D.interval_from_c(), Maj2);
    assert_eq!(E.interval_from_c(), Maj3);
    assert_eq!(F.interval_from_c(), Perfect4);
    assert_eq!(G.interval_from_c(), Perfect5);
    assert_eq!(A.interval_from_c(), Maj6);
    assert_eq!(B.interval_from_c(), Maj7);
}

#[test]
fn test_get_note_interval_from_c() {
    use Interval::*;
    assert_eq!(Note::from_str("A").unwrap().interval_from_c(), Maj6);
    assert_eq!(Note::from_str("A#").unwrap().interval_from_c(), Min7);
    assert_eq!(Note::from_str("Bb").unwrap().interval_from_c(), Min7);
    assert_eq!(Note::from_str("B").unwrap().interval_from_c(), Maj7);
    assert_eq!(Note::from_str("C").unwrap().interval_from_c(), Unison);
    assert_eq!(Note::from_str("C#").unwrap().interval_from_c(), Min2);
    assert_eq!(Note::from_str("D").unwrap().interval_from_c(), Maj2);
    assert_eq!(Note::from_str("D#").unwrap().interval_from_c(), Min3);
    assert_eq!(Note::from_str("E").unwrap().interval_from_c(), Maj3);
    assert_eq!(Note::from_str("F").unwrap().interval_from_c(), Perfect4);
    assert_eq!(Note::from_str("F#").unwrap().interval_from_c(), Tritone);
    assert_eq!(Note::from_str("G").unwrap().interval_from_c(), Perfect5);
    assert_eq!(Note::from_str("G#").unwrap().interval_from_c(), Min6);
}

#[test]
fn test_get_note_offset() {
    use Interval::*;
    let a = Note::from_str("A").unwrap();
    assert_eq!(Note::from_str("A").unwrap().get_offset(a), Unison);
    assert_eq!(Note::from_str("A#").unwrap().get_offset(a), Min2);
    assert_eq!(Note::from_str("B").unwrap().get_offset(a), Maj2);
    assert_eq!(Note::from_str("C").unwrap().get_offset(a), Min3);
    assert_eq!(Note::from_str("C#").unwrap().get_offset(a), Maj3);
    assert_eq!(Note::from_str("D").unwrap().get_offset(a), Perfect4);
    assert_eq!(Note::from_str("D#").unwrap().get_offset(a), Tritone);
    assert_eq!(Note::from_str("E").unwrap().get_offset(a), Perfect5);
    assert_eq!(Note::from_str("F").unwrap().get_offset(a), Min6);
    assert_eq!(Note::from_str("F#").unwrap().get_offset(a), Maj6);
    assert_eq!(Note::from_str("G").unwrap().get_offset(a), Min7);
    assert_eq!(Note::from_str("G#").unwrap().get_offset(a), Maj7);
}

#[test]
fn test_add_interval_to_note() {
    use Interval::*;
    let a = Note::from_str("A").unwrap();
    assert_eq!(a + Unison, a);
    assert_eq!(a + Min2, Note::from_str("A#").unwrap());
    assert_eq!(a + Maj2, Note::from_str("B").unwrap());
    assert_eq!(a + Min3, Note::from_str("C").unwrap());
    assert_eq!(a + Maj3, Note::from_str("C#").unwrap());
    assert_eq!(a + Perfect4, Note::from_str("D").unwrap());
    assert_eq!(a + Tritone, Note::from_str("D#").unwrap());
    assert_eq!(a + Perfect5, Note::from_str("E").unwrap());
    assert_eq!(a + Min6, Note::from_str("F").unwrap());
    assert_eq!(a + Maj6, Note::from_str("F#").unwrap());
    assert_eq!(a + Min7, Note::from_str("G").unwrap());
    assert_eq!(a + Maj7, Note::from_str("G#").unwrap());
}

#[test]
fn test_c_major() {
    assert_eq!(
        &Key::new(Scale::default(), PianoKey::default(), 1).to_string(),
        "[ C D E F G A B C ]"
    )
}

#[test]
fn test_a_major() {
    assert_eq!(
        &Key::new(Scale::default(), PianoKey::from_str("A4").unwrap(), 1).to_string(),
        "[ A B C# D E F# G# A ]"
    )
}

#[test]
fn test_g_major() {
    assert_eq!(
        &Key::new(Scale::default(), PianoKey::from_str("G4").unwrap(), 1).to_string(),
        "[ G A B C D E F# G ]"
    )
}

#[test]
fn test_a_minor() {
    use Mode::*;
    use Scale::*;
    assert_eq!(
        &Key::new(Diatonic(Aeolian), PianoKey::from_str("A4").unwrap(), 1).to_string(),
        "[ A B C D E F G A ]"
    )
}

#[test]
fn test_semitones_to_cents() {
    assert_eq!(Cents::from(Semitones::from(1)), Cents::from(100.0));
    assert_eq!(Cents::from(Semitones::from(12)), Cents::from(1200.0));
}

#[test]
fn test_interval_to_cents() {
    use Interval::*;
    assert_eq!(Cents::from(Unison), Cents::from(0.0));
    assert_eq!(Cents::from(Min2), Cents::from(100.0));
    assert_eq!(Cents::from(Octave), Cents::from(1200.0));
}

#[test]
fn test_add_cents_to_pitch() {
    let mut pitch = Pitch::default();
    pitch += Cents::from(3.9302);
    assert_eq!(pitch, Pitch::new(Hertz::from(441.0)));
}

#[test]
fn test_add_semitones_to_pitch() {
    use Interval::Octave;
    let mut pitch = Pitch::default();
    pitch += Semitones::from(Octave);
    assert_eq!(pitch, Pitch::new(Hertz::from(880.0)))
}

#[test]
fn test_add_interval_to_pitch() {
    use Interval::Min2;
    let mut pitch = Pitch::default();
    pitch += Min2;
    assert_eq!(pitch, Pitch::new(Hertz::from(466.1)))
}

#[test]
fn test_piano_key_to_pitch() {
    assert_eq!(Pitch::from(PianoKey::new("A4").unwrap()), Pitch::default());
    assert_eq!(Pitch::from(PianoKey::default()), Pitch::new(C_ZERO));
}
