fn main () -> utils::Result<()> {
  let input = include_str!("input");
  let positions: Vec<i32> = input.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
  let min_pos = *positions.iter().min().unwrap();
  let max_pos = *positions.iter().max().unwrap();
  let mut cheapest = None;

  for pos in min_pos..max_pos {
    let consumption = positions.iter().fold(0, |acc, p| acc + (p - pos).abs());
    cheapest = if cheapest == None || consumption < cheapest.unwrap() { Some(consumption) } else { cheapest };
  }

  println!("cheapest {}", cheapest.unwrap());
  Ok(())
}