fn get_move_value(choice: &str) -> u64 {
  return match choice {
    "X" => 1,
    "Y" => 2,
    "Z" => 3,
    _ => unreachable!()
  };
}

fn my_score(my_move: &str, opponent_move: &str) -> u64 {
  let my_move_value = get_move_value(my_move);

  return match (my_move, opponent_move) {
    ("X", "A") => get_move_value("Z"),
    ("X", "B") => get_move_value("X"),
    ("X", "C") => get_move_value("Y"),
    ("Y", "A") => 3 + get_move_value("X"),
    ("Y", "B") => 3 + get_move_value("Y"),
    ("Y", "C") => 3 + get_move_value("Z"),
    ("Z", "A") => 6 + get_move_value("Y"),
    ("Z", "B") => 6 + get_move_value("Z"),
    ("Z", "C") => 6 + get_move_value("X"),
    (_, _) => unreachable!()
  };
}

pub fn main(game_input: String) {
  let mut total_score = 0;

  for round in game_input.split("\n") {
    let round_string: Vec<&str> = round.split(" ").collect();

    if round_string.len() == 2 {
      let opponent_move = round_string[0];
      let my_move = round_string[1];

      total_score += my_score(my_move, opponent_move);
    }
  }

  println!("my to total score is: {}", total_score);
}
