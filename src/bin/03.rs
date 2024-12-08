use advent_of_code::parse_int;
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(
                |line| match many1(many_till(anychar, parse_mul).map(|(_, valid)| valid))(line) {
                    Ok((_, results)) => results.into_iter().map(|(a, b)| a * b).sum::<usize>(),
                    Err(_) => panic!("Failed to parse line"),
                },
            )
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn parse_mul(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, _) = tag("mul")(input)?;
    let (input, (a, b)) =
        delimited(tag("("), separated_pair(digit1, tag(","), digit1), tag(")"))(input)?;

    Ok((input, (parse_int(a), parse_int(b))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, None);
    }
}
