use std::cmp::{max, min};

use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{i64, newline},
    multi::many1,
    sequence::{separated_pair, terminated},
};

advent_of_code::solution!(9);

fn parse_input(input: &str) -> IResult<&str, Vec<(i64, i64)>> {
    let mut parser = many1(terminated(separated_pair(i64, tag(","), i64), newline));
    parser.parse(input)
}

pub fn part_one(input: &str) -> Option<i64> {
    let (_, tiles) = parse_input(input).unwrap();

    tiles
        .iter()
        .combinations(2)
        .map(|v| {
            let (x1, y1) = v[0];
            let (x2, y2) = v[1];
            let h = (y2 - y1).abs() + 1;
            let w = (x2 - x1).abs() + 1;
            w * h
        })
        .max()
}

fn is_inside_polygon(tiles: &[(i64, i64)], (x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> bool {
    let l_horiz = y1 == y2;
    let l_x1 = min(x1, x2);
    let l_x2 = max(x1, x2);
    let l_y1 = min(y1, y2);
    let l_y2 = max(y1, y2);

    let (mut up, mut down, mut left, mut right) = (false, false, false, false);

    // Iterate all lines on the polygon and check they don't cross any lines internally
    for (idx, &(next_x, next_y)) in tiles.iter().enumerate() {
        let &(prev_x, prev_y) = if idx > 0 {
            &tiles[idx - 1]
        } else {
            tiles.last().unwrap()
        };

        if prev_y == next_y {
            // horizontal
            let t_y = prev_y;
            let (t_x1, t_x2) = (min(prev_x, next_x), max(prev_x, next_x));

            if !l_horiz {
                let line_x = l_x1;
                if l_y1 < t_y && t_y < l_y2 && t_x1 < line_x && line_x < t_x2 {
                    // Intersection between candidate line and polygon line
                    return false;
                }
            }

            // Check if this polygon line is outside of the first point
            // Only check the first point provided as caller loops through all points in that position
            if t_x1 <= x1 && x1 <= t_x2 {
                if y1 >= t_y {
                    up = true;
                }
                if y1 <= t_y {
                    down = true;
                }
            }
        }

        if prev_x == next_x {
            // vertical
            let t_x = prev_x;
            let (t_y1, t_y2) = (min(prev_y, next_y), max(prev_y, next_y));
            if l_horiz {
                let line_y = l_y1;
                if l_x1 < t_x && t_x < l_x2 && t_y1 < line_y && line_y < t_y2 {
                    // Intersection between candidate line and polygon line
                    return false;
                }
            }

            // Check if this polygon line is outside of the first point
            // Only check the first point provided as caller loops through all points in that position
            if t_y1 <= y1 && y1 <= t_y2 {
                if x1 >= t_x {
                    left = true;
                }
                if x1 <= t_x {
                    right = true;
                }
            }
        }
    }

    // No intersections for candidate line
    // Candidate point outside polygon if not bounded by lines
    left && right && up && down
}

pub fn part_two(input: &str) -> Option<i64> {
    let (_, tiles) = parse_input(input).unwrap();

    let sorted: Vec<((i64, i64), (i64, i64), i64)> = tiles
        .iter()
        .combinations(2)
        .map(|v| {
            let &(x1, y1) = v[0];
            let &(x2, y2) = v[1];
            let h = (y2 - y1).abs() + 1;
            let w = (x2 - x1).abs() + 1;
            ((x1, y1), (x2, y2), w * h)
        })
        .sorted_by(|(_, _, a), (_, _, b)| b.cmp(&a))
        .collect();

    for ((x1, y1), (x2, y2), a) in sorted {
        if is_inside_polygon(&tiles, (x1, y1), (x1, y2))
            && is_inside_polygon(&tiles, (x1, y2), (x2, y2))
            && is_inside_polygon(&tiles, (x2, y2), (x2, y1))
            && is_inside_polygon(&tiles, (x2, y1), (x1, y1))
        {
            return Some(a);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
