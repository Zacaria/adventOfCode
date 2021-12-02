use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
  let file = File::open("src/day2/input1.txt")?;
  let reader = BufReader::new(file);

  let lines = reader.lines()
    .map(|l| {
      let line = l.unwrap();
      let instruction = line.split(" ").collect::<Vec<&str>>();
      let direction = String::from(instruction[0]);
      let value = instruction[1].parse::<i32>().unwrap();
      (direction, value)
    });

  let mut h_pos = 0;
  let mut depth = 0;

  for (direction, value) in lines {
    match direction.as_str() {
      "down" => depth += value,
      "up" => depth -= value,
      "forward" => h_pos += value,
      _ => ()
    }
  }

  println!("horiz {}, depth {}, result {}", h_pos, depth, h_pos*depth);

  Ok(())
}
