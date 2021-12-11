use std::fs::File;
use std::io::{prelude::*, BufReader};

pub type BitLine = [u8; 12];
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn to_u32(slice: &[u8]) -> u32 {
  // should check that slice length is no more than 32
  // slice.len() <= 32
  slice.iter().fold(0, |result, &bit| {
    // Ex result = 00110
    // result << 1 => 01100
    // ^ is OR, so it sets the bit at that place without affecting the rest of bits
    // 01100 ^ 1 => 01101
    // it's shifting everything to the left (result << 1) and placing a bit at the end (bits ^ 1)
    (result << 1) ^ bit as u32

    // println!("{:b}", result) // to print result in binary representation
  })
}

pub fn get_lines(file_path: &str) -> Vec<String> {
  let file = File::open(file_path);
  let reader = BufReader::new(file.unwrap());
  reader.lines().map(|x| x.unwrap()).collect()
}

pub fn count_bits(lines: &Vec<BitLine>) -> ([i32; 12], i32) {
  let mut bits_counter: [i32; 12] = [0; 12];
  let mut total_lines = 0;

  for line in lines {
    for index in 0..line.len() {
      if line[index] == 1 {
        bits_counter[index] += 1;
      }
    }
    total_lines += 1;
  }

  (bits_counter, total_lines)
}

pub fn lines_to_bit_lines (lines: &Vec<String>) -> Vec<BitLine> {
  let mut new_lines: Vec<BitLine> = vec![];
  for line in lines.iter() {
    let mut new_line:BitLine = [0; 12];
    for (index, bit_char) in line.chars().enumerate() {
      new_line[index] = if bit_char.to_digit(10).unwrap() == 1 { 1 } else { 0 }
    }

    new_lines.push(new_line)
  }
  new_lines
}

