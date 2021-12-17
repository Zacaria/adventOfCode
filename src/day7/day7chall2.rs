fn main () -> utils::Result<()> {
  let input = include_str!("input");
  let positions: Vec<i64> = input.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
  let min_pos = *positions.iter().min().unwrap();
  let max_pos = *positions.iter().max().unwrap();
  let mut cheapest = None;

  for (index, pos) in (min_pos..max_pos).enumerate() {
    let consumption = positions
      .iter()
      .fold(0, |acc, p| acc + (0..=(p - pos).abs()).sum::<i64>());
    cheapest = if cheapest == None || consumption < cheapest.unwrap() { Some(consumption) } else { cheapest };

    let prct = (index as f64/(max_pos-min_pos) as f64)*100f64;
    println!("{:.2} %", prct)
  }

  println!("cheapest {}", cheapest.unwrap());
  Ok(())
}