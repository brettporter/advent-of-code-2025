use factor::factor::factor;
use fxhash::FxHashSet;
use nom::{
    IResult, Parser, bytes::complete::tag, character::complete::usize, multi::separated_list1,
    sequence::separated_pair,
};

advent_of_code::solution!(2);

fn parse_input(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    let mut parser = separated_list1(tag(","), separated_pair(usize, tag("-"), usize));
    parser.parse(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, ranges) = parse_input(input).unwrap();

    let mut total = 0;

    for (low, high) in ranges {
        let mut current = low;
        while current < high {
            let num_digits = num_digits(current);
            if num_digits % 2 == 1 {
                current = 10usize.pow(num_digits);
                continue;
            }

            let div = 10usize.pow(num_digits / 2);
            let top = current / div;
            for i in top..div {
                let num = i * div + i;
                if num > high {
                    break;
                }
                if num >= low {
                    total += num;
                }
            }
            current = 10usize.pow(num_digits);
        }
    }

    Some(total)
}

fn num_digits(current: usize) -> u32 {
    current.to_string().len().try_into().unwrap()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, ranges) = parse_input(input).unwrap();

    let mut invalid_ids = FxHashSet::default();

    for (low, high) in ranges {
        let mut current = low;
        while current < high {
            let num_digits = num_digits(current);
            if num_digits < 2 {
                current = 10usize.pow(num_digits);
                continue;
            }

            let mut factors = factor(num_digits as i64);
            factors.push(1);
            // use all values as the lengths of repeatable segments repeat them (nd / len) times

            for rpt_len in factors {
                let div = 10usize.pow(num_digits - rpt_len as u32);
                let next = 10usize.pow(rpt_len as u32);
                let top = current / div;
                for i in top..next {
                    let num = (0..=(num_digits / rpt_len as u32) as usize)
                        .reduce(|acc, _| acc * next + i)
                        .unwrap();
                    if num > high {
                        break;
                    }
                    if num >= low {
                        invalid_ids.insert(num);
                    }
                }
            }
            current = 10usize.pow(num_digits);
        }
    }

    Some(invalid_ids.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
