use std::str::Lines;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
  x: u32,
  y: u32,
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
    }
    if self.a.x == self.b.x {
      for y in if self.a.y < self.b.y { self.a.y..=self.b.y } else { self.b.y..=self.a.y } {
        res.push(Point{x: self.a.x, y});
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
          let coords: Vec<u32> = point.split(",").map(|coord| coord.parse::<u32>().unwrap()).collect();
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

fn count_points (segments: Vec<Segment>) -> HashMap<Point, u32> {
  segments.iter()
    .fold(HashMap::new(), |mut map: HashMap<Point, u32>, segment| {
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
  let segments = read_segments(input.lines(), true);
  let counted_points = count_points(segments);

  let result = counted_points.iter().filter(|(_point, &value)| value > 1).count();

  println!("{}", result);
  Ok(())
}