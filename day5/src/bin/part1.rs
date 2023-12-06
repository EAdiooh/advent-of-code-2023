fn main() {
  let input = include_str!("./input.txt");
  let output = part1(input);
  dbg!(output);
}


fn part1(input: &str) -> i64{
  let mut data: Vec<&str> = input.split('\n').collect();

  let mut seeds: Vec<&str> = data[0].split(" ").collect();
  seeds.remove(0);

  data.remove(0);
  data.retain(|&i|i  != "");

  let mut seeds_to_soil: Vec<Vec<i64>> = Vec::new();
  let mut soil_to_fertilizer: Vec<Vec<i64>> = Vec::new();
  let mut fertilizer_to_water: Vec<Vec<i64>> = Vec::new();
  let mut water_to_light: Vec<Vec<i64>> = Vec::new();
  let mut light_to_temperature: Vec<Vec<i64>> = Vec::new();
  let mut temperature_to_humidity: Vec<Vec<i64>> = Vec::new();
  let mut humidity_to_location: Vec<Vec<i64>> = Vec::new();

  let mut locations: Vec<i64> =  Vec::new();

  let indices: Vec<usize> = data
                .iter()
                .enumerate()
                .filter(|(_, &r)| r.contains("map"))
                .map(|(index, _)| index)
                .collect::<Vec<_>>();
  
  for (index, value) in data.iter().enumerate() {
    

    if  indices[0] < index && index < indices[1] {
      let val: Vec<i64> = value.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
      seeds_to_soil.push(val);
    }else if indices[1] < index && index < indices[2] {
      let val: Vec<i64> = value.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
      soil_to_fertilizer.push(val);
    }else if indices[2] < index && index < indices[3] {
      let val: Vec<i64> = value.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
      fertilizer_to_water.push(val);
    }else if indices[3] < index && index < indices[4] {
      let val: Vec<i64> = value.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
      water_to_light.push(val);
    }else if indices[4] < index && index < indices[5] {
      let val: Vec<i64> = value.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
      light_to_temperature.push(val);
    }else if indices[5] < index && index < indices[6] {
      let val: Vec<i64> = value.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
      temperature_to_humidity.push(val);
    }else if indices[6] < index {
      let val: Vec<i64> = value.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
      humidity_to_location.push(val);
    }
  }

  for seed in seeds {
    let soil: i64 = compare(seeds_to_soil.clone(), seed.parse::<i64>().unwrap());

    let fertilizer: i64 = compare(soil_to_fertilizer.clone(), soil);

    let water: i64 = compare(fertilizer_to_water.clone(), fertilizer);

    let light: i64 = compare(water_to_light.clone(), water);

    let temperature: i64 = compare(light_to_temperature.clone(), light);

    let humidity: i64 = compare(temperature_to_humidity.clone(), temperature);

    let location: i64 = compare(humidity_to_location.clone(), humidity);
    
    locations.push(location);
  }

  let min_value = locations.iter().min().unwrap();

  return min_value.clone();
}

fn compare(ref_value: Vec<Vec<i64>>, identifier: i64) -> i64{
  for value in &ref_value {
    if value[1] <= identifier && identifier <= (value[1] + value[2]) {
      return identifier + (value[0] - value[1]);
    }
  }

  return identifier;
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = include_str!("./inputTest.txt");
      let result: i64 = part1(input);
      assert_eq!(result, 35);
  }
}