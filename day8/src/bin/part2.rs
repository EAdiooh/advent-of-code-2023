use std::collections::HashMap;

use prime_factorization::Factorization;

#[derive(Debug, Clone)]
struct Node {
  node: String,
  left_edge: String,
  right_edge: String
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Road{
  road: Vec<Direction>
}

fn main() {
  let input = include_str!("./input.txt");
  let output = part2(input);
  dbg!(output);
}

fn build_nodes(data: Vec<&str>) -> Vec<Node>{
  let mut nodes: Vec<Node> = Vec::new();

  for datum in data{
    let node: &str = datum.split(" = ").next().expect("Retrieve node");
    let edges: &str = datum.split("= ").last().expect("Retrieve edges");

    let tedges = edges.replace(&['(', ')', ',', '\"', '.', ';', ':', '\''][..], "");

    let left_edge: &str = tedges.split(" ").next().expect("Left edge");
    let right_edge: &str = tedges.split(" ").last().expect("Right edge");

    nodes.push(Node { 
                      node: node.to_owned(),
                      left_edge: left_edge.to_owned(),
                      right_edge: right_edge.to_owned() 
                    }
              )
  }

  return nodes;
}

fn part2(input: &str) -> u64{
  let mut data: Vec<&str> = input.split('\n').collect();

  let instructions: Vec<char> = data[0].chars().collect();

  let mut tempo: Vec<Direction> = Vec::new();
  
  for instru in  instructions{
      match instru {
          'L' => tempo.push(Direction::Left),
          'R' =>tempo.push(Direction::Right),
          _ => panic!("Invalid direction"),
      }
  }

  let road: Road = Road { road: tempo };

  data.remove(0);
  data.remove(0);

  
  let nodes: Vec<Node> = build_nodes(data);

  let mut positions: Vec<String> = nodes.iter()
                                        .filter(|node| node.node.ends_with('A'))
                                        .map(|node| node.node.clone())
                                        .collect::<Vec<String>>();

  let mut counter: Vec<u32> = Vec::new();

  let mut directions_iterator = road.road.iter().cycle();

  for position in positions.iter_mut(){
    let mut counting: u32 = 0;

    while !position.ends_with('Z') {

      let current_direction = directions_iterator.next().unwrap();
  
     
      let index: usize = nodes.iter().position(|r| &r.node == position).expect("Get index of position");
  
      let next_node_name = match current_direction {
          Direction::Left => nodes[index].left_edge.clone(),
          Direction::Right =>  nodes[index].right_edge.clone(),
      };
  
      *position = next_node_name;
        
      
      counting += 1;
    }

    counter.push(counting);
  }

  let mut n: Vec<Vec<u32>> = Vec::new();

  
  //Result will be PPCM () -> 21366921060721 
  for number in counter {
    let factor_repr = Factorization::<u32>::run(number);
    n.push(factor_repr.factors);
  }

  let mut number_exponent: HashMap<u32, u32> = HashMap::new();
  for d in n.iter() {
    let mut tempo : HashMap<u32, u32> = HashMap::new();
    for value in d {
      *tempo.entry(*value).or_insert(0) += 1;
    }
    number_exponent = merge_hashmap(number_exponent, tempo);
  }

  let mut total: u64 = 1;

  for (value, exponent) in  number_exponent{
    total = total * ((value * exponent) as u64);
  }
  return total;
}

fn merge_hashmap(hash1: HashMap<u32, u32>, hash2: HashMap<u32, u32>) -> HashMap<u32, u32>{
  let mut new_context = HashMap::new();
  for (key, value) in hash1.iter() {
      new_context.insert(*key, *value);
  }

  for (key, value) in hash2.iter() {
    if !new_context.contains_key(key) {
      new_context.insert(*key, *value);
    }else{
      *new_context.entry(*key).or_insert(0) = *value;
    }
  }
  new_context
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = include_str!("./input2Test.txt");
      let result: u64 = part2(input);
      assert_eq!(result, 6);
  }
}