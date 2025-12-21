use nom::{
    IResult, Parser,
    character::complete::{digit1, newline},
    combinator::opt,
    multi::many1,
    sequence::terminated,
};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let (_, banks) = parse_input(input).unwrap();

    let mut total = 0;

    for bank in banks {
        let b = bank.as_bytes();
        let (_, bank_not_last) = b.split_last().unwrap();

        let (idx, max_digit) = bank_not_last
            .iter()
            .enumerate()
            .max_by(|x, y| {
                let c = x.1.cmp(&y.1);
                match c {
                    // prefer first index to get largest possible number after that
                    std::cmp::Ordering::Equal => y.0.cmp(&x.0),
                    _ => c,
                }
            })
            .unwrap();
        let max_second_digit = b[idx + 1..].iter().max().unwrap();
        let result = (max_digit - b'0') * 10 + (max_second_digit - b'0');
        total += result as u64;
    }

    Some(total)
}

fn parse_input(input: &str) -> IResult<&str, Vec<&str>> {
    let mut parser = many1(terminated(digit1, opt(newline)));

    parser.parse(input)
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
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
