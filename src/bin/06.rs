use std::cmp::min;

use nom::{
    IResult, Parser,
    character::complete::{newline, one_of},
    multi::many1,
    sequence::{pair, terminated},
};

advent_of_code::solution!(6);

#[derive(Debug)]
enum Op {
    Add,
    Multiply,
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Vec<char>>, Vec<char>)> {
    let mut parser = pair(
        many1(terminated(many1(one_of(" 0123456789")), newline)),
        terminated(many1(one_of(" *+")), newline),
    );

    parser.parse(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, (number_rows, operations)) = parse_input(input).unwrap();

    let number_rows: Vec<Vec<u64>> = number_rows
        .iter()
        .map(|r| {
            let s: String = r.iter().collect();
            s.split_ascii_whitespace()
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect();

    let s: String = operations.iter().collect();
    let operations: Vec<Op> = s
        .split_ascii_whitespace()
        .map(|op| match op {
            "+" => Op::Add,
            "*" => Op::Multiply,
            _ => unimplemented!(),
        })
        .collect();

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
                    |acc, e| {
                        let v = e[idx];
                        match op {
                            Op::Add => acc + v,
                            Op::Multiply => acc * v,
                        }
                    },
                )
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, (number_rows, operations)) = parse_input(input).unwrap();

    let mut total = 0;
    let line_len = number_rows.iter().map(|x| x.len()).max().unwrap();

    let indexed_ops: Vec<(Op, usize)> = operations
        .iter()
        .enumerate()
        .filter_map(|(i, op)| match op {
            '+' => Some((Op::Add, i)),
            '*' => Some((Op::Multiply, i)),
            _ => None,
        })
        .collect();

    for (op, start_idx) in indexed_ops {
        let width = operations[start_idx + 1..]
            .iter()
            .position(|&x| x != ' ')
            .unwrap_or(line_len - start_idx + 1);

        let nums: Vec<&[char]> = number_rows
            .iter()
            .map(|r| {
                let end_idx = min(r.len(), start_idx + width);
                &r[start_idx..end_idx]
            })
            .collect();
        let mut result = match op {
            Op::Add => 0,
            Op::Multiply => 1,
        };
        for i in 0..width {
            let num = nums.iter().fold(0, |acc, n| {
                let d = n.get(i).unwrap_or(&' ');
                match d {
                    ' ' => acc,
                    _ => acc * 10 + d.to_digit(10).unwrap() as u64,
                }
            });
            result = match op {
                Op::Add => result + num,
                Op::Multiply => result * num,
            };
        }
        total += result;
    }

    Some(total)
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
        assert_eq!(result, Some(3263827));
    }
}
