type Point = (i32, i32);


const COORTOCHECK: [Point; 8] = [ (0, -1),
                                  (1, -1),
                                  (1, 0),
                                  (1, 1),
                                  (0, 1),
                                  (-1, 1),
                                  (-1, 0),
                                  (-1, -1)];


fn main() {
  let input = include_str!("./input.txt");
  let output = part1(input);
  dbg!(output);
}


fn part1(input: &str) -> i32{
  let mut total: i32 = 0;
  let mut number_building: String = "".to_owned();
  let mut current_points: Vec<(i32, i32)> = Vec::new();
  let mut numbers: Vec<(i32, Vec<Point>)> = Vec::new();
  let mut symbols: Vec<Point> = Vec::new();

  //better way to iterate through each line
  for (y, line) in input.lines().enumerate() {
    for (x, value) in line.chars().enumerate() {
      
      
      if value.is_ascii_digit() {
        number_building.push(value);
        current_points.push((x as i32, y as i32));
  
        continue;
      }

      match value {
        // Ignore empty spaces
        '.' => (),
        _ => {
            symbols.push((x as i32, y as i32));
        }
    };


      if number_building != "".to_owned() {
        numbers.push((number_building.parse::<i32>().unwrap(), current_points.clone()));
        number_building = "".to_owned();
        current_points.clear();
      }
    }    
  }

  
  //Foreach numbers, we iterate through the coord link to the numbers and check if there is
  // a coord in symbols
  for (number, points) in numbers {
    'points: for (x, y) in points {
        for check_point in COORTOCHECK.iter().map(|(dx, dy)| (dx + x, dy + y)) {
            
            if symbols.contains(&check_point) {
              
              total = total + number;

              // We don't need to check any more points for this number
              break 'points;
            }
        }
    }
}
  return total;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = include_str!("./inputTest.txt");
      let result: i32 = part1(input);
      assert_eq!(result, 4361);
  }
}