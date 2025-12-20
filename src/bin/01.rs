use nom::{
    IResult, Parser,
    character::complete::{newline, one_of, u32},
    combinator::opt,
    multi::many1,
    sequence::{pair, terminated},
};

advent_of_code::solution!(1);

fn parse_input(input: &str) -> IResult<&str, Vec<(char, u32)>> {
    let mut parser = many1(terminated(pair(one_of("LR"), u32), opt(newline)));
    parser.parse(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, values) = parse_input(input).unwrap();

    let mut zeros = 0;
    let mut position = 50;

    for (direction, distance) in values {
        match direction {
            'L' => position = (position + 100 - distance % 100) % 100,
            'R' => position = (position + distance) % 100,
            _ => unreachable!(),
        }
        if position == 0 {
            zeros += 1;
        }
    }

    Some(zeros)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, values) = parse_input(input).unwrap();

    let mut zeros = 0;
    let mut position = 50;
    let mut previous_position = position;

    for (direction, distance) in values {
        if distance > 100 {
            let full_cycles = distance / 100;
            zeros += full_cycles as u64;
        }
        let distance = distance % 100;

        match direction {
            'L' => position = position - distance as i32,
            'R' => position = position + distance as i32,
            _ => unreachable!(),
        }

        if position == 0 {
            zeros += 1;
        }

        if position < 0 {
            position += 100;
            if previous_position > 0 {
                zeros += 1;
            }
        }

        if position >= 100 {
            position -= 100;
            zeros += 1;
        }
        previous_position = position;
    }

    Some(zeros)
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
        assert_eq!(result, Some(6));
    }
}
