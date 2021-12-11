use grid::Grid;
use std::fmt;

mod cell;
use cell::{Cell};

struct Coord {
  row: usize,
  col: usize
}

fn print<T: fmt::Debug> (grid: &Grid<T>) {
  if grid.cols() > 0 {
    for (row, _) in (0..grid.rows()).enumerate() {
      for (col, _) in (0..(grid.cols())).enumerate() {
        let cell: &T = grid.get(row, col).unwrap();
        print!("{:?} ", *cell);
      }
      println!();
    }
    println!();
  }
}

fn mark<T: Copy + PartialEq> (grid: &mut Grid<Cell<T>>, value: T) -> Option<Coord> {
  for (row, _) in (0..grid.rows()).enumerate() {
    for (col, _) in (0..(grid.cols())).enumerate() {
      let cell_value = match grid.get(row, col).unwrap() {
        Cell::Clean(x) => Some(x),
        _ => None,
      };
      if cell_value == None {
        continue;
      }

      if cell_value.unwrap() == &value {
        let new_cell = Cell::Marked(*cell_value.unwrap());
        let c = grid.get_mut(row, col).unwrap();
        *c = new_cell;
        return Some(Coord{row, col})
      }
    }
  }
  None
}

fn is_complete<T> (grid: &Grid<Cell<T>>, coords: Coord) -> bool {
  let is_line_complete = grid.iter_row(coords.row)
    .all(|x| match x {
      Cell::Marked(_) => true,
      _ => false
    });
  let is_column_complete = grid.iter_col(coords.col)
    .all(|x| match x {
      Cell::Marked(_) => true,
      _ => false
    });

  is_column_complete || is_line_complete
}

fn grid_value (grid: &Grid<Cell<u32>>, last_marked: u32) -> u32 {
  let sum_clean_cells = grid.iter()
    .filter_map(|x| match x {
      Cell::Clean(x) => Some(x),
      _ => None
    })
    .fold(0, |value, x| value + x);

  sum_clean_cells * last_marked
}

fn main () -> utils::Result<()> {
  let input = include_str!("input.txt");
  let seq: Vec<_> = input.lines().collect();

  let first_line = seq.iter().next().ok_or("first line not found")?;
  let grids_string: Vec<_> = seq.iter().skip(1)
    .enumerate()
    .filter_map(|(i, e)| if i % 6 != 0 { Some(e) } else { None })
    .collect();

  let mut grids: Vec<Grid<Cell<u32>>> = Vec::new();

  for (i, _) in grids_string.iter().step_by(5).enumerate() {
    let mut grid_holder: Vec<Cell<u32>> = Vec::new();
    for line_in_grid in 0..5 {
      let absolute_line_position = i*5+line_in_grid;
      let mut line: Vec<_> = grids_string[absolute_line_position]
        .split(" ")
        .filter_map(|x| if x.len() > 0 { Some(Cell::Clean(x.trim().parse::<u32>().unwrap())) } else { None })
        .collect();
      grid_holder.append(&mut line);
    }
    grids.push(Grid::from_vec(grid_holder, 5));
  }


  let instructions: Vec<_> = first_line
    .split(",")
    .map(|l| l.parse::<u32>().unwrap())
    .collect();

  for instruction in instructions {
    for grid in grids.iter_mut() {
      let coords = mark(grid, instruction);
      match coords {
        None => continue,
        _ => ()
      }
      if is_complete(grid, coords.unwrap()) {
        println!("found {}", grid_value(grid, instruction));
        return Ok(());
      }
    }
  }

  Ok(())
}