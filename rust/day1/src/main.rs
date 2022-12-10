use std::{fs};

fn main() {
  let path = "input/sample.txt";
  let input = fs::read_to_string(path)
    .expect(format!("Could not read file `{}`", path).as_str());

  let mut calories: Vec<i64> = vec![];
  let mut calory = 0;

  for line in input.split('\n') {
    if line.is_empty() {
      calories.push(calory);
      calory = 0;
    } else {
      calory += line.parse::<i64>().unwrap();
    }
  }

  let first_max = calories.iter().max().unwrap();
  let second_max = calories.iter()
    .filter(|x| **x != *first_max)
    .into_iter()
    .max()
    .unwrap();

  let third_max = calories.iter()
    .filter(|x| **x != *first_max && **x != *second_max)
    .into_iter()
    .max()
    .unwrap();
    
  let top_three_calories = [
    first_max,
    second_max,
    third_max,
  ];

  let mut sum = 0;

  for calory in top_three_calories {
    sum += calory;
  }

  println!("total: {}", sum);
}
