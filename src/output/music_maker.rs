use super::seed::{Random, Seed};
use crate::{
	f64_to_f32,
	theory::{hertz::SAMPLE_RATE, key::Key, piano_key::PianoKey, pitch::Pitch, scale::Scale},
};
use rodio::source::Source;
use std::{f64::consts::PI, fmt, str::FromStr, time::Duration};

pub struct MusicMaker {
	key: Key,
	seed: Box<dyn Seed>,
	current_note: PianoKey,
	current_sample: u32,
	sample_rate: u32,
	volume: f64,
}

impl Default for MusicMaker {
	fn default() -> Self {
		Self {
			key: Key::default(),
			seed: Box::<Random>::default(),
			current_note: PianoKey::from_str("C4").unwrap(),
			current_sample: u32::default(),
			sample_rate: u32::try_from(SAMPLE_RATE).unwrap(),
			volume: 2.0,
		}
	}
}

pub type Sample = f32;

impl MusicMaker {
	#[must_use]
	pub fn new(base_note: PianoKey, scale: Scale, octaves: u8) -> Self {
		Self::default().set_key(base_note, scale, octaves)
	}
	fn get_frequency(&mut self) -> Sample {
		let pitch = Pitch::from(self.current_note);
		f64_to_f32(pitch.into())
	}
	fn new_note(&mut self) {
		let new_note = self.seed.get_note(self.key);
		//print!("{} ", new_note); TODO doesn't work b/c sleep until end, i think?
		self.current_note = new_note;
	}
	#[must_use]
	pub fn set_key(mut self, base_note: PianoKey, scale: Scale, octaves: u8) -> Self {
		self.key = Key::new(scale, base_note, octaves);
		self
	}
}

impl Iterator for MusicMaker {
	type Item = Sample; // Sampled amplitude
	fn next(&mut self) -> Option<Self::Item> {
		self.current_sample = self.current_sample.wrapping_add(1); // will cycle

		let value =
			self.volume * PI * f64::from(self.get_frequency()) * f64::from(self.current_sample)
				/ f64::from(self.sample_rate);
		// when to switch notes?
		if f64::from(self.current_sample) >= f64::from(self.sample_rate) {
			self.current_sample = 0;
			self.new_note();
		}
		let result = f64_to_f32(value.sin());
		Some(result)
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
		self.sample_rate
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
