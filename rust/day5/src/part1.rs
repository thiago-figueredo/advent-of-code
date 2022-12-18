#[path ="./helper.rs"]
mod helper;

use helper::get_stack_hash_map;
use helper::Move;

pub fn solve(input: String) -> String {
  let (crates_stack_input, steps) = input.split_once("\n\n").unwrap();
  let mut stack_hash_map = get_stack_hash_map(crates_stack_input);
  let mut stacks_top = String::from("");

  steps.split("\n")
    .filter(|line| !line.is_empty())
    .for_each(|line| {
      let step = line.parse::<Move>().unwrap();
      let stacks_source: Vec<char> = stack_hash_map.get(&step.from)
        .unwrap()
        .into_iter()
        .rev()
        .map(|c| *c)
        .take(step.count as usize)
        .collect();

      stack_hash_map.get_mut(&step.to)
        .unwrap()
        .extend(stacks_source.clone());

      for _ in 0..step.count {
        stack_hash_map.get_mut(&step.from).unwrap().pop();
      }
    });

  stack_hash_map.iter()
    .for_each(|(_, stacks)| stacks_top.push(*stacks.last().unwrap()));

  return stacks_top;
}