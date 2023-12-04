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
  let output = part2(input);
  dbg!(output);
}


fn part2(input: &str) -> i32{
  let mut total: i32 = 0;
  let mut number_building: String = "".to_owned();
  let mut current_points: Vec<(i32, i32)> = Vec::new();
  let mut numbers: Vec<(i32, Vec<Point>)> = Vec::new();
  let mut symbols: Vec<Point> = Vec::new();
  let mut gears: Vec<Point> = Vec::new();

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

        //GEARRRRR
        '*' => {
          gears.push((x as i32, y as i32));
        },
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
    
    if number_building != "".to_owned() {
      numbers.push((number_building.parse::<i32>().unwrap(), current_points.clone()));
      number_building = "".to_owned();
      current_points.clear();
    }
  }

  //get the last number in case the grid end with a number
  if number_building != "".to_owned() {
    numbers.push((number_building.parse::<i32>().unwrap(), current_points.clone()));
    current_points.clear();
  }


  //iterate through gears to get all numbers link to one gear
  
  for (x, y) in gears {
    let mut last_point: Vec<(i32, i32)> = Vec::new();
    let mut geared: Vec<&i32> = Vec::new();

    for check_point in COORTOCHECK.iter().map(|(dx, dy)| (x + dx, y + dy)) {
      for (number, points) in &numbers{
        
        if points.contains(&check_point) && (!last_point.contains(&check_point) && !geared.contains(&number)  ) {
              
          geared.push(number);
          //Adding all point linked to the numbers for not repeating ourself
          last_point = points.clone();
        }
      }
     
    }

    if geared.len() > 1 {
      println!("{:?}", geared);
      total += geared[0] * geared[1];
    }
  }

  return total;
//84883664

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = include_str!("./inputTest2.txt");
      let result: i32 = part2(input);
      assert_eq!(result, 6756);
  }
}