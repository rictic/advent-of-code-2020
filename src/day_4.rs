#![allow(dead_code)]

use enum_map::EnumMap;
use PassportField::*;

#[derive(enum_map::Enum, Copy, Clone, PartialEq, Eq, Debug)]
enum PassportField {
  BirthYear,
  IssueYear,
  ExpirationYear,
  Height,
  HairColor,
  EyeColor,
  PassportId,
  CountryId,
}
impl PassportField {
  fn validate(&self, value: Option<&str>) -> bool {
    match (self, value) {
      (CountryId, _) => true,
      (_, None) => false,
      (_, Some(s)) => self.validate_value(s),
    }
  }

  fn validate_value(&self, value: &str) -> bool {
    match self {
      BirthYear => value
        .parse()
        .map(|v: u16| v >= 1920 && v <= 2002)
        .unwrap_or(false),
      IssueYear => value
        .parse()
        .map(|v: u16| v >= 2010 && v <= 2020)
        .unwrap_or(false),
      ExpirationYear => value
        .parse()
        .map(|v: u16| v >= 2020 && v <= 2030)
        .unwrap_or(false),
      Height => {
        let in_cm = match &value[value.len() - 2..] {
          "cm" => true,
          "in" => false,
          _ => return false,
        };

        value[0..value.len() - 2]
          .parse()
          .map(|v: u16| {
            if in_cm {
              v >= 150 && v <= 193
            } else {
              v >= 59 && v <= 76
            }
          })
          .unwrap_or(false)
      }
      HairColor => {
        if value.len() != 7 {
          return false;
        }
        let mut chars = value.chars();
        if Some('#') != chars.next() {
          return false;
        }
        chars.all(|c| c.is_digit(16))
      }
      EyeColor => match value {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
      },
      PassportId => value.len() == 9 && value.chars().all(|c| c.is_ascii_digit()),
      CountryId => true,
    }
  }
}
const PASSPORT_FIELDS: [PassportField; 8] = [
  BirthYear,
  IssueYear,
  ExpirationYear,
  Height,
  HairColor,
  EyeColor,
  PassportId,
  CountryId,
];

struct PassportCandidate<'a> {
  fields: EnumMap<PassportField, Option<&'a str>>,
}
impl<'a> PassportCandidate<'a> {
  fn parse(str: &'a str) -> Self {
    let mut fields = EnumMap::new();
    let key_value_pairs = str.split(|c| c == ' ' || c == '\n').map(|key_value| {
      let key = match &key_value[0..4] {
        "byr:" => BirthYear,
        "iyr:" => IssueYear,
        "eyr:" => ExpirationYear,
        "hgt:" => Height,
        "hcl:" => HairColor,
        "ecl:" => EyeColor,
        "pid:" => PassportId,
        "cid:" => CountryId,
        unkn => panic!("Unknown field: {}", unkn),
      };
      (key, &key_value[4..])
    });
    for (key, value) in key_value_pairs {
      fields[key] = Some(value);
    }
    Self { fields }
  }

  fn is_valid(&self) -> bool {
    for field in PASSPORT_FIELDS.iter() {
      if let CountryId = field {
        continue;
      }
      if self.fields[*field].is_none() {
        return false;
      }
    }
    return true;
  }

  fn extended_validation(&self) -> bool {
    PASSPORT_FIELDS.iter().all(|field| {
      let result = field.validate(self.fields[*field]);
      if !result {
        println!("{:?} is an invalid {:?}", field, self.fields[*field]);
      }
      result
    })
  }
}

fn problem(input: &str) -> usize {
  input
    .trim_end()
    .split("\n\n")
    .map(|c| PassportCandidate::parse(c))
    .filter(|pc| pc.is_valid())
    .count()
}

fn problem_part_2(input: &str) -> usize {
  input
    .trim_end()
    .split("\n\n")
    .map(|c| PassportCandidate::parse(c))
    .filter(|pc| pc.extended_validation())
    .count()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn examples() {
    assert_eq!(2, problem(EXAMPLE));
  }

  #[test]
  fn my_input() {
    assert_eq!(247, problem(MY_INPUT));
  }

  #[test]
  fn part_2_negative_examples() {
    assert_eq!(
      0,
      problem_part_2(
        "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007\
        "
      )
    );
  }

  #[test]
  fn part_2_positive_examples() {
    assert_eq!(
      4,
      problem_part_2(
        "\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719\
        "
      )
    );
  }

  #[test]
  fn part_2_my_input() {
    assert_eq!(145, problem_part_2(MY_INPUT));
  }

  static EXAMPLE: &'static str = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in\
  ";
  static MY_INPUT: &'static str = include_str!("day_4_input.txt");
}
