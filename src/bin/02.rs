use std::cmp::Ordering;

use advent_of_code::parse_int;

advent_of_code::solution!(2);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    Increasing,
    Decreasing,
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter(|line| {
                let nums = line
                    .split_ascii_whitespace()
                    .map(parse_int)
                    .collect::<Vec<_>>();

                test_line(nums)
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter(|line| {
                let nums = line
                    .split_ascii_whitespace()
                    .map(parse_int)
                    .collect::<Vec<_>>();

                if test_line(nums.clone()) {
                    return true;
                }

                let line_length = nums.len();
                let mut index = 0;

                while index < line_length {
                    let mut new_nums = nums.clone();
                    new_nums.remove(index);

                    if test_line(new_nums) {
                        return true;
                    }

                    index += 1;
                }

                false
            })
            .count(),
    )
}

fn test_line(nums: Vec<usize>) -> bool {
    let mut previous_dir: Option<Direction> = None;

    for window in nums.windows(2) {
        let diff = window[0].abs_diff(window[1]);

        if !(1..=3).contains(&diff) {
            return false;
        }

        let current_dir = match window[0].cmp(&window[1]) {
            Ordering::Less => Some(Direction::Increasing),
            Ordering::Greater => Some(Direction::Decreasing),
            _ => None,
        };

        if previous_dir.is_some() && previous_dir != current_dir {
            return false;
        }

        previous_dir = current_dir;
    }

    true
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
        assert_eq!(result, Some(4));
    }
}
