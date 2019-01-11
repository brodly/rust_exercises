use std::collections::HashMap;

pub fn test() {
  let teams = vec![String::from("Blue"), String::from("Yellow")];
  let initial_scores = vec![10, 50];

  for i in &teams {
    println!("{}", i);
  }

  for i in &initial_scores {
    println!("{}", i);
  }

  let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

  for (key, value) in &scores {
    println!("{}: {}", key, value);
  }

  let blue_team = String::from("Blue");
  let score = scores.get(&blue_team);

  println!("{:?}", score)
}