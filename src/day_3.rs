type Point = (usize, usize);

struct Map {
  patterns: Vec<Vec<bool>>,
}
impl Map {
  fn parse(str: &str) -> Self {
    let patterns = str
      .lines()
      .map(|line| {
        line
          .bytes()
          .map(|b| match b {
            b'.' => false,
            b'#' => true,
            _ => panic!("Unknown byte: {}", b),
          })
          .collect()
      })
      .collect();
    Map { patterns }
  }

  fn is_tree(&self, (x, y): Point) -> bool {
    let pattern = &self.patterns[y];
    pattern[x % pattern.len()]
  }

  fn count_trees_along_slope(&self, slope: Point) -> usize {
    let mut count = 0;
    let mut current = (0, 0);
    while current.1 < self.patterns.len() {
      if self.is_tree(current) {
        count += 1;
      }
      current = (current.0 + slope.0, current.1 + slope.1);
    }
    count
  }
}

pub fn problem(input: &str) -> usize {
  Map::parse(input).count_trees_along_slope((3, 1))
}

pub fn problem_part_2(input: &str) -> usize {
  let map = Map::parse(input);
  let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
  slopes
    .iter()
    .map(|slope| map.count_trees_along_slope(*slope))
    .product()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn examples() {
    assert_eq!(7, problem(EXAMPLE));
  }

  #[test]
  fn my_input() {
    assert_eq!(209, problem(MY_INPUT));
  }

  #[test]
  fn part_2_examples() {
    assert_eq!(336, problem_part_2(EXAMPLE));
  }

  #[test]
  fn part_2_my_input() {
    assert_eq!(1574890240, problem_part_2(MY_INPUT));
  }

  static EXAMPLE: &'static str = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#\
  ";
  static MY_INPUT: &'static str = include_str!("day_3_input.txt");
}
