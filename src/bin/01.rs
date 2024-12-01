advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i64> {
    let (mut left, mut right) = split_input(input);

    left.sort();
    right.sort();

    Some(
        left.iter()
            .zip(right.iter())
            .map(|(l, r)| (r - l).abs())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let (left, right) = split_input(input);

    let mut score = 0;

    for l in left {
        score += l * right.iter().filter(|&r| r == &l).count() as i64;
    }

    Some(score)
}

fn split_input(input: &str) -> (Vec<i64>, Vec<i64>) {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0], v[1]))
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
