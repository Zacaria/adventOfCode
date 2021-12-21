use std::collections::HashSet;

type DigitDash = Option<char>;

struct DigitalMap {
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

	let one_code = get_word_with_length(code, 2);
	let four_code = get_word_with_length(code, 4);
	let seven_code = get_word_with_length(code, 3);
	let eight_code = get_word_with_length(code, 7);
	let zero_six_or_nine = get_words_with_length(code, 6);
	let two_three_or_five = get_words_with_length(code, 5);

	let (top_right, six_code, zero_or_nine) = letter_not_in_code(&zero_six_or_nine, &one_code);
	let (middle, zero_code, nine_vec) = letter_not_in_code(&zero_or_nine, &four_code);
	let nine_code = nine_vec[0];
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

// fn find_1 (code: &str, zeroOrSixOrNine: Vec<String>) -> (char, char, String) {
// 	let oneLetters = get_word_with_length(code, 2);
// 	let mut six:Option<String> = None;
// 	let top_right:DigitDash = None;
// 	let bottom_right:DigitDash = None;

// 	for maybeSix in zeroOrSixOrNine {
// 		let letters: HashSet<char> = maybeSix.chars().collect();
// 		if !oneLetters.chars().all(|letter| letters.contains(&letter)) {
// 			for letters
// 			six = Some(maybeSix);
// 			top_right = letters
// 			continue;
// 		}
// 	}

// 	"toto"
// }

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

fn letter_not_in_code (codes: &Vec<String>, letters: &String) -> (Option<char>, Option<String>, Vec<String>) {
	let mut found_code: Option<String> = None;
	let mut found_letter: Option<char> = None;
	for code in codes {
		for letter in letters.chars() {
			if !code.contains(letter) {
				found_letter = Some(letter);
				found_code = Some(code.clone());
				continue;
			}
		}
		if found_code != None {
			continue;
		}
	}
	let new_codes: Vec<String> = codes.iter().filter(|&code| code.eq(found_code.as_ref().unwrap())).collect();
	(found_letter, found_code, new_codes)
}
