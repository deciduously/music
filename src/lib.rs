#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub mod output;
pub mod theory;

#[cfg(test)]
mod test;

/// Helper function to produce an f32 from an f64, isolating the possible truncation.
fn f64_to_f32(f: f64) -> f32 {
	#![allow(clippy::cast_possible_truncation)]
	f as f32
}
