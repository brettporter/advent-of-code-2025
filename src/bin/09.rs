use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{i64, newline},
    multi::many1,
    sequence::{separated_pair, terminated},
};

advent_of_code::solution!(9);

fn parse_input(input: &str) -> IResult<&str, Vec<(i64, i64)>> {
    let mut parser = many1(terminated(separated_pair(i64, tag(","), i64), newline));
    parser.parse(input)
}

pub fn part_one(input: &str) -> Option<i64> {
    let (_, tiles) = parse_input(input).unwrap();

    tiles
        .iter()
        .combinations(2)
        .map(|v| {
            let (x1, y1) = v[0];
            let (x2, y2) = v[1];
            let h = (y2 - y1).abs() + 1;
            let w = (x2 - x1).abs() + 1;
            w * h
        })
        .max()
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
