use std::collections::HashSet;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut numbers: Vec<u32> = input
        .lines()
        .filter_map(|line| line.trim().parse::<u32>().ok())
        .collect();

    numbers.sort();

    let mut left = 0;
    let mut right = numbers.len() - 1;

    while left < right {
        let left_val = numbers[left];
        let right_val = numbers[right];

        let sum = left_val + right_val;

        match sum.cmp(&2020) {
            std::cmp::Ordering::Greater => {
                right -= 1; // Decrease the sum by moving the right pointer
            }
            std::cmp::Ordering::Less => {
                left += 1; // Increase the sum by moving the left pointer
            }
            std::cmp::Ordering::Equal => {
                return Some(left_val * right_val); // Found the pair
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    // Store the numbers into a hashset for quick lookup
    let numbers: HashSet<u32> = input
        .lines()
        .filter_map(|line| line.trim().parse::<u32>().ok())
        .collect();

    for number in &numbers {
        let target: u32 = 2020 - number;
        for number2 in &numbers {
            if number != number2 {
                if *number2 < target {
                    let target2 = target - number2;
                    if numbers.contains(&target2) {
                        return Some(number2 * number * target2);
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(866436));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(276650720));
    }
}
