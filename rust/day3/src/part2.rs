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
  let splitted_input: Vec<&str> = input.split("\n").collect();
  let mut i = 0;

  while i <= splitted_input.len() - 3 {
    if !splitted_input[i].is_empty() {
      let rucksacks = &splitted_input.as_slice()[i..i+3];
      let first_rucksack = rucksacks[0];

      for item in first_rucksack.chars() {
        if rucksacks[1].contains(item) && rucksacks[2].contains(item) {
          priorities_sum += prioritize(item.to_string());
          break;
        }
      }
    }

    i += 3;
  }

  println!("priorities sum: {}", priorities_sum); 
}