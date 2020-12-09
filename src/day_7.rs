use regex::Regex;
use Adjective::*;
use Hue::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Hue {
  Aqua,
  Beige,
  Black,
  Blue,
  Bronze,
  Brown,
  Chartreuse,
  Coral,
  Crimson,
  Cyan,
  Fuchsia,
  Gold,
  Gray,
  Green,
  Indigo,
  Lavender,
  Lime,
  Magenta,
  Maroon,
  Olive,
  Orange,
  Plum,
  Purple,
  Red,
  Salmon,
  Silver,
  Tan,
  Teal,
  Tomato,
  Turquoise,
  Violet,
  White,
  Yellow,
}
impl Hue {
  fn parse(s: &str) -> Self {
    match s {
      "aqua" => Aqua,
      "beige" => Beige,
      "black" => Black,
      "blue" => Blue,
      "bronze" => Bronze,
      "brown" => Brown,
      "chartreuse" => Chartreuse,
      "coral" => Coral,
      "crimson" => Crimson,
      "cyan" => Cyan,
      "fuchsia" => Fuchsia,
      "gold" => Gold,
      "gray" => Gray,
      "green" => Green,
      "indigo" => Indigo,
      "lavender" => Lavender,
      "lime" => Lime,
      "magenta" => Magenta,
      "maroon" => Maroon,
      "olive" => Olive,
      "orange" => Orange,
      "plum" => Plum,
      "purple" => Purple,
      "red" => Red,
      "salmon" => Salmon,
      "silver" => Silver,
      "tan" => Tan,
      "teal" => Teal,
      "tomato" => Tomato,
      "turquoise" => Turquoise,
      "violet" => Violet,
      "white" => White,
      "yellow" => Yellow,
      _ => panic!("Unknown color hue: {}", s),
    }
  }
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Adjective {
  Bright,
  Clear,
  Dark,
  Dim,
  Dotted,
  Drab,
  Dull,
  Faded,
  Light,
  Mirrored,
  Muted,
  Pale,
  Plaid,
  Posh,
  Shiny,
  Striped,
  Vibrant,
  Wavy,
}
impl Adjective {
  fn parse(s: &str) -> Self {
    match s {
      "bright" => Bright,
      "clear" => Clear,
      "dark" => Dark,
      "dim" => Dim,
      "dotted" => Dotted,
      "drab" => Drab,
      "dull" => Dull,
      "faded" => Faded,
      "light" => Light,
      "mirrored" => Mirrored,
      "muted" => Muted,
      "pale" => Pale,
      "plaid" => Plaid,
      "posh" => Posh,
      "shiny" => Shiny,
      "striped" => Striped,
      "vibrant" => Vibrant,
      "wavy" => Wavy,
      _ => panic!("Unknown color adjective: {}", s),
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Color {
  adjective: Adjective,
  hue: Hue,
}
impl Color {
  fn parse(text: &str) -> Self {
    let idx = text.find(' ').unwrap();
    let adjective = Adjective::parse(&text[0..idx]);
    let hue = Hue::parse(&text[idx + 1..]);
    Self { adjective, hue }
  }
}

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
struct Rule {
  container: Color,
  contents: Vec<(usize, Color)>,
}
impl Rule {
  fn parse(line: &str) -> Self {
    lazy_static! {
      static ref CONTAINER_RE: Regex = Regex::new(r"(.*?) bags contain ").unwrap();
      static ref CONTAINED_RE: Regex = Regex::new(r"(\d+) (.*?) bag").unwrap();
    }
    let capture = CONTAINER_RE.captures(line).unwrap();
    let container = Color::parse(&capture[1]);
    let contents = CONTAINED_RE
      .captures_iter(line)
      .map(|capture| (capture[1].parse().unwrap(), Color::parse(&capture[2])))
      .collect();
    Rule {
      container,
      contents,
    }
  }
}
#[derive(Debug)]
struct Ruleset {
  rules: Vec<Rule>,
  contained: BTreeMap<Color, BTreeSet<Color>>,
  contents: BTreeMap<Color, Vec<(usize, Color)>>,
}

impl Ruleset {
  fn new(rules: Vec<Rule>) -> Self {
    let mut contained: BTreeMap<Color, BTreeSet<Color>> = BTreeMap::new();
    for rule in rules.iter() {
      for (_num, color) in rule.contents.iter() {
        let set = contained.entry(*color).or_default();
        set.insert(rule.container);
      }
    }
    let mut contents: BTreeMap<Color, Vec<(usize, Color)>> = BTreeMap::new();
    for rule in rules.iter() {
      contents.insert(rule.container, rule.contents.clone());
    }
    Self {
      rules,
      contained,
      contents,
    }
  }

  fn toplevel_bag_options(&self, target: Color) -> BTreeSet<Color> {
    let mut set = BTreeSet::new();
    let mut current = vec![target];
    while let Some(color) = current.pop() {
      if color != target {
        set.insert(color);
      }
      let containers = match self.contained.get(&color) {
        Some(c) => c,
        None => {
          continue;
        }
      };
      for c in containers.iter() {
        current.push(*c);
      }
    }
    set
  }

  fn nested_contents_inner(&self, target: Color, cache: &mut BTreeMap<Color, usize>) -> usize {
    if let Some(v) = cache.get(&target) {
      return *v;
    }

    let mut count = 0;
    let contained_bags = match self.contents.get(&target) {
      Some(c) => c,
      None => {
        return 1;
      }
    };
    for (num, color) in contained_bags.iter() {
      count += (1 + self.nested_contents_inner(*color, cache)) * num;
    }
    cache.insert(target, count);
    count
  }

  fn nested_contents_count(self, target: Color) -> usize {
    self.nested_contents_inner(target, &mut BTreeMap::new())
  }
}

pub fn problem(input: &str) -> usize {
  let rules: Vec<_> = input.lines().map(|line| Rule::parse(line)).collect();
  let ruleset = Ruleset::new(rules);
  let toplevel_options = ruleset.toplevel_bag_options(Color::parse("shiny gold"));
  toplevel_options.len()
}

pub fn problem_part_2(input: &str) -> usize {
  let rules: Vec<_> = input.lines().map(|line| Rule::parse(line)).collect();
  let ruleset = Ruleset::new(rules);
  ruleset.nested_contents_count(Color::parse("shiny gold"))
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn examples() {
    assert_eq!(4, problem(EXAMPLES));
  }

  #[test]
  fn my_input() {
    assert_eq!(274, problem(MY_INPUT));
  }

  #[test]
  fn examples_part_2() {
    assert_eq!(32, problem_part_2(EXAMPLES))
  }

  #[test]
  fn part_2_my_input() {
    assert_eq!(158730, problem_part_2(MY_INPUT))
  }

  const EXAMPLES: &'static str = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.\
  ";
  static MY_INPUT: &'static str = include_str!("day_7_input.txt");
}
