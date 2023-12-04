use std::collections::HashMap;

fn main() {
  let input = include_str!("./input.txt");
  let output = part2(input);
  dbg!(output);
}


fn part2(input: &str) -> i32{
  let mut total: i32 = 0;
  let mut winning_numbers: Vec<&str>;
  let mut having_numbers: Vec<&str>;
  let mut cards_total: HashMap<i32, i32> = HashMap::new();

  for (i, _line) in input.lines().enumerate() {
    cards_total.insert(i as i32 +1, 1);
  }
  //better way to iterate through each line
  for (i, line) in input.lines().enumerate() {

    let numbers: &str =  line.split(":").last().unwrap();

    let num: Vec<&str> = numbers.split("|").collect();

    winning_numbers = num[0].trim().split(" ").collect();
    having_numbers = num[1].trim().split(" ").collect();

    winning_numbers.retain(|&i|i  != "");
    having_numbers.retain(|&i|i  != "");

    //Need to find how to get and mut properly
    let t: HashMap<i32, i32> = cards_total.clone();
    let multiply: &i32 =t.get(&(i as i32 + 1)).unwrap();

    let mut line_gains: i32 = 1;
    for win in  winning_numbers{
      for have in  &having_numbers{
          if have == &win {
            cards_total.entry(i as i32 + (1 + line_gains))
            .and_modify(|e| { *e += 1 * multiply });
           line_gains+= 1;
          }
      }
    }
  }

  for (_key, value) in cards_total {
      total = total + value;
  }
  return total;

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = include_str!("./inputTest.txt");
      let result: i32 = part2(input);
      assert_eq!(result, 30);
  }
}