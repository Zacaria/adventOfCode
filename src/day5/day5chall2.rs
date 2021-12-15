use std::str::Lines;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}

struct Segment {
  a: Point,
  b: Point
}

impl Segment {
  fn line (&self) -> Vec<Point> {
    let mut res: Vec<Point> = Vec::new();
    if self.a.y == self.b.y {
      for x in if self.a.x < self.b.x { self.a.x..=self.b.x } else { self.b.x..=self.a.x }  {
        res.push(Point{x, y: self.a.y});
      }
    } else if self.a.x == self.b.x {
      for y in if self.a.y < self.b.y { self.a.y..=self.b.y } else { self.b.y..=self.a.y } {
        res.push(Point{x: self.a.x, y});
      }
    } else {
      let a_x_smaller = self.a.x < self.b.x;
      let a_y_smaller = self.a.y < self.b.y;
      let range = if a_x_smaller { self.a.x..=self.b.x } else { self.b.x..=self.a.x };

      for (index, _) in range.enumerate() {
        let x = if a_x_smaller { self.a.x + index as i32 } else { self.a.x - index as i32 };
        let y = if a_y_smaller { self.a.y + index as i32 } else { self.a.y - index as i32 };

        res.push(Point{x, y});
      }
    }
    res
  }
}

fn read_segments (lines: Lines, straight: bool) -> Vec<Segment> {
  let seq: Vec<Segment> = lines
    .filter_map(|line| -> Option<Segment> {
      let points: Vec<Point> = line
        .split(" -> ")
        .map(|point|{
          let coords: Vec<i32> = point.split(",").map(|coord| coord.parse::<i32>().unwrap()).collect();
          Point{x: coords[0], y: coords[1]}
        })
        .collect();
      let segment = Segment{a: points[0], b: points[1]};

      if !straight || (segment.a.x == segment.b.x || segment.a.y == segment.b.y) {
        return Some(segment)
      }
      None
    })
    .collect();
  seq
}

fn count_points (segments: Vec<Segment>) -> HashMap<Point, i32> {
  segments.iter()
    .fold(HashMap::new(), |mut map: HashMap<Point, i32>, segment| {
      for point in segment.line() {
        map.insert(point,
          if map.contains_key(&point) {
            map.get(&point).unwrap() + 1
          } else {
            1
          }
        );
      }
      return map
    })
}

fn main () -> utils::Result<()> {
  let input = include_str!("input.txt");

  let segments_1 = read_segments(input.lines(), true);
  let counted_points_1 = count_points(segments_1);
  let result_1 = counted_points_1.iter().filter(|(_point, &value)| value > 1).count();

  let segments_2 = read_segments(input.lines(), false);
  let counted_points_2 = count_points(segments_2);
  let result_2 = counted_points_2.iter().filter(|(_point, &value)| value > 1).count();

  println!("solution 1 : {} - solution 1 : {}", result_1, result_2);
  Ok(())
}