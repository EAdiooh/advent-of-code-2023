
static RED_CUBES: i32 = 12;
static GREEN_CUBES: i32 = 13;
static BLUE_CUBES: i32 = 14;

fn main() {
  let input = include_str!("./input1.txt");
  let output = part1(input);
  dbg!(output);
}


fn part1(input: &str) -> i32{
  let mut total: i32 = 0;
  let games: Vec<&str> = input.split('\n').collect();
  let mut realistic_game: bool;

  for game in games {
    realistic_game = true;
    
    //separate game id from score
    let details: Vec<&str> = game.split(':').collect();

    let game_result: Vec<&str> = details[1].split(';').collect();

    for result in  game_result{
      let draw: Vec<&str> = result.split(',').collect();

      for det in draw {

          if det.contains("green") {
            let t: String = det.chars().filter(|c| c.is_digit(10)).collect();

            if t.parse::<i32>().unwrap() > GREEN_CUBES {
              realistic_game = false;
            }
          }

          if det.contains("red") {
            let t: String = det.chars().filter(|c| c.is_digit(10)).collect();

            if t.parse::<i32>().unwrap() > RED_CUBES {
              realistic_game = false;
            }
          }

          if det.contains("blue") {
            let t: String = det.chars().filter(|c| c.is_digit(10)).collect();

            if t.parse::<i32>().unwrap() > BLUE_CUBES {
              realistic_game = false;
            }
          }

          if !realistic_game {
            break
          }
      }

      if !realistic_game {
        break
      }
    }

    if realistic_game {
      let id_game: String = details[0].chars().filter(|c| c.is_digit(10)).collect();
      
      total = total + id_game.parse::<i32>().unwrap();
    } 
  }

  return total;
}



#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = include_str!("./input1Test.txt");
      let result: i32 = part1(input);
      assert_eq!(result, 8);
  }
}