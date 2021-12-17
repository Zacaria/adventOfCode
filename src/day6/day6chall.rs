use std::collections::HashMap;

fn process_day(people: &HashMap<u32, u64>) -> HashMap<u32, u64> {
  let mut number_to_append = 0;
  let mut new_people = HashMap::new();

  for (&life_index, &count) in &*people {
    let new_life_index = if life_index == 0 {
      number_to_append += count;
      6
    } else {
      life_index - 1
    };
    new_people.insert(
      new_life_index,
      if new_people.contains_key(&new_life_index) {
        new_people.get(&new_life_index).unwrap() + &count
      } else {
        count
      },
    );
  }

  new_people.insert(8, number_to_append);
  new_people
}

fn process_for_days (people: &HashMap<u32, u64>, days: u32) {
  let mut new_people = people.clone();
  for day_i in 0..days {
    new_people = process_day(&new_people);
    println!("day {} finished with {:?}", day_i, &people);
  }
  let count: u64 = new_people.values().sum::<u64>();
  println!("{:?}", count);
}

fn main() -> utils::Result<()> {
  println!("===========START==========");
  let input = include_str!("input.txt");
  let count_people = |mut map: HashMap<u32, u64>, guy: &u32| {
    map.insert(
      *guy,
      if map.contains_key(&guy) {
        map.get(&guy).unwrap() + 1
      } else {
        1
      },
    );
    return map;
  };
  let people = input
    .split(",")
    .map(|x| x.parse::<u32>().unwrap())
    .collect::<Vec<u32>>()
    .iter()
    .fold(HashMap::new(), count_people);

  process_for_days(&people, 80); // challenge1
  process_for_days(&people, 256); // challenge2

  println!("===========END==========");
  Ok(())
}
