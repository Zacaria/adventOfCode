use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use itertools::Itertools;


fn main() -> io::Result<()> {

  let file = File::open("src/day1/input1.txt")?;
  let reader = BufReader::new(file);

  let lines = reader.lines()
    .map(|l| l.unwrap().parse::<i32>().unwrap());

  let mut counter = 0;

  for (l1, l2, l3, l4) in lines.tuple_windows() {
    if l1 + l2 + l3 < l2 + l3 + l4 {
      counter += 1;
    }
  }

  println!("result {}", counter);

  Ok(())
}
