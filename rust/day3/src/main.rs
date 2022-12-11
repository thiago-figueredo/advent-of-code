mod part1;
mod part2;

use std::{fs, env};

fn main() {
  let args: Vec<String> = env::args().skip(2).collect();

  for arg in args.iter() {
    let current_arg = arg.as_str();
    let next_arg = args.iter().skip(1).next().unwrap().as_str();

    match (current_arg, next_arg) {
      ("-part1", input_file) => {
        print!("part1 -> ");
        return part1::main(fs::read_to_string(input_file).unwrap());
      },
      ("-part2", input_file) => {
        print!("part2 -> ");
        return part2::main(fs::read_to_string(input_file).unwrap());
      },
      (_, _) => panic!("Command must be either `-part1` or `-part2`"),
    }
  }
}
