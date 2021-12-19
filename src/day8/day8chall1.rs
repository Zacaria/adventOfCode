use itertools::Itertools;

fn main () -> utils::Result<()> {
  let input = include_str!("input");

  let result = input
    .lines()
    .map(|line| line.split(" | ").collect::<Vec<&str>>()[1])
    .join(" ")
    .split(" ")
    .filter(|word| word.len() == 2 || word.len() == 3 || word.len() == 4  || word.len() == 7)
    .count();
  println!("{}", result);
  Ok(())
}