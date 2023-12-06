#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
struct Race{
  time: u64,
  distance: u64
}

impl Race {

  fn get_min_button_push_win(self) -> u64{
    
    for time_push in 1..self.time {
      let simulated_dist: u64 = Self::simulate_boat_distance(time_push, self.time);
      if simulated_dist > self.distance {
        return time_push;
      }
    }

    return 0;
  }

  fn get_max_button_push_win(self, min: u64) -> u64{
   
    for time_push in (min..self.time).rev() {
      let simulated_dist: u64 = Self::simulate_boat_distance(time_push, self.time);
      if simulated_dist > self.distance {
        return time_push;
      }
    }

    return 0;
  }

  fn simulate_boat_distance(time_push: u64, time_limit: u64) -> u64{
    return time_push * (time_limit - time_push);
  }
}

fn main() {
  let input = include_str!("./input.txt");
  let output = part2(input);
  dbg!(output);
}

fn part2(input: &str) -> u64{
  let v: Vec<&str> = input.split('\n').collect();
  let time: String = v[0].split(": ").skip(1).collect();
  let distance: String = v[1].split(": ").skip(1).collect();
  let t: String = time.chars().filter(|c| c.is_digit(10)).collect();
  let d: String = distance.chars().filter(|c| c.is_digit(10)).collect();
  let race: Race = Race{
                    time: t.parse::<u64>().unwrap(), 
                    distance: d.parse::<u64>().unwrap()
                  };

  let min_win :u64 = race.get_min_button_push_win();
  let max_win :u64 = race.get_max_button_push_win(min_win);
  


  let total: u64 = (max_win - min_win) + 1;
  return  total;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = include_str!("./inputTest.txt");
      let result: u64 = part2(input);
      assert_eq!(result, 71503);
  }
}