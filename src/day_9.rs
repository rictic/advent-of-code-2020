use std::collections::{BTreeSet, VecDeque};

struct Protocol {
  window_size: usize,
  window: VecDeque<i64>,
}

impl Protocol {
  fn new(window_size: usize) -> Self {
    Self {
      window_size,
      window: VecDeque::new(),
    }
  }

  fn is_valid(&mut self, new_number: i64) -> bool {
    if self.window.len() < self.window_size {
      self.window.push_back(new_number);
      return true;
    }
    let mut seeking = BTreeSet::new();
    let mut valid = false;
    for window_num in self.window.iter().copied() {
      let compliment = new_number - window_num;
      if compliment == window_num {
        continue;
      }
      if seeking.contains(&compliment) {
        valid = true;
      }
      seeking.insert(window_num);
    }
    if valid {
      self.window.pop_front();
      self.window.push_back(new_number);
    }
    valid
  }
}

pub fn problem(input: &str, window_size: usize) -> i64 {
  let mut protocol = Protocol::new(window_size);
  input
    .trim()
    .lines()
    .map(|line| line.parse().unwrap())
    .find(|v| !protocol.is_valid(*v))
    .unwrap()
}

pub fn problem_part_2(input: &str, target: i64) -> i64 {
  let nums: Vec<i64> = input
    .trim()
    .lines()
    .map(|line| line.parse().unwrap())
    .collect();
  let mut range: Option<(usize, usize)> = None;
  'outer: for lower_idx in 0..nums.len() {
    for upper_idx in (lower_idx + 1)..nums.len() {
      let sum: i64 = nums[lower_idx..=upper_idx].iter().sum();
      match sum.cmp(&target) {
        std::cmp::Ordering::Less => continue,
        std::cmp::Ordering::Equal => {
          range = Some((lower_idx, upper_idx));
          break 'outer;
        }
        std::cmp::Ordering::Greater => {
          break;
        }
      }
    }
  }
  let (lower, upper) = range.unwrap();
  nums[lower..=upper].iter().min().unwrap() + nums[lower..=upper].iter().max().unwrap()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn examples() {
    assert_eq!(127, problem(EXAMPLES, 5));
  }

  #[test]
  fn my_input() {
    assert_eq!(507622668, problem(MY_INPUT, 25));
  }

  #[test]
  fn examples_part_2() {
    assert_eq!(62, problem_part_2(EXAMPLES, 127));
  }

  #[cfg(not(debug_assertions))]
  #[test]
  fn part_2_my_input() {
    assert_eq!(76688505, problem_part_2(MY_INPUT, 507622668));
  }

  const EXAMPLES: &'static str = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
  ";
  static MY_INPUT: &'static str = include_str!("day_9_input.txt");
}
