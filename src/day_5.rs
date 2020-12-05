#![allow(dead_code)]

struct Index {
  plane_partitionings: [PlanePartitioning; 7],
  row_partitionings: [RowPartitioning; 3],
}
impl Index {
  fn parse(input: &str) -> Self {
    let mut plane_partitionings = [PlanePartitioning::Back; 7];
    let mut chars = input.chars();
    for part in plane_partitionings.iter_mut() {
      *part = match chars.next() {
        Some('F') => PlanePartitioning::Front,
        Some('B') => PlanePartitioning::Back,
        c => panic!("Couldn't get next plane partition? Got: {:?}", c),
      }
    }
    let mut row_partitionings = [RowPartitioning::Left; 3];
    for part in row_partitionings.iter_mut() {
      *part = match chars.next() {
        Some('L') => RowPartitioning::Left,
        Some('R') => RowPartitioning::Right,
        c => panic!("Couldn't get next row partition? Got: {:?}", c),
      }
    }
    Self {
      plane_partitionings,
      row_partitionings,
    }
  }
}

#[derive(Clone, Copy, Debug)]
enum PlanePartitioning {
  Front,
  Back,
}
#[derive(Clone, Copy, Debug)]
enum RowPartitioning {
  Left,
  Right,
}

struct Range {
  low: usize,
  high: usize,
}
impl Range {
  fn midpoint(&self) -> usize {
    ((self.high - self.low) / 2) + self.low
  }
  fn take_upper_half(&mut self) {
    self.low = self.midpoint() + 1;
  }
  fn take_lower_half(&mut self) {
    self.high = self.midpoint();
  }
  fn remaining_value(&self) -> usize {
    if self.low == self.high {
      return self.low;
    }
    panic!(
      "Expected to get down to one value in range, but have {} - {}",
      self.low, self.high
    )
  }
}

struct Plane {
  rows: [Row; 128],
}
impl Plane {
  fn new() -> Self {
    Self {
      rows: [Row::default(); 128],
    }
  }

  fn seat_id(&mut self, index: &Index) -> usize {
    let ((row, col), _seat) = self.get(index);
    (row * 8) + col
  }

  fn get(&mut self, index: &Index) -> ((usize, usize), &mut bool) {
    let (row_idx, row) = self.get_row(index.plane_partitionings.iter().copied());
    let (col_idx, seat) = row.get_seat(index.row_partitionings.iter().copied());
    ((row_idx, col_idx), seat)
  }

  fn get_row(
    &mut self,
    partitionings: impl Iterator<Item = PlanePartitioning>,
  ) -> (usize, &mut Row) {
    let mut range = Range {
      low: 0,
      high: self.rows.len() - 1,
    };
    for partitioning in partitionings {
      match partitioning {
        PlanePartitioning::Front => range.take_lower_half(),
        PlanePartitioning::Back => range.take_upper_half(),
      }
    }
    let idx = range.remaining_value();
    (idx, &mut self.rows[idx])
  }

  fn find_my_seat(&mut self) -> Option<usize> {
    let mut began = false;
    for (row_idx, row) in self.rows.iter().enumerate() {
      for (col_idx, seat) in row.seats.iter().enumerate() {
        if *seat {
          began = true;
        }
        if began && !*seat {
          return Some((row_idx * 8) + col_idx);
        }
      }
    }
    None
  }
}

#[derive(Default, Copy, Clone)]
struct Row {
  seats: [bool; 8],
}
impl Row {
  fn get_seat(
    &mut self,
    partitionings: impl Iterator<Item = RowPartitioning>,
  ) -> (usize, &mut bool) {
    let mut range = Range {
      low: 0,
      high: self.seats.len() - 1,
    };
    for partitioning in partitionings {
      match partitioning {
        RowPartitioning::Left => range.take_lower_half(),
        RowPartitioning::Right => range.take_upper_half(),
      }
    }
    let idx = range.remaining_value();
    (idx, &mut self.seats[idx])
  }
}

fn problem(input: &str) -> usize {
  let mut plane = Plane::new();
  input
    .trim()
    .lines()
    .map(|line| plane.seat_id(&Index::parse(line)))
    .max()
    .unwrap()
}

fn problem_part_2(input: &str) -> usize {
  let mut plane = Plane::new();
  let tickets = input.trim().lines().map(|line| Index::parse(line));
  for ticket in tickets {
    let (_, seat) = plane.get(&ticket);
    *seat = true;
  }
  plane.find_my_seat().unwrap()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn examples() {
    let mut plane = Plane::new();
    assert_eq!(357, plane.seat_id(&Index::parse("FBFBBFFRLR")));
    assert_eq!(567, plane.seat_id(&Index::parse("BFFFBBFRRR")));
    assert_eq!(119, plane.seat_id(&Index::parse("FFFBBBFRRR")));
    assert_eq!(820, plane.seat_id(&Index::parse("BBFFBBFRLL")));
    assert_eq!(
      820,
      problem(
        "
FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL
"
      )
    );
  }

  #[test]
  fn my_input() {
    assert_eq!(930, problem(MY_INPUT));
  }

  #[test]
  fn part_2_my_input() {
    assert_eq!(515, problem_part_2(MY_INPUT));
  }

  static MY_INPUT: &'static str = include_str!("day_5_input.txt");
}
