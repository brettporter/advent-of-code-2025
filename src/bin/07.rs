use fxhash::{FxHashMap, FxHashSet};
use nom::{
    IResult, Parser,
    character::complete::{newline, one_of},
    combinator::opt,
    multi::many1,
    sequence::terminated,
};

advent_of_code::solution!(7);

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let mut parser = many1(terminated(many1(one_of("^.S")), opt(newline)));
    parser.parse(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, rows) = parse_input(input).unwrap();

    // assume start is in first row
    let start = (0usize, rows[0].iter().position(|&x| x == 'S').unwrap());

    let mut beams_to_process = vec![start];
    let mut split_beams = FxHashSet::default();

    while let Some((beam_row, beam_col)) = beams_to_process.pop() {
        for (idx, r) in rows[beam_row + 1..].iter().enumerate() {
            let row = beam_row + idx + 1;
            if r[beam_col] == '^' {
                let loc = (row, beam_col);
                if !split_beams.contains(&loc) {
                    beams_to_process.push((row, beam_col - 1));
                    beams_to_process.push((row, beam_col + 1));
                    split_beams.insert(loc);
                }
                break;
            }
        }
    }

    Some(split_beams.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, rows) = parse_input(input).unwrap();

    // assume start is in first row
    let start = (0usize, rows[0].iter().position(|&x| x == 'S').unwrap());
    let mut memo = FxHashMap::default();

    Some(count_timelines(start, &rows, &mut memo))
}

fn count_timelines(
    start: (usize, usize),
    rows: &Vec<Vec<char>>,
    memo: &mut FxHashMap<(usize, usize), u64>,
) -> u64 {
    if let Some(&count) = memo.get(&start) {
        return count;
    }

    let (beam_row, beam_col) = start;

    for (idx, r) in rows[beam_row + 1..].iter().enumerate() {
        let row = beam_row + idx + 1;
        if r[beam_col] == '^' {
            let timelines = count_timelines((row, beam_col - 1), rows, memo)
                + count_timelines((row, beam_col + 1), rows, memo);
            memo.insert(start, timelines);
            return timelines;
        }
    }
    return 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
