use std::collections::HashSet;

type DigitDash = Option<char>;

struct DigitalNumber {
	top: DigitDash,
	middle: DigitDash,
	bottom: DigitDash,
	top_right: DigitDash,
	top_left: DigitDash,
	bottom_right: DigitDash,
	bottom_left: DigitDash
}

fn decode_line (line: &str) -> i32 {
	let mut split_line = line.split(" | ");
	let (code, number) = (split_line.next().unwrap(), split_line.next().unwrap());

	let zeroOrSixOrNine = get_words_with_length(code, 6);
	12
}

fn main() -> utils::Result<()> {
	let input = include_str!("input");

	for line in input.lines() {
		decode_line(line);
	}

	Ok(())
}

fn get_words_with_length (code: &str, length: usize) -> Vec<String> {
	code.split(" ")
		.filter_map(|word| if word.len() == length {Some(String::from(word))} else {None})
		.collect::<Vec<String>>()
}

fn get_word_with_length (code: &str, length: usize) -> String {
	get_words_with_length(code, length)[0].clone()
}

fn find_1 (code: &str, zeroOrSixOrNine: Vec<String>) -> (char, char, String) {
	let oneLetters = get_word_with_length(code, 2);
	let mut six:Option<String> = None;
	let top_right:DigitDash = None;
	let bottom_right:DigitDash = None;

	for maybeSix in zeroOrSixOrNine {
		let letters: HashSet<char> = maybeSix.chars().collect();
		if !oneLetters.chars().all(|letter| letters.contains(&letter)) {
			for letters
			six = Some(maybeSix);
			top_right = letters
			continue;
		}
	}

	"toto"
}

fn find_4 (code: &str) -> String {
	get_word_with_length(code, 4)
}

fn find_7 (code: &str) -> String {
	get_word_with_length(code, 3)
}

fn find_8 (code: &str) -> String {
	get_word_with_length(code, 7)
}

// do unit test
fn find_char_not_in_string (the_string: String, the_char: char) {

}
