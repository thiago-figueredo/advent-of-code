mod part1;
mod part2;

use std::fs;

mod tests {
  use super::*;

  pub fn part1_test_with_small_input() {
    assert_eq!(part1::solve(fs::read_to_string("./src/test.txt").unwrap()), "CMZ");
  }

  pub fn part2_test_with_small_input() {
    assert_eq!(part2::solve(fs::read_to_string("./src/test.txt").unwrap()), "MCD");
  }
}

fn main() {
  tests::part1_test_with_small_input();
  println!("part1 solution: {}", part1::solve(fs::read_to_string("./src/puzzle.txt").unwrap()));

  tests::part2_test_with_small_input();
  println!("part1 solution: {}", part2::solve(fs::read_to_string("./src/puzzle.txt").unwrap()));
}
