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

fn is_inside_polygon(tiles: &Vec<(i64, i64)>, p: &(i64, i64)) -> bool {
    // ray cast from (0, y) -> (x, y)
    // succeed if a) this point is on a line; b) ray casting intersects odd numbers of lines
    let mut intersections = 0;
    for (idx, next) in tiles.iter().enumerate() {
        let prev = if idx > 0 {
            &tiles[idx - 1]
        } else {
            tiles.last().unwrap()
        };

        if prev.1 == next.1 {
            // horizontal
            let y = prev.1;
            let (x1, x2) = (min(prev.0, next.0), max(prev.0, next.0));
            if p.1 == y {
                // check if point is on the line
                if x1 <= p.0 && p.0 <= x2 {
                    return true;
                }
            }

            // check if point is beyond the line
            if p.0 > x2 {
                intersections += 1;
            }
        } else {
            // vertical
            let x = prev.0;
            let (y1, y2) = (min(prev.1, next.1), max(prev.1, next.1));

            // check if point is on the line
            if p.0 == x {
                if y1 <= p.1 && p.1 <= y2 {
                    return true;
                }
            }

            // check if a ray intersects - vertical lines crossed
            if x < p.0 {
                if y1 <= p.1 && p.1 <= y2 {
                    intersections += 1;
                }
            }
        }
    }
    intersections % 2 == 1
}

pub fn part_two(input: &str) -> Option<i64> {
    let (_, tiles) = parse_input(input).unwrap();

    tiles
        .iter()
        .combinations(2)
        .filter(|v| {
            let (x1, y1) = (v[0].0, v[0].1);
            let (x2, y2) = (v[1].0, v[1].1);
            // rectangle for these two points
            [(x1, y1), (x2, y1), (x2, y2), (x1, y2)]
                .iter()
                .all(|p| is_inside_polygon(&tiles, p))
        })
        .map(|v| {
            let (x1, y1) = v[0];
            let (x2, y2) = v[1];
            let h = (y2 - y1).abs() + 1;
            let w = (x2 - x1).abs() + 1;
            w * h
        })
        .max()
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
