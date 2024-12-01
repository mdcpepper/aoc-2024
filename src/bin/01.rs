use itertools::Itertools;
use rustc_hash::{FxBuildHasher, FxHashMap};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<usize> {
    let (left, right) = split_input(input);

    Some(
        left.into_iter()
            .sorted_unstable()
            .zip(right.into_iter().sorted_unstable())
            .map(|(l, r)| l.abs_diff(r))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (left, right) = split_input(input);

    let counts = right.into_iter().fold(
        FxHashMap::with_capacity_and_hasher(left.len(), FxBuildHasher),
        |mut counts, r| {
            *counts.entry(r).or_insert(0) += 1;
            counts
        },
    );

    Some(
        left.into_iter()
            .fold(0, |score, l| score + l * counts.get(&l).unwrap_or(&0)),
    )
}

fn split_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .map(|line| {
            let mut nums = line.split_ascii_whitespace().map(parse_int);
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip()
}

// https://rust-malaysia.github.io/code/2020/07/11/faster-integer-parsing.html
pub fn parse_int(s: &str) -> usize {
    s.bytes().fold(0, |a, c| a * 10 + (c & 0x0f) as usize)
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
