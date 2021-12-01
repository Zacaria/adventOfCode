use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {

  let file = File::open("src/day1/input1.txt")?;
  let reader = BufReader::new(file);

  let mut previous_value = 0;
  let mut counter = 0;

  for (index, line) in reader.lines().enumerate() {
      let current_value = line?.parse::<i32>().unwrap();

      if index != 0 && previous_value < current_value {
        counter += 1;
      }
      previous_value = current_value;
  }

  println!("result {}", counter);

  Ok(())
}
