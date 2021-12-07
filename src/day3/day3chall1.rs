use std::io;

type BitLine = [u8; 12];

fn main() -> io::Result<()> {
  let lines = utils::get_lines("src/day3/input.txt");

  let bit_lines = utils::lines_to_bit_lines(&lines);

  let (bits_counter, total_lines) = utils::count_bits(&bit_lines);

  let (epsilon, gamma) = get_epsilon_gamma(bits_counter, total_lines);

  let e = utils::to_u32(&epsilon);
  let g = utils::to_u32(&gamma);

  println!("e {} - g {} - result {}", e, g, e * g);

  Ok(())
}

fn get_epsilon_gamma(bits_counter: [i32; 12], total: i32) -> ([u8; 12], [u8; 12]) {
  let mut epsilon: BitLine = [0; 12];
  let mut gamma: BitLine = [0; 12];
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
