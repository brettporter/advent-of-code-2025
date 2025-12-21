use nom::{
    IResult, Parser,
    character::complete::{newline, one_of, space0, space1, u64},
    multi::{many1, separated_list1},
    sequence::{delimited, pair, terminated},
};

advent_of_code::solution!(6);

#[derive(Debug)]
enum Op {
    Add,
    Multiply,
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Vec<u64>>, Vec<Op>)> {
    let mut parser = pair(
        many1(terminated(
            delimited(space0, separated_list1(space1, u64), space0),
            newline,
        )),
        terminated(
            delimited(
                space0,
                separated_list1(
                    space1,
                    one_of("*+").map(|c| match c {
                        '+' => Op::Add,
                        '*' => Op::Multiply,
                        _ => panic!("Unsupported operation"),
                    }),
                ),
                space0,
            ),
            newline,
        ),
    );

    parser.parse(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, (number_rows, operations)) = parse_input(input).unwrap();

    Some(
        operations
            .iter()
            .enumerate()
            .map(|(idx, op)| {
                number_rows.iter().fold(
                    match op {
                        Op::Add => 0,
                        Op::Multiply => 1,
                    },
                    |acc, e| match op {
                        Op::Add => acc + e[idx],
                        Op::Multiply => acc * e[idx],
                    },
                )
            })
            .sum(),
    )
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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
