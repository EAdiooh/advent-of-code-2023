#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
struct Race{
  time: u64,
  distance: u64
}

impl Race {

  fn create_races(time: Vec<u64>, distance: Vec<u64>) -> Vec<Race>{
    let mut races: Vec<Race> = Vec::new();
    for (index, value) in time.iter().enumerate(){
      races.push( Race {
        time: *value,
        distance: distance[index]
      })
    }

    return races;
  }

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
      println!("{} {} {}", self.time, min, simulated_dist);
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
  let output = part1(input);
  dbg!(output);
}

fn part1(input: &str) -> u64{
  let v: Vec<&str> = input.split('\n').collect();
  let time: Vec<u64> = process_line(v[0].split(": ").skip(1).collect());
  let distances: Vec<u64> = process_line(v[1].split(": ").skip(1).collect());
  
  let races: Vec<Race> = Race::create_races(time, distances);
  println!("{:?}", races);

  let mut min_win: Vec<u64> = Vec::new();
  let mut max_win: Vec<u64> = Vec::new();
  for race in races{
    let t :u64 = race.get_min_button_push_win();
    let y :u64 = race.get_max_button_push_win(t);
    min_win.push(t);
    max_win.push(y)
  }
  println!("{:?} {:?}", min_win, max_win);
  let mut win: Vec<u64> = Vec::new();
  for (index, value) in max_win.iter().enumerate() {
      win.push((value - min_win[index]) + 1);
  }


  let mut total: u64 = 1;
  for value in win {
      total = total * value;
  }
  return  total;
}

fn process_line(string_to_process: String) -> Vec<u64>{
  let mut temp: Vec<&str> = string_to_process.split(" ").collect::<Vec<&str>>();

  temp.retain(|&i|i  != "");

  return temp.iter()
              .map(|s| s.parse::<u64>().expect("Seed string to u64"))
              //return a vec of seed as u64
              .collect::<Vec<u64>>();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = include_str!("./inputTest.txt");
      let result: u64 = part1(input);
      assert_eq!(result, 288);
  }
}