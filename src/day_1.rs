#![allow(dead_code)]

fn multiply_those_that_sum_to(list: &Vec<i64>, target: i64) -> Option<i64> {
  let mut compliments = std::collections::HashSet::with_capacity(list.len());
  for k in list {
    let compliment = target - k;
    compliments.insert(compliment);
    if compliments.contains(&k) {
      return Some(k * compliment);
    }
  }
  None
}

fn problem(input: &str) -> Option<i64> {
  let list = &input.trim().lines().map(|s| s.parse().unwrap()).collect();
  multiply_those_that_sum_to(list, 2020)
}

fn problem_part_2(input: &str) -> Option<i64> {
  let list: Vec<i64> = input.trim().lines().map(|s| s.parse().unwrap()).collect();
  for a in list.iter() {
    if let Some(result) = multiply_those_that_sum_to(&list, 2020 - a) {
      return Some(a * result);
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
