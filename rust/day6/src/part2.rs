use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
  let window_size = 14;
  let chars = input.chars().collect::<Vec<char>>();
  let first_position = chars.windows(window_size)
    .position(|substr| {
      let set = substr.iter().collect::<HashSet<&char>>();
      return set.len() == substr.len();
    })
    .unwrap();

  return first_position + window_size;
}