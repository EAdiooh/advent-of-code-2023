use std::collections::HashMap;

fn main() {
  let input = include_str!("./input2.txt");
  let output = part2(input);
  dbg!(output);
}

fn part2(input: &str) -> i32{
  let number_as_string: HashMap<String, String> = HashMap::from([
                                                              ("one".to_owned(), String::from("1")),
                                                              ("two".to_owned(), String::from("2")),
                                                              ("three".to_owned(), String::from("3")),
                                                              ("four".to_owned(), String::from("4")),
                                                              ("five".to_owned(), String::from("5")),
                                                              ("six".to_owned(), String::from("6")),
                                                              ("seven".to_owned(), String::from("7")),
                                                              ("eight".to_owned(), String::from("8")),
                                                              ("nine".to_owned(), String::from("9"))
                                                          ]);


  let mut total: i32 = 0;
  let v: Vec<&str> = input.split('\n').collect();
  let mut calibration: String = String::from("");


  for line in v {
    //get first number
    let mut save_string: String = String::from("");

    for char in line.chars() {
      if char.is_numeric(){
        calibration.push(char);
        break;
      }else{
        save_string.push(char);
      }

      for (key, value) in  number_as_string.iter(){
          if save_string.find(key) != None {
            calibration.push_str(value);
            break;
          }
      }
      if calibration.len() > 0 {
        break
      }
    }

    save_string = String::from("");

    for char in line.chars().rev() {
      if char.is_numeric(){
        calibration.push(char);
        break;
      }else{
        save_string.insert(0, char);
      }

      
      for (key, value) in  number_as_string.iter(){
          if save_string.find(key) != None {
            calibration.push_str(value);
            break;
          }
      }

      if calibration.len() > 1 {
        break
      }
    }
    
    let to_int: i32 = calibration.parse::<i32>().unwrap();
    total = total + to_int;
    calibration = String::from("");
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
      assert_eq!(result, 281);
  }
}