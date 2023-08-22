use super::seed::*;
use crate::theory::{hertz::*, key::Key, piano_key::PianoKey, pitch::Pitch, scale::Scale};
use rodio::source::Source;
use std::{f32::consts::PI, fmt, str::FromStr, time::Duration};

pub struct MusicMaker {
    key: Key,
    seed: Box<dyn MusicSeed>,
    current_note: PianoKey,
    current_sample: usize,
    sample_rate: Hertz,
    volume: f32,
}

impl Default for MusicMaker {
    fn default() -> Self {
        Self {
            key: Key::default(),
            seed: Box::<RandomSeed>::default(),
            current_note: PianoKey::from_str("C4").unwrap(),
            current_sample: usize::default(),
            sample_rate: SAMPLE_RATE,
            volume: 2.0,
        }
    }
}

pub type Sample = f32;

impl MusicMaker {
    pub fn new(base_note: PianoKey, scale: Scale, octaves: u8) -> Self {
        Self::default().set_key(base_note, scale, octaves)
    }
    fn get_frequency(&mut self) -> Sample {
        let pitch = Pitch::from(self.current_note);
        pitch.into()
    }
    fn new_note(&mut self) {
        let new_note = self.seed.get_note(self.key);
        //print!("{} ", new_note); TODO doenst work b/c sleep until end, i think?
        self.current_note = new_note;
    }
    pub fn set_key(mut self, base_note: PianoKey, scale: Scale, octaves: u8) -> Self {
        self.key = Key::new(scale, base_note, octaves);
        self
    }
}

impl Iterator for MusicMaker {
    type Item = Sample; // Sampled amplitude
    fn next(&mut self) -> Option<Self::Item> {
        self.current_sample = self.current_sample.wrapping_add(1); // will cycle

        let value = self.volume * PI * self.get_frequency() * self.current_sample as Sample
            / f64::from(self.sample_rate) as Sample;
        // when to switch notes?
        if self.current_sample as f64 >= f64::from(self.sample_rate) {
            self.current_sample = 0;
            self.new_note();
        }
        Some(value.sin())
    }
}

impl Source for MusicMaker {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        f64::from(self.sample_rate) as u32
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl fmt::Display for MusicMaker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let key = self.key;
        write!(
            f,
            "Generating music from the {} {}\nOctaves: {} - {}\n{}",
            key.base_note.note,
            key.scale,
            key.base_note.octave,
            key.base_note.octave + key.octaves,
            key
        )
    }
}

impl From<Pitch> for Sample {
    fn from(p: Pitch) -> Self {
        f64::from(p) as f32
    }
}
