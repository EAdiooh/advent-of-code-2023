/*

*/

use std::ops::{Add, AddAssign};

use grid::{Grid, grid};

#[derive(Debug, PartialEq, Clone)]
struct Cell{ 
  content: String
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

impl Point {
  pub const fn new(x: i32, y: i32) -> Self {
      Point { x, y }
  }
}

impl Add for Point {
  type Output = Self;

  #[inline]
  #[must_use]
  fn add(self, rhs: Self) -> Self {
      Point::new(self.x + rhs.x, self.y + rhs.y)
  }
}

impl AddAssign for Point {
  #[inline]
  fn add_assign(&mut self, rhs: Self) {
      self.x += rhs.x;
      self.y += rhs.y;
  }
}

pub const UP: Point = Point::new(0, -1);
pub const DOWN: Point = Point::new(0, 1);
pub const LEFT: Point = Point::new(-1, 0);
pub const RIGHT: Point = Point::new(1, 0);

fn main() {
  let input = include_str!("./input.txt");
  let output = part1(input);
  dbg!(output);
}

fn build_grid(data: Vec<&str>) -> Grid<Cell>{
  let mut grid: Grid<Cell> = grid![];

  for datum in  data{
    let row: Vec<Cell> = datum.chars().map(|x| Cell{content: x.to_string()}).collect();
    grid.push_row(row);
  }

  return grid;
}

fn process_path(grid: Grid<Cell>, start_coordinate: Point) -> i64{
  let mut counter:i64 = 0;
  let mut direction: Point = DOWN;
  

  let coor: Point = start_coordinate + UP;
  let next:Cell = grid.get(coor.y as usize, coor.x as usize).expect("ss").clone();
  
  if matches!(next.content.as_str(), "|" | "7" | "F") { 
    direction = UP; 
  }

  let mut position = start_coordinate + direction;
  
  loop {
    let cell:Cell = grid.get(position.y as usize, position.x as usize).expect("ss").clone();

    direction = match cell.content.as_str() {
      "-" | "|" => direction,
      "7" if direction == UP => LEFT,
      "F" if direction == UP => RIGHT,
      "J" if direction == DOWN => LEFT,
      "L" if direction == DOWN => RIGHT,
      "J" | "L" => UP,
      "7" | "F" => DOWN,
      _ => {
          counter += 1;
          break;
      }
    };
  position += direction;
  counter += 1;

  }

  return counter/2;
}

fn part1(input: &str) -> i64{
  let data: Vec<&str> = input.split('\n').collect();

  let grid: Grid<Cell> = build_grid(data);

  //get coord S
  let mut hamster_coor: Point = Point::new(0, 0);
  for (point, cell) in grid.indexed_iter() {
      if cell.content == "S" {
        hamster_coor.x = point.1 as i32;
        hamster_coor.y = point.0 as i32;
      }
  }
  
  

  let total: i64 = process_path(grid, hamster_coor);

  return total;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = include_str!("./inputTest.txt");
      let result: i64 = part1(input);
      assert_eq!(result, 4);
  }
}