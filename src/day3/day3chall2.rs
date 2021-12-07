use std::io;

pub type BitLine = [u8; 12];

fn main() -> io::Result<()> {
  let lines = utils::get_lines("src/day3/input.txt");

  let bit_lines = utils::lines_to_bit_lines(&lines);

  let (bits_counter, total_lines) = utils::count_bits(&bit_lines);

  let o2_criterias = get_o2_criterias(&bits_counter.to_vec(), total_lines);
  let co2_criterias = get_co2_criterias(&bits_counter.to_vec(), total_lines);

  println!("o2_criterias {:?} - co2_criterias {:?}", o2_criterias, co2_criterias);


  let o2 = filter_by_criterias(
    o2_criterias,
    utils::lines_to_bit_lines(&lines),
    0,
    get_o2_criterias
  );
  let co2 = filter_by_criterias(
    co2_criterias,
    utils::lines_to_bit_lines(&lines),
    0,
    get_co2_criterias
  );

  let val_o2 = utils::to_u32(&o2);
  let val_co2 = utils::to_u32(&co2);

  println!("o2 {} - co2 {} - result {}", val_o2, val_co2, val_o2*val_co2);

  Ok(())
}

fn get_o2_criterias (bits_counter: &Vec<i32>, total: i32) -> Vec<i32> {
  let mut criterias = vec![];
  if bits_counter.len() == 0 {
    return criterias
  }
  for index in 0..bits_counter.len() {
    criterias.push(if bits_counter[index] * 2 >= total { 1 } else { 0 });
  }

  println!("o2 total {} - counter {:?} - criterias {:?}", total, bits_counter, criterias);

  criterias
}

fn get_co2_criterias (bits_counter: &Vec<i32>, total: i32) -> Vec<i32> {
  let mut criterias = vec![];
  for index in 0..bits_counter.len() {
    criterias.push(if bits_counter[index] * 2 < total { 1 } else { 0 });
  }

  println!("co2 total {} - counter {:?} - criterias {:?}", total, bits_counter, criterias);

  criterias
}

fn filter_by_criterias (criterias: Vec<i32>, lines: Vec<BitLine>, position: u8, recompute: fn (&Vec<i32>, i32) -> Vec<i32>) -> BitLine {
  if lines.len() == 1 {
    return lines.first().cloned().unwrap();
  } else {
    let mut new_lines: Vec<BitLine> = Vec::new();
    let criteria = criterias[position as usize];

    for line in lines {
      if line[position as usize] == criteria as u8 {
        new_lines.push(line);
      }
    }

    let (bits_counter, total_lines) = utils::count_bits(&new_lines);

    let new_criterias = recompute(&bits_counter.to_vec(), total_lines);

    return filter_by_criterias(new_criterias, new_lines, position + 1, recompute)
  }
}
