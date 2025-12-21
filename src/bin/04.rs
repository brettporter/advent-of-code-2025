use nom::{
    IResult, Parser,
    character::complete::{newline, one_of},
    combinator::opt,
    multi::many1,
    sequence::terminated,
};

advent_of_code::solution!(4);

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let mut parser = many1(terminated(many1(one_of("@.")), opt(newline)));
    parser.parse(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, grid) = parse_input(input).unwrap();

    let mut accessible = 0;
    for row in 0..grid.len() {
        let row_length = grid[row].len();
        for col in 0..row_length {
            if grid[row][col] == '@' {
                let mut surrounding_count = 0;
                if row > 0 {
                    if col > 0 && grid[row - 1][col - 1] == '@' {
                        surrounding_count += 1;
                    }
                    if grid[row - 1][col] == '@' {
                        surrounding_count += 1;
                    }
                    if col < row_length - 1 && grid[row - 1][col + 1] == '@' {
                        surrounding_count += 1;
                    }
                }
                if row < grid.len() - 1 {
                    if col > 0 && grid[row + 1][col - 1] == '@' {
                        surrounding_count += 1;
                    }
                    if grid[row + 1][col] == '@' {
                        surrounding_count += 1;
                    }
                    if col < row_length - 1 && grid[row + 1][col + 1] == '@' {
                        surrounding_count += 1;
                    }
                }
                if col > 0 && grid[row][col - 1] == '@' {
                    surrounding_count += 1;
                }
                if col < row_length - 1 && grid[row][col + 1] == '@' {
                    surrounding_count += 1;
                }
                if surrounding_count < 4 {
                    accessible += 1;
                }
            }
        }
    }

    Some(accessible)
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
