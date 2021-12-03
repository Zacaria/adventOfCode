use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn to_u32(slice: &[u8]) -> u32 {
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

fn main() -> io::Result<()> {
  let file = File::open("src/day3/input.txt")?;
  let reader = BufReader::new(file);

  let mut bits_counter: [i32; 12] = [0; 12];
  let mut total_lines = 0;

  for line in reader.lines() {
    for (index, bit) in line.unwrap().chars().enumerate() {
      if bit.to_digit(10).unwrap() == 1 {
        bits_counter[index] += 1;
      }
    }
    total_lines += 1;
  }

  let mut epsilon: [u8; 12] = [0; 12];
  let mut gamma: [u8; 12] = [0; 12];
  for index in 0..bits_counter.len() {
    let has_more_one = bits_counter[index] > (total_lines / 2);
    epsilon[index] = if has_more_one { 1 } else { 0 };
    gamma[index] = if has_more_one { 0 } else { 1 };
  }

  let e = to_u32(&epsilon);
  let g = to_u32(&gamma);

  println!(
    "total {} - counter {:?} - epsilon {:?}  - gamma {:?}",
    total_lines, bits_counter, epsilon, gamma
  );
  println!("e {} - g {} - result {}", e, g, e * g);

  Ok(())
}
