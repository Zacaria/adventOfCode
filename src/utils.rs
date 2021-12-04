use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};

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

