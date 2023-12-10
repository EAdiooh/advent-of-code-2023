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
  let output = part1(input);
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

fn part1(input: &str) -> u64{
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

  let destination: String = String::from("ZZZ");

  let mut position: String = String::from("AAA");

  let mut counter: u64 = 0;

  let mut directions_iterator = road.road.iter().cycle();

  while destination != position {

    let current_direction = directions_iterator.next().unwrap();
    let index: usize = nodes.iter().position(|r| r.node ==  position).expect("Get index of position");

    let next_node_name = match current_direction {
        Direction::Left => nodes[index].left_edge.clone(),
        Direction::Right =>  nodes[index].right_edge.clone(),
    };

    position = next_node_name;
    counter += 1;
  }
  
  return counter;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = include_str!("./inputTest.txt");
      let result: u64 = part1(input);
      assert_eq!(result, 2);
  }
}