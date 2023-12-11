

fn main() {
  let input = include_str!("./input.txt");
  let output = part1(input);
  dbg!(output);
}

fn parse(data: Vec<&str>) -> i64 {
  let mut total: i64 = 0;

  for datum in data {
    let tempo: Vec<i64> = datum.split(" ")
                              .map(|x| x.parse::<i64>().expect("Convert str to i64"))
                              .collect();
      
    
    total = total + find_value(tempo);
  }


  return total;
}

fn find_value(values: Vec<i64>) -> i64{
  let mut report: Vec<Vec<i64>> = build_history(values.clone());

  report.insert(0, values.clone());

  let mut last_value: i64 = 0;

  for (index, value) in report.iter().rev().enumerate(){
    if index == report.len(){
      continue;
    }

    last_value = value[0] - last_value ;

  }
  
  return last_value;
}

fn build_history(values: Vec<i64>) -> Vec<Vec<i64>>{
  let mut return_value: Vec<Vec<i64>> = Vec::new();
  let mut tempo: Vec<i64> = Vec::new();

  for (index, value) in values.iter().enumerate() {
    if values.get(index+1) != None {
      tempo.push(values.get(index+1).expect("Value next index") - value);
    }
  }

  return_value.push(tempo.clone());
  let number_of_zero: Vec<&i64> = tempo.iter().filter(|x| x != &&0).collect();
  if number_of_zero.len() > 0 {
    return_value.append(&mut build_history(tempo));
  }

  return return_value;
}

fn part1(input: &str) -> i64{
  let data: Vec<&str> = input.split('\n').collect();

  let total: i64 = parse(data);

  
  return total;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = include_str!("./inputTest.txt");
      let result: i64 = part1(input);
      assert_eq!(result, 2);
  }
}