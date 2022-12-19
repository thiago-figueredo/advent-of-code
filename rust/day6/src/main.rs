mod part1;
mod part2;

use std::fs;

mod tests {
  use super::*;

  pub fn part1_test_with_small_input() {
    assert_eq!(part1::solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
  }
}

fn main() {
  tests::part1_test_with_small_input();
  println!("part1 solution: {}", part1::solve(fs::read_to_string("./src/puzzle.txt").unwrap().as_str()));

  println!("part2 solution: {}", part2::solve(fs::read_to_string("./src/puzzle2.txt").unwrap().as_str()));
}
