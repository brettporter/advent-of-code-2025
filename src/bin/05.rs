use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{newline, u64},
    combinator::opt,
    multi::many1,
    sequence::{separated_pair, terminated},
};

advent_of_code::solution!(5);

fn parse_input(input: &str) -> IResult<&str, (Vec<(u64, u64)>, Vec<u64>)> {
    let mut parser = separated_pair(
        many1(terminated(separated_pair(u64, tag("-"), u64), newline)),
        newline,
        many1(terminated(u64, opt(newline))),
    );
    parser.parse(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, (fresh, available)) = parse_input(input).unwrap();

    let mut count = 0;

    // naïve version for first go
    for a in available {
        if fresh.iter().any(|&(low, high)| a >= low && a <= high) {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, (fresh, _)) = parse_input(input).unwrap();

    let mut sorted_ranges = fresh.clone();
    sorted_ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut collated_ranges = Vec::new();

    for (low, high) in sorted_ranges {
        if let Some(&(_, prev_high)) = collated_ranges.last() {
            if low > prev_high {
                // no overlap with previous
                collated_ranges.push((low, high));
            } else if high <= prev_high {
                // full overlaps with previous, don't add
            } else {
                // partially overlaps previos, just add difference
                collated_ranges.push((prev_high + 1, high));
            }
        } else {
            // no previous, add this one
            collated_ranges.push((low, high));
        }
    }

    Some(
        collated_ranges
            .iter()
            .map(|(low, high)| high - low + 1)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
