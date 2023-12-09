use std::{cmp::Ordering, collections::HashMap};

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Hash, Copy, Clone)]
enum Card {
  Ace,
  King,
  Queen,
  Jack,
  Ten,
  Nine,
  Eight,
  Seven,
  Six,
  Five,
  Four,
  Three,
  Two,
}

impl Card {
  fn parse(c: char) -> Card {
    match c {
      'A' => Card::Ace,
      'K' => Card::King,
      'Q' => Card::Queen,
      'J' => Card::Jack,
      'T' => Card::Ten,
      '9' => Card::Nine,
      '8' => Card::Eight,
      '7' => Card::Seven,
      '6' => Card::Six,
      '5' => Card::Five,
      '4' => Card::Four,
      '3' => Card::Three,
      '2' => Card::Two,
      x => panic!("Invalid card: {}", x),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
struct Hand(Vec<Card>);

impl Hand {
  fn iter(&self) -> std::slice::Iter<Card> {
    self.0.iter()
  }

  fn contains(&self, card: &Card) -> bool {
    self.0.contains(card)
  }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Hand {
  fn cmp(&self, other: &Self) -> Ordering {
    let self_cards: Vec<Card> = self.0.clone();
    let other_cards: Vec<Card> = other.0.clone();

    for (self_card, other_card) in self_cards.iter().zip(other_cards.iter()) {
      match self_card.cmp(other_card) {
        Ordering::Equal => continue,
        Ordering::Less => return Ordering::Less,
        Ordering::Greater => return Ordering::Greater,
      }
    }

    Ordering::Equal
  }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum HandType {
  FiveOfKind, //AAAAA 6
  FourOfKind, //AA8AA 5
  FullHouse, //23332 4
  ThreeOfKind, //TTT98 3
  TwoPair, //23432 2
  OnePair, //A23A4 1
  HighCard //23456 0
}

impl HandType {
  fn calculate(cards: Vec<Card>) -> HandType {
    let mut cards = cards;

    //enum auto sort by the order we define them (I guess not sure need research)
    cards.sort();

    //Count number of cards for checking which type of hand
    let mut cards_counts: HashMap<Card, u32> = HashMap::new();
    for card in cards {
      let count = cards_counts.entry(card).or_insert(0);
      *count += 1;
    }

    //To vec for sorting by number of cards
    let mut cards_by_number: Vec<(Card, u32)> = cards_counts.into_iter().collect::<Vec<(Card, u32)>>();
    cards_by_number.sort_by(|a, b| b.1.cmp(&a.1));


    match cards_by_number[0].1 {
      5 => HandType::FiveOfKind,
      4 => HandType::FourOfKind,
      3 => {
        if cards_by_number[1].1 == 2 {
          HandType::FullHouse
        } else {
          HandType::ThreeOfKind
        }
      }
      2 => {
        if cards_by_number[1].1 == 2 {
          HandType::TwoPair
        } else {
          HandType::OnePair
        }
      }
      _ => HandType::HighCard,
    }
  }
}

#[derive(Clone, Debug)]
struct Round{
  hand: Hand,
  hand_value: HandType,
  bid: u64
}

impl Round {
  fn create(cards: &Vec<Card>, bid: u64) -> Round {
    Round {
      hand: Hand(cards.clone()),
      hand_value: HandType::calculate(cards.clone()),
      bid,
    }
  }
}

impl PartialEq for Round {
  fn eq(&self, other: &Self) -> bool {
    self.hand_value == other.hand_value && self.hand.iter().all(|c| other.hand.contains(c))
  }
}

impl Eq for Round {}

impl PartialOrd for Round {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Round {
  fn cmp(&self, other: &Self) -> Ordering {
    if self.hand_value == other.hand_value {
      self.hand.cmp(&other.hand)
    } else {
      self.hand_value.cmp(&other.hand_value)
    }
  }
}
fn main() {
  let input = include_str!("./input.txt");
  let output = part1(input);
  dbg!(output);
}

fn part1(input: &str) -> u64{
  let mut rounds: Vec<Round> = Vec::new();

  let data: Vec<&str> = input.split('\n').collect();

  for datum in data {

    let temp: Vec<&str> = datum.split(" ").collect();
    let iter: Vec<char>= temp[0].chars().collect();

    let mut cards: Vec<Card> = Vec::new();

    for value in  iter{
        cards.push(Card::parse(value));
    }
   
    rounds.push(Round::create(&cards, temp[1].parse().expect("Convert bid")));

  }

  rounds.sort();
  
  let mut total: u64 = 0;
  for (index, round) in rounds.iter().rev().enumerate() {
      total = total + (round.bid * (index +1) as u64);
  }
  return total;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = include_str!("./inputTest.txt");
      let result: u64 = part1(input);
      assert_eq!(result, 6440);
  }
}