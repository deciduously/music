//! Theory is the toolkit for working with notes in terms of piano keys and intervals

pub mod cent;
pub mod hertz;
pub mod interval;
pub mod key;
pub mod note;
pub mod piano_key;
pub mod pitch;
pub mod scale;
pub mod semitone;

/// Helper function to split a string into a vector of strings, one per character
/// Example: "hello" => vec!["h", "e", "l", "l", "o"]
fn char_strs(s: &str) -> impl Iterator<Item = &str> {
	s.split("").skip(1).take_while(|c| !c.is_empty())
}

#[cfg(test)]
mod test {
	use super::*;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_char_strs() {
		assert_eq!(
			char_strs("hello").collect::<Vec<_>>(),
			vec!["h", "e", "l", "l", "o"]
		);
	}
}
