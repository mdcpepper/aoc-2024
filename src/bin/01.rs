use advent_of_code::parse_int;
use itertools::Itertools;
use rustc_hash::{FxBuildHasher, FxHashMap};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<usize> {
    let (left, right) = split_input(input);

    Some(
        left.into_iter()
            .sorted_unstable()
            .zip(right.into_iter().sorted_unstable())
            .map(|(left, right)| left.abs_diff(right))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (left, right) = split_input(input);

    let counts = right.into_iter().fold(
        FxHashMap::with_capacity_and_hasher(left.len(), FxBuildHasher),
        |mut counts, right| {
            *counts.entry(right).or_insert(0) += 1;
            counts
        },
    );

    Some(left.into_iter().fold(0, |score, left| {
        score + left * counts.get(&left).unwrap_or(&0)
    }))
}

#[inline(always)]
fn split_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .map(|line| {
            let mut nums = line.split_ascii_whitespace().map(parse_int);
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
