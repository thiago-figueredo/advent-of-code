pub fn main(input: String) {
  println!(
    "number of pairs that the range fully contains the other: {}", 
    input.lines().filter(|line| {
      let (first_pair, second_pair) = line.split_once(',').unwrap();
      let ((a,b), (c,d)) = (first_pair.split_once('-').unwrap(), second_pair.split_once('-').unwrap());
      let ((e,f), (g,h)) = (
        (a.parse::<u8>().unwrap(), b.parse::<u8>().unwrap()), 
        (c.parse::<u8>().unwrap(), d.parse::<u8>().unwrap())
      );

      return e >= g && f <= h || g >= e && h <= f;
    })
    .count()
  );
}