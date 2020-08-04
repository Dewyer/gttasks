//uncomment if you want to compile the benchmarks :S#![feature(test)]
//this as well : extern crate test;

fn main() {
	// The key that should be used run our one-time test
	let key = "SIDIOUS";
	// The text we want to "encrypt"
	let text = "EXECUTEORDERSIXTYSIX".to_string();

	let cypher = key_sub_cypher::KeySubCypher::new(key).unwrap();
	// Intentional unwrap, program should panic on invalid inputs
	let c1 = cypher.encrypt(&text).unwrap();
	let text_again = cypher.decrypt(&c1).unwrap();

	println!("Key : {}, Text: {}", key, text);
	println!("Cypher-text: {}, Text Again: {}", c1, text_again);
}

/// Hahahha, rextester does not compile the tests so this whole code is copiable to the site.
/// Otherwise the compilation would fail on missing crates
#[cfg(test)]
mod tests {
	use super::*;
	use rsgen::{gen_random_string, OutputCharsType};
	use rand::distributions::{Distribution, Uniform};
	use crate::key_sub_cypher;
	use rand::prelude::ThreadRng;
	use test::Bencher;

	fn run_fuzz_iteration(rng:&mut ThreadRng, sizer:&Uniform<usize>,ot:OutputCharsType)
	{
		let l1 = sizer.sample(rng);
		let l2 = sizer.sample(rng);
		let key = gen_random_string(l1, ot);
		let text = gen_random_string(l2, ot);

		let cip = key_sub_cypher::KeySubCypher::new(&key).unwrap();
		let cip_text = cip.encrypt(&text).unwrap();
		let t_again = cip.decrypt(&cip_text).unwrap();

		assert_eq!(text, t_again);
	}


	/// Tries a lot of random strings and checks if it can decypher it to the same, and does not panic
	#[test]
	fn fuzz() {
		let output_chars_type = OutputCharsType::LatinAlphabet {
			use_upper_case: true,
			use_lower_case: false,
		};
		let mut rng = rand::thread_rng();
		let sizer = Uniform::from(1..100);

		for ii in 0..1000
		{
			run_fuzz_iteration(&mut rng, &sizer,output_chars_type);
		}
	}

	/// Key length zero is invalid
	#[test]
	fn invalid_key()
	{
		let cip = key_sub_cypher::KeySubCypher::new("");
		assert!(cip.is_err());
	}

	/// Encryption should not work on invalid characters
	#[test]
	fn invalid_cypher_text()
	{
		let cip = key_sub_cypher::KeySubCypher::new("ABC").unwrap();
		assert!(cip.encrypt(&"ééééé1234".to_string()).is_err())
	}

	#[bench]
	pub fn bench(b: &mut Bencher)
	{
		let output_chars_type = OutputCharsType::LatinAlphabet {
			use_upper_case: true,
			use_lower_case: false,
		};
		let mut rng = rand::thread_rng();
		let sizer = Uniform::from(1..100);
		b.iter(|| run_fuzz_iteration(&mut rng, &sizer, output_chars_type))
	}

}

/// Key(ed) substitution cypher implementation
pub mod key_sub_cypher
{
	use std::fmt;
	use std::error::Error;

	/// Error type or the cypher
	#[derive(Debug, Eq, PartialEq)]
	pub enum KeySubCypherError
	{
		/// Given if user provides a string that is invalid as a cyhper key, as a key
		InvalidKey,
		InvalidTextToCypher,
	}

	impl Error for KeySubCypherError {}

	impl fmt::Display for KeySubCypherError {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			write!(f, "{:?}", self)
		}
	}

	static LENGTH_OF_ABC: i32 = 26;

	/// Static array that holds the uppercase english alphabet, duh
	static ALPHABET: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', ];

	/// Structure that holds the key
	/// Implements methods to encrypt and decrypt cyphertext in string form
	pub struct KeySubCypher
	{
		key: String
	}

	#[derive(Debug, Eq, PartialEq, Clone, Copy)]
	pub enum AdditionMode
	{
		Add,
		Subtract,
	}

	impl KeySubCypher
	{
		/// Creates a new Key(ed) substituion cypher with the given string as the key
		pub fn new(key: &str) -> Result<Self, KeySubCypherError>
		{
			if key.len() != 0
			{
				Ok(Self
				{
					key: key.to_string()
				})
			} else {
				Err(KeySubCypherError::InvalidKey)
			}
		}

		/// Returns the index, if any of the given character in the uppercase alphabet, returns an error if the character is not present in the alphabet
		fn get_character_index_in_alphabet(character: &char) -> Result<usize, KeySubCypherError>
		{
			ALPHABET.iter().position(|cc| cc == character).ok_or_else(|| KeySubCypherError::InvalidTextToCypher)
		}

		/// Adds or subtracts two characters and returns their sum, this implements the necesary "roll-over" as well
		fn add_or_subtract_two_characters(char1: &char, char2: &char, mode: AdditionMode) -> Result<char, KeySubCypherError>
		{
			let index1 = Self::get_character_index_in_alphabet(char1)?;
			let index2 = Self::get_character_index_in_alphabet(char2)?;
			let mult: i32 = if mode == AdditionMode::Add { 1 } else { -1 };
			let mut new_index = index1 as i32 + mult * index2 as i32;
			if new_index < 0
			{
				new_index = LENGTH_OF_ABC + new_index;
			}
			Ok(ALPHABET[(new_index % LENGTH_OF_ABC) as usize])
		}

		/// Does this : https://github.com/rust-lang/rust/issues/34354, for some reason Index<usize> for String is still not implemented
		fn get_character_at_index_in_string(ss: &String, index: usize) -> Option<char>
		{
			ss[index..].chars().next()
		}

		/// Adds or subtracts two strings, the second one is repeated if the first one is longer.
		fn add_or_subtract_strings(text1: &String, text2: &String, mode: AdditionMode) -> Result<String, KeySubCypherError>
		{
			let mut result_char_vec = Vec::new();
			for character_index in 0..text1.len()
			{
				// Neither of these will panic beacause the first one is at max the length of text1 and the second is modulo-ed down
				let char1 = Self::get_character_at_index_in_string(text1, character_index).unwrap();
				let char2 = Self::get_character_at_index_in_string(text2, character_index % text2.len()).unwrap();
				let new_character = Self::add_or_subtract_two_characters(&char1, &char2, mode)?;
				result_char_vec.push(new_character);
			}

			Ok(result_char_vec.iter().collect::<String>())
		}

		/// Encrypts the given text, with the key owned by this struct
		pub fn encrypt(&self, text: &String) -> Result<String, KeySubCypherError>
		{
			Self::add_or_subtract_strings(text, &self.key, AdditionMode::Add)
		}

		/// Decrypts the given text, with the key owned by this struct
		pub fn decrypt(&self, text: &String) -> Result<String, KeySubCypherError>
		{
			Self::add_or_subtract_strings(text, &self.key, AdditionMode::Subtract)
		}
	}
}