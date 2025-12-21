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

    let accessible = find_accessible(&grid);
    Some(accessible.len() as u64)
}

fn find_accessible(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut accessible = vec![];
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
                    accessible.push((row, col));
                }
            }
        }
    }

    accessible
}

fn count_total_accessible(grid: &Vec<Vec<char>>) -> u64 {
    let accessible = find_accessible(grid);
    let accessible_count = accessible.len();
    if accessible.is_empty() {
        return 0;
    }

    let mut new_grid: Vec<Vec<char>> = grid.iter().map(|r| r.clone()).collect();

    for (row, col) in accessible {
        new_grid[row][col] = '.';
    }

    accessible_count as u64 + count_total_accessible(&new_grid)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, grid) = parse_input(input).unwrap();

    Some(count_total_accessible(&grid))
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
        assert_eq!(result, Some(43));
    }
}
