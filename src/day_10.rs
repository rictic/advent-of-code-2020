pub fn problem(input: &str) -> u64 {
  let mut nums: Vec<u64> = input
    .trim()
    .lines()
    .map(|line| line.parse().unwrap())
    .collect();
  nums.sort();
  let mut one_diffs: u64 = 0;
  let mut three_diffs: u64 = 1; // one to account for the final diff
  let mut current_jolts = 0;
  for num in nums {
    let diff = num - current_jolts;
    if diff == 1 {
      one_diffs += 1
    } else if diff == 3 {
      three_diffs += 1;
    } else if diff > 3 {
      panic!(
        "Impossible: got too big of a difference: {} between {} and {}",
        diff, current_jolts, num,
      );
    }
    current_jolts = num;
  }
  one_diffs * three_diffs
}

pub fn problem_part_2(input: &str) -> u64 {
  let mut nums: Vec<u64> = input
    .trim()
    .lines()
    .map(|line| line.parse().unwrap())
    .collect();
  nums.sort();

  let reversed_numbers = nums.into_iter().rev().chain([0].iter().copied());
  reversed_numbers
    .fold((None, None, None), |prevs, num| {
      let mut solutions_from_here = 0;
      if let Some((val, count)) = prevs.0 {
        if val - num <= 3 {
          solutions_from_here += count;
        }
      } else {
        solutions_from_here += 1;
      }
      if let Some((val, count)) = prevs.1 {
        if val - num <= 3 {
          solutions_from_here += count;
        }
      }
      if let Some((val, count)) = prevs.2 {
        if val - num <= 3 {
          solutions_from_here += count;
        }
      }
      let result = (Some((num, solutions_from_here)), prevs.0, prevs.1);
      result
    })
    .0
    .unwrap()
    .1
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn examples() {
    assert_eq!(7 * 5, problem(EXAMPLE));
    assert_eq!(22 * 10, problem(LONGER_EXAMPLE));
  }

  #[test]
  fn my_input() {
    assert_eq!(1690, problem(MY_INPUT));
  }

  #[test]
  fn examples_part_2() {
    assert_eq!(8, problem_part_2(EXAMPLE));
    assert_eq!(19208, problem_part_2(LONGER_EXAMPLE));
  }

  #[test]
  fn part_2_my_input() {
    assert_eq!(5289227976704, problem_part_2(MY_INPUT));
  }

  const EXAMPLE: &'static str = "\
16
10
15
5
1
11
7
19
6
12
4
  ";
  const LONGER_EXAMPLE: &'static str = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
";
  static MY_INPUT: &'static str = include_str!("day_10_input.txt");
}
