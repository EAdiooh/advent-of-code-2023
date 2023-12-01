fn main() {
  let input = include_str!("./input1.txt");
  let output = part1(input);
  dbg!(output);
}

fn part1(input: &str) -> i32{
  let mut total: i32 = 0;
  let v: Vec<&str> = input.split('\n').collect();
  
  for line in v {

    //Deleting all no numerics char
    let t: String = line.chars().filter(|c| c.is_digit(10)).collect();

    //getting first and last digit
    let first_char = t.split("").nth(1).unwrap();
    let second_char = t.split("").nth(t.len()).unwrap();

    //Concat the two digit and tranform it to int for getting the calibration value for this line
    let  calibration: i32 =format!("{first_char}{second_char}").parse::<i32>().unwrap();

    total = total + calibration;
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
      assert_eq!(result, 142);
  }
}