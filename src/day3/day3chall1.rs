use std::io;

fn main() -> io::Result<()> {
  let lines = utils::get_lines("src/day3/input.txt");

  let (bits_counter, total_lines) = count_bits(lines);

  let (epsilon, gamma) = get_epsilon_gamma(bits_counter, total_lines);

  let e = utils::to_u32(&epsilon);
  let g = utils::to_u32(&gamma);

  println!("e {} - g {} - result {}", e, g, e * g);

  Ok(())
}

fn count_bits(lines: Vec<String>) -> ([i32; 12], i32) {
  let mut bits_counter: [i32; 12] = [0; 12];
  let mut total_lines = 0;

  for line in lines {
    for (index, bit) in line.chars().enumerate() {
      if bit.to_digit(10).unwrap() == 1 {
        bits_counter[index] += 1;
      }
    }
    total_lines += 1;
  }

  (bits_counter, total_lines)
}

fn get_epsilon_gamma(bits_counter: [i32; 12], total: i32) -> ([u8; 12], [u8; 12]) {
  let mut epsilon: [u8; 12] = [0; 12];
  let mut gamma: [u8; 12] = [0; 12];
  for index in 0..bits_counter.len() {
    let has_more_one = bits_counter[index] > (total / 2);
    epsilon[index] = if has_more_one { 1 } else { 0 };
    gamma[index] = if has_more_one { 0 } else { 1 };
  }

  println!(
    "total {} - counter {:?} - epsilon {:?}  - gamma {:?}",
    total, bits_counter, epsilon, gamma
  );

  (epsilon, gamma)
}
