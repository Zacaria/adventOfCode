use std::str::Lines;

type Point = (usize, usize);

fn is_lower (a: Option<&usize>, center: &usize) -> bool {
  match a {
    Some(v) => {
      v > &center
    },
    None => true,
  }
}

fn is_low_point (map: &Vec<Vec<usize>>, point: Point) -> bool {
  let (x, y) = point;
  let current = &map[y][x];

  let is_uppest = y == 0;
  let is_lowest = y == map.len() - 1;
  let is_leftest = x == 0;
  let is_rightest = x == map.len() - 1;
  let up = if !is_uppest { Some(&map[y - 1][x]) } else { None };
  let down = if !is_lowest { Some(&map[y + 1][x]) } else { None };
  let left = if !is_leftest { Some(&map[y][x - 1]) } else { None };
  let right = if !is_rightest { Some(&map[y][x + 1]) } else { None };

  let is_low = is_lower(up, current) && is_lower(down, current) && is_lower(left, current) && is_lower(right, current);

  // println!("Point ({}, {}) : {} is_low : {}", x, y, current, is_low);

  return is_low;
}

fn parse_input () -> Vec<Vec<usize>> {
  include_str!("input")
    .lines()
    .map(|line| line
      .split("")
      .filter_map(|s| if !s.is_empty() { Some(s.parse::<usize>().unwrap())} else {None})
      .collect::<Vec<usize>>()
    )
    .collect::<Vec<Vec<usize>>>()
}

#[cfg(test)]
fn part_1() -> usize {
  let input = parse_input();
  let mut result = 0;

  for (j, line) in input.iter().enumerate() {
    for (i, val) in line.iter().enumerate() {
      if is_low_point(&input, (i, j)) {
        result += val + 1;
      }
    }
  }

  result
}

#[test]
fn test_part1() {
  assert_eq!(part_1(), 508)
}
