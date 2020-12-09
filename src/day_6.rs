use std::collections::BTreeSet;

pub fn problem(input: &str) -> usize {
  input
    .split("\n\n")
    .map(|group| {
      let mut map = BTreeSet::new();
      for line in group.lines() {
        for char in line.chars() {
          map.insert(char);
        }
      }
      map.len()
    })
    .sum()
}

pub fn problem_part_2(input: &str) -> usize {
  input
    .trim()
    .split("\n\n")
    .map(|group| {
      let mut outer = None;
      for line in group.lines() {
        let map: BTreeSet<_> = line.chars().collect();
        match outer {
          None => outer = Some(map),
          Some(m) => outer = Some(m.intersection(&map).map(|c| *c).collect()),
        }
      }
      let outer = outer.unwrap();
      println!(
        "in: \n{:?}\n, {} answers were in common",
        group,
        outer.len()
      );
      outer.len()
    })
    .sum()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn examples() {
    assert_eq!(
      11,
      problem(
        "
abc

a
b
c

ab
ac

a
a
a
a

b\
        "
      )
    );
  }

  #[test]
  fn my_input() {
    assert_eq!(6521, problem(MY_INPUT));
  }

  #[test]
  fn examples_part_2() {
    assert_eq!(
      6,
      problem_part_2(
        "
abc

a
b
c

ab
ac

a
a
a
a

b\
        "
      )
    );
  }

  #[test]
  fn part_2_my_input() {
    assert_eq!(3305, problem_part_2(MY_INPUT));
  }

  static MY_INPUT: &'static str = include_str!("day_6_input.txt");
}
