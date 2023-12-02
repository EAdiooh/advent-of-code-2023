fn main() {
  let input = include_str!("./input2.txt");
  let output = part2(input);
  dbg!(output);
}


fn part2(input: &str) -> i32{
  let mut total: i32 = 0;
  let games: Vec<&str> = input.split('\n').collect();

  for game in games {

    let mut min_blue: i32 = 0;
    let mut min_red: i32 = 0;
    let mut min_green: i32 = 0;
    
    //separate game id from score
    let details: Vec<&str> = game.split(':').collect();

    let game_result: Vec<&str> = details[1].split(';').collect();

    for result in  game_result{
      let draw: Vec<&str> = result.split(',').collect();

      

      for det in draw {

          if det.contains("green") {
            let t: String = det.chars().filter(|c| c.is_digit(10)).collect();

            if t.parse::<i32>().unwrap() > min_green {
              min_green = t.parse::<i32>().unwrap();
            }
          }

          if det.contains("red") {
            let t: String = det.chars().filter(|c| c.is_digit(10)).collect();

            if t.parse::<i32>().unwrap() > min_red {
              min_red = t.parse::<i32>().unwrap();
            }
          }

          if det.contains("blue") {
            let t: String = det.chars().filter(|c| c.is_digit(10)).collect();

            if t.parse::<i32>().unwrap() > min_blue {
              min_blue = t.parse::<i32>().unwrap();
            }
          }

      }

      
    }

    
      
      total = total + (min_red * min_green * min_blue); 
  }

  return total;
}



#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = include_str!("./input2Test.txt");
      let result: i32 = part2(input);
      assert_eq!(result, 2286);
  }
}