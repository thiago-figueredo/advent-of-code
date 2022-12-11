use std::{cmp};

fn intersection(s1: &str, s2: &str) -> Option<String> {
  let (greater, lesser) = if cmp::max(s1.len(), s1.len()) == s1.len() { (s1, s2) } else { (s2, s1) };

  for i in 0..greater.len() {
    let common: String = greater.chars().skip(i).take(1).collect();

    if lesser.contains(common.as_str()) {
      return Some(common);
    }
  }

  return None;
}

fn prioritize(item: String) -> u32 {
  let character = item.chars().nth(0).unwrap();

  if character.is_lowercase() {
    return character as u32 - 'a' as u32 + 1;
  }

  return character as u32 - 'A' as u32 + 27;
}

pub fn main(input: String) {
  let mut priorities_sum = 0;

  for rucksack in input.split("\n") {
    let middle = rucksack.len() / 2;
    let first_compartment = &rucksack[0..middle];
    let second_compartment = &rucksack[middle..];
    let common_item = intersection(first_compartment, second_compartment);

    match common_item {
      Some(item) => priorities_sum += prioritize(item),
      None => {}
    }
  }

  println!("priorities sum: {}", priorities_sum); 
}