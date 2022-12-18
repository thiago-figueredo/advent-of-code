use std::{str::FromStr, collections::BTreeMap};

#[derive(Debug)]
pub struct Move {
  pub count: u32,
  pub from: u32,
  pub to: u32
}

impl Move {
  fn make(count: u32, from: u32, to: u32) -> Self {
    return Self { count, from, to };
  }
}

impl FromStr for Move { type Err = ();
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    return Ok(
      s.split_once(" ")
        .map(|(_, rhs)| {
          let (count, remaining) = rhs.split_once(" ").unwrap();
          let (_, from_rest) = remaining.split_once(" ").unwrap();
          let (from, to_rest) = from_rest.split_once(" ").unwrap();
          let (_, to) = to_rest.split_once(" ").unwrap();

          return Move::make(
            count.parse().unwrap(), 
            from.parse().unwrap(), 
            to.parse().unwrap()
          );
        })
        .unwrap()
    );
  }
}

pub fn get_stack_hash_map(crates_stack_input: &str) -> BTreeMap<u32, Vec<char>> {
  let mut stack_hash_map: BTreeMap<u32, Vec<char>> = BTreeMap::new();
  let mut stacks_numbers: Vec<u32> = vec![];

  crates_stack_input.split("\n")
    .for_each(|line| {
      let parsed_line: Vec<char> = line.chars()
        .enumerate()
        .filter(|(i,c)| i % 4 == 1)
        .map(|(_, c)| c)
        .collect();

      parsed_line.iter().for_each(|c| {
        if c.is_digit(10) { 
          stacks_numbers.push(c.to_digit(10).unwrap());
        }
      });

      parsed_line.iter().enumerate().for_each(|(i, c)| {
        if !c.is_whitespace() && !c.is_digit(10) {
          let index = (i+1) as u32;
          let element = stack_hash_map.get_mut(&index);

          if element.is_some() {
            element.unwrap().insert(0, *c);
          } else {
            stack_hash_map.insert(index, vec![*c]);
          }
        }
      })
    });

  return stack_hash_map;
}
