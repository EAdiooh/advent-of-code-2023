fn main() {
  let input = include_str!("./input.txt");
  let output = part1(input);
  dbg!(output);
}


fn part1(input: &str) -> i32{
  let mut total: i32 = 0;
  let mut winning_numbers: Vec<&str>;
  let mut having_numbers: Vec<&str>;

  //better way to iterate through each line
  for (_i, line) in input.lines().enumerate() {
    let numbers: &str =  line.split(":").last().unwrap();

    let num: Vec<&str> = numbers.split("|").collect();

    winning_numbers = num[0].trim().split(" ").collect();
    having_numbers = num[1].trim().split(" ").collect();

    winning_numbers.retain(|&i|i  != "");
    having_numbers.retain(|&i|i  != "");

    let mut line_gains: i32 = 0;
    for win in  winning_numbers{
        for have in  &having_numbers{
            if have == &win {
                if line_gains == 0 {
                  line_gains += 1;
                  continue;
                }else {
                  line_gains = line_gains *2;
                  continue;
                }
            }
        }
    }
    
    total += line_gains;
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
      assert_eq!(result, 13);
  }
}