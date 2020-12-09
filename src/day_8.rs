use std::collections::BTreeSet;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Instruction {
  Acc(i64),
  Jmp(i64),
  Nop(i64),
}
impl Instruction {
  fn parse(s: &str) -> Self {
    match &s[0..3] {
      "acc" => Instruction::Acc(s[4..].parse().unwrap()),
      "jmp" => Instruction::Jmp(s[4..].parse().unwrap()),
      "nop" => Instruction::Nop(s[4..].parse().unwrap()),
      _ => panic!("Unknown instruction: {}", s),
    }
  }
}

enum StepStatus {
  Finished,
  Broke,
  InProgress,
}

enum Completion {
  Finished,
  Looped,
  Broke,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Computer {
  instructions: Vec<Instruction>,
  accumulator: i64,
  instruction_pointer: usize,
}
impl Computer {
  fn parse_from_instructions(instructions: &str) -> Self {
    let instructions = instructions
      .lines()
      .map(|line| Instruction::parse(line))
      .collect();
    Self {
      instructions,
      accumulator: 0,
      instruction_pointer: 0,
    }
  }

  fn step(&mut self) -> StepStatus {
    let instruction = self.instructions.get(self.instruction_pointer);
    let instruction = match instruction {
      Some(s) => *s,
      None => {
        if self.instruction_pointer == self.instructions.len() {
          return StepStatus::Finished;
        }
        return StepStatus::Broke;
      }
    };
    match instruction {
      Instruction::Acc(v) => {
        self.accumulator += v;
        self.instruction_pointer += 1;
      }
      Instruction::Jmp(v) => {
        self.instruction_pointer = (self.instruction_pointer as i64 + v) as usize;
      }
      Instruction::Nop(_) => {
        self.instruction_pointer += 1;
      }
    }
    StepStatus::InProgress // not done yet!
  }

  fn simple_infinite_loop_detector(&mut self) -> Completion {
    let mut visited = BTreeSet::new();
    while !visited.contains(&self.instruction_pointer) {
      visited.insert(self.instruction_pointer);
      match self.step() {
        StepStatus::Finished => {
          return Completion::Finished;
        }
        StepStatus::Broke => {
          return Completion::Broke;
        }
        StepStatus::InProgress => continue,
      }
    }
    return Completion::Looped;
  }
}

pub fn problem(input: &str) -> i64 {
  let mut computer = Computer::parse_from_instructions(input.trim());
  computer.simple_infinite_loop_detector();
  computer.accumulator
}

pub fn problem_part_2(input: &str) -> Option<i64> {
  let base_instructions = Computer::parse_from_instructions(input.trim()).instructions;
  for idx in 0..base_instructions.len() {
    let mut instructions = base_instructions.clone();
    match instructions.get(idx) {
      Some(Instruction::Nop(v)) => {
        instructions[idx] = Instruction::Jmp(*v);
      }
      Some(Instruction::Jmp(v)) => {
        instructions[idx] = Instruction::Nop(*v);
      }
      _ => {
        continue;
      }
    }
    let mut computer = Computer {
      instructions,
      accumulator: 0,
      instruction_pointer: 0,
    };
    match computer.simple_infinite_loop_detector() {
      Completion::Finished => {
        return Some(computer.accumulator);
      }
      Completion::Looped | Completion::Broke => {
        continue;
      }
    }
  }
  return None;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn examples() {
    assert_eq!(5, problem(EXAMPLES));
  }

  #[test]
  fn my_input() {
    assert_eq!(1501, problem(MY_INPUT));
  }

  #[test]
  fn examples_part_2() {
    assert_eq!(Some(8), problem_part_2(EXAMPLES))
  }

  #[test]
  fn part_2_my_input() {
    assert_eq!(Some(509), problem_part_2(MY_INPUT))
  }

  const EXAMPLES: &'static str = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
  ";
  static MY_INPUT: &'static str = include_str!("day_8_input.txt");
}
