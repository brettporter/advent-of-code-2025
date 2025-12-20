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

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
