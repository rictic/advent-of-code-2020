use regex::Regex;

struct ConstrainedPassword {
  lower: usize,
  upper: usize,
  char: char,
  password: String,
}
impl ConstrainedPassword {
  fn is_valid(&self) -> bool {
    let times = self.password.chars().filter(|c| *c == self.char).count();
    times >= self.lower && times <= self.upper
  }

  fn is_valid_2(&self) -> bool {
    let b = format!("{}", self.char).as_bytes()[0];
    let bytes: Vec<_> = self.password.bytes().collect();
    let left = bytes[self.lower - 1] == b;
    let right = bytes[self.upper - 1] == b;
    match (left, right) {
      (true, false) | (false, true) => true,
      _ => false,
    }
  }
}

pub fn problem(input: &str) -> Option<usize> {
  let regex = Regex::new(r"(\d+)-(\d+) (.): (.*)").unwrap();

  let list: Vec<_> = input
    .trim()
    .lines()
    .map(|s| {
      let captures = regex.captures(s).unwrap();
      ConstrainedPassword {
        lower: captures[1].parse().unwrap(),
        upper: captures[2].parse().unwrap(),
        char: captures[3].chars().next().unwrap(),
        password: captures[4].to_string(),
      }
    })
    .collect();
  Some(list.iter().filter(|p| p.is_valid()).count())
}

pub fn problem_part_2(input: &str) -> Option<usize> {
  let regex = Regex::new(r"(\d+)-(\d+) (.): (.*)").unwrap();
  let list: Vec<_> = input
    .trim()
    .lines()
    .map(|s| {
      let captures = regex.captures(s).unwrap();
      ConstrainedPassword {
        lower: captures[1].parse().unwrap(),
        upper: captures[2].parse().unwrap(),
        char: captures[3].chars().next().unwrap(),
        password: captures[4].to_string(),
      }
    })
    .collect();
  Some(list.iter().filter(|p| p.is_valid_2()).count())
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn examples() {
    assert_eq!(Some(2), problem(EXAMPLE));
  }

  #[test]
  fn my_input() {
    assert_eq!(Some(422), problem(MY_INPUT));
  }

  #[test]
  fn part_2_examples() {
    assert_eq!(Some(1), problem_part_2(EXAMPLE));
  }

  #[test]
  fn part_2_my_input() {
    assert_eq!(Some(451), problem_part_2(MY_INPUT));
  }

  static EXAMPLE: &'static str = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
  ";
  static MY_INPUT: &'static str = include_str!("day_2_input.txt");
}
