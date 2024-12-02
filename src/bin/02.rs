use advent_of_code::parse_int;
use rayon::prelude::*;
use smallvec::SmallVec;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .collect::<Vec<_>>()
            .par_iter()
            .filter(|line| {
                let nums = line
                    .split_ascii_whitespace()
                    .map(parse_int)
                    .collect::<SmallVec<[usize; 8]>>();

                test_line(&nums, None)
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .collect::<Vec<_>>()
            .par_iter()
            .filter(|line| {
                let nums = line
                    .split_ascii_whitespace()
                    .map(parse_int)
                    .collect::<SmallVec<[usize; 8]>>();

                if test_line(&nums, None) {
                    return true;
                }

                let mut index = 0;
                let line_length = nums.len();

                while index < line_length {
                    if test_line(&nums, Some(index)) {
                        return true;
                    }

                    index += 1;
                }

                false
            })
            .count(),
    )
}

fn test_line(nums: &SmallVec<[usize; 8]>, skip: Option<usize>) -> bool {
    let mut was_increasing: Option<bool> = None;
    let mut previous_num: Option<usize> = None;

    for (i, curr) in nums.into_iter().enumerate() {
        if Some(i) == skip {
            continue;
        }

        if let Some(prev) = previous_num {
            let diff = prev.abs_diff(*curr);

            if !(1..=3).contains(&diff) {
                return false;
            }

            let is_increasing = Some(*curr > prev);

            if was_increasing.is_some() && was_increasing != is_increasing {
                return false;
            }

            was_increasing = is_increasing;
        }

        previous_num = Some(*curr);
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
