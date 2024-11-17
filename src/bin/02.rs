use de_regex;
use serde::Deserialize;
advent_of_code::solution!(2);

const PATTERN: &str = r"^(?P<min>\d+)-(?P<max>\d+) (?P<letter>[a-z]): (?P<password>.+)$";

#[derive(Debug, Deserialize)]
struct PasswordRule {
    min: u32,
    max: u32,
    letter: char,
    password: String,
}

fn parse_line(line: &str) -> PasswordRule {
    de_regex::from_str(line, PATTERN).unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let rules: Vec<PasswordRule> = input.lines().map(|line| parse_line(line)).collect();

    let mut total = 0;
    for rule in &rules {
        let count = rule.password.chars().filter(|&c| c == rule.letter).count() as u32;
        if (count <= rule.max) && (count >= rule.min) {
            total += 1;
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rules: Vec<PasswordRule> = input.lines().map(|line| parse_line(line)).collect();

    let mut total = 0;
    for rule in &rules {
        let a = rule.password.chars().nth((rule.min - 1) as usize).unwrap();
        let b = rule.password.chars().nth((rule.max - 1) as usize).unwrap();

        if (a == rule.letter) ^ (b == rule.letter) {
            total += 1
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
