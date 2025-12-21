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
    let (_, banks) = parse_input(input).unwrap();

    let mut total = 0;

    for bank in banks {
        if let Some(result) = find_max_joltage(bank.as_bytes(), 12) {
            total += result;
        }
    }

    Some(total)
}

fn find_max_joltage(bank: &[u8], req_len: usize) -> Option<u64> {
    let mut max_joltage = None;

    if req_len == bank.len() {
        return Some(bank.iter().fold(0, |acc, e| acc * 10 + (e - b'0') as u64));
    }

    let mut choices: Vec<(usize, &u8)> =
        bank[0..=bank.len() - req_len].iter().enumerate().collect();
    choices.sort_by(|x, y| {
        let c = y.1.cmp(&x.1);
        match c {
            // prefer first index to get largest possible number after that
            std::cmp::Ordering::Equal => x.0.cmp(&y.0),
            _ => c,
        }
    });

    for (i, b) in choices {
        let d1 = (b - b'0') as u64;
        if req_len == 1 {
            return Some(d1);
        }

        if let Some(rem) = find_max_joltage(&bank[i + 1..], req_len - 1) {
            let res = 10u64.pow(req_len as u32 - 1) * d1 + rem;
            if max_joltage.is_none() || res > max_joltage.unwrap() {
                max_joltage = Some(res);
                break;
            }
        }
    }

    max_joltage
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
        assert_eq!(result, Some(3121910778619));
    }

    #[test]
    fn test_joltage() {
        assert_eq!(
            find_max_joltage("234234234234278".as_bytes(), 12),
            Some(434234234278)
        );
    }
}
