#![allow(dead_code)]

fn problem(input: &str) -> Option<i64> {
  let mut compliments = std::collections::HashSet::with_capacity(input.len() / 4);
  let list: Vec<i64> = input.trim().lines().map(|s| s.parse().unwrap()).collect();
  for k in list {
    let compliment = 2020 - k;
    compliments.insert(compliment);
    if compliments.contains(&k) {
      return Some(k * compliment);
    }
  }
  None
}

fn problem_part_2(input: &str) -> Option<i64> {
  let vals: Vec<i64> = input.trim().lines().map(|s| s.parse().unwrap()).collect();
  for a in vals.iter() {
    for b in vals.iter() {
      for c in vals.iter() {
        if a != b && b != c && (a + b + c == 2020) {
          return Some(a * b * c);
        }
      }
    }
  }
  None
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn examples() {
    assert_eq!(Some(514579), problem(EXAMPLE));
  }

  #[test]
  fn my_input() {
    assert_eq!(Some(211899), problem(MY_INPUT));
  }

  #[test]
  fn part_2_examples() {
    assert_eq!(Some(241861950), problem_part_2(EXAMPLE));
  }

  #[test]
  fn part_2_my_input() {
    assert_eq!(Some(275765682), problem_part_2(MY_INPUT));
  }

  static EXAMPLE: &'static str = "\
1721
979
366
299
675
1456
  ";
  static MY_INPUT: &'static str = include_str!("day_1_input.txt");
}
