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

fn is_inside_polygon(tiles: &[(i64, i64)], (p_x, p_y): (i64, i64)) -> bool {
    // simplified version of point-in-polygon algorithm based on assumption of horizontal and vertical lines
    // and no internal gaps. Usually ray casting would be an odd number of intersections from a point outside
    // the polygon. However, we can just test that if there are lines in every direction from the point, it
    // is inside

    // Iterate all lines on the polygon
    let (mut left, mut right, mut up, mut down) = (false, false, false, false);
    for (idx, &(next_x, next_y)) in tiles.iter().enumerate() {
        let &(prev_x, prev_y) = if idx > 0 {
            &tiles[idx - 1]
        } else {
            tiles.last().unwrap()
        };

        if prev_y == next_y {
            // horizontal - test up / down

            let (x1, x2) = (min(prev_x, next_x), max(prev_x, next_x));
            if x1 <= p_x && p_x <= x2 {
                if p_y >= prev_y {
                    println!("U -> {p_x}, {p_y} below ({x1}-{x2}, {prev_y})");
                    up = true;
                }
                if p_y <= prev_y {
                    println!("D -> {p_x}, {p_y} above ({x1}-{x2}, {prev_y})");
                    down = true;
                }
            }
        }

        if prev_x == next_x {
            // vertical - test left/right

            let (y1, y2) = (min(prev_y, next_y), max(prev_y, next_y));
            if y1 <= p_y && p_y <= y2 {
                if p_x >= prev_x {
                    println!("L -> {p_x}, {p_y} right of ({prev_x}, {y1}-{y2})");
                    left = true;
                }
                if p_x <= prev_x {
                    println!("R -> {p_x}, {p_y} left of ({prev_x}, {y1}-{y2})");
                    right = true;
                }
            }
        }
        if left && right && up && down {
            return true;
        }
    }
    println!("Failed on point ({p_x}, {p_y}) - L {left} R {right} U {up} D {down}");
    false
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
            ((x1, y2), (x2, y1), w * h)
        })
        .sorted_by(|(_, _, a), (_, _, b)| b.cmp(&a))
        .collect();

    for ((x1, y1), (x2, y2), a) in sorted {
        println!("Testing rectangle ({x1}, {y1}), ({x2}, {y2}) size {a}");
        if is_inside_polygon(&tiles, (x1, y1)) && is_inside_polygon(&tiles, (x2, y2)) {
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
