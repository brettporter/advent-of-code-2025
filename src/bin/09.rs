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

fn intersects_line(tiles: &[(i64, i64)], p1: (i64, i64), p2: (i64, i64)) -> bool {
    let horiz = p1.1 == p2.1;

    let mut adjust_p1 = p1;
    let mut adjust_p2 = p2;
    for (idx, next) in tiles.iter().enumerate() {
        // TODO: change p1 to be the segment not on the line if any end matches
    }

    let p1 = adjust_p1;
    let p2 = adjust_p2;

    for (idx, next) in tiles.iter().enumerate() {
        let prev = if idx > 0 {
            &tiles[idx - 1]
        } else {
            tiles.last().unwrap()
        };

        // getting caught here where lines intersect only at the corners of the polygon
        // I think we need to remove full line segments first

        if prev.1 == next.1 && !horiz {
            // horizontal
            let y = prev.1;
            let (x1, x2) = (min(prev.0, next.0), max(prev.0, next.0));
            let (line_y1, line_y2) = (min(p1.1, p2.1), max(p2.1, p1.1));
            let line_x = p1.0;

            if x1 <= line_x && line_x <= x2 && line_y1 <= y && y <= line_y2 {
                // ignore if it's an origin point
                if line_x != p1.0 || y != p1.1 {
                    println!(
                        "intersect horiz {:?} {:?} :: {:?} {:?} @ {line_x} {y}",
                        prev, next, p1, p2
                    );
                    return true;
                }
            }
        }

        if prev.0 == next.0 && horiz {
            // vertical
            let x = prev.0;
            let (y1, y2) = (min(prev.1, next.1), max(prev.1, next.1));
            let (line_x1, line_x2) = (min(p1.0, p2.0), max(p2.0, p1.0));
            let line_y = p1.1;

            if y1 <= line_y && line_y <= y2 && line_x1 <= x && x <= line_x2 {
                // ignore if it's an origin point
                if x != p1.0 || line_y != p1.1 {
                    println!(
                        "intersect vert {:?} {:?} :: {:?} {:?} @ {x} {line_y}",
                        prev, next, p1, p2
                    );
                    return true;
                }
            }
        }
    }
    false
}

pub fn part_two(input: &str) -> Option<i64> {
    let (_, tiles) = parse_input(input).unwrap();

    let sorted: Vec<(Vec<&(i64, i64)>, i64)> = tiles
        .iter()
        .combinations(2)
        .map(|v| {
            let (x1, y1) = v[0];
            let (x2, y2) = v[1];
            let h = (y2 - y1).abs() + 1;
            let w = (x2 - x1).abs() + 1;
            (v, w * h)
        })
        .sorted_by(|a, b| b.1.cmp(&a.1))
        .collect();

    for (v, a) in sorted {
        let (x1, y1) = (v[0].0, v[0].1);
        let (x2, y2) = (v[1].0, v[1].1);

        println!("Rect {:?} {:?}", (x1, y1), (x2, y2));

        if intersects_line(&tiles, (x1, y1), (x2, y1))
            || intersects_line(&tiles, (x2, y1), (x2, y2))
            || intersects_line(&tiles, (x2, y2), (x1, y2))
            || intersects_line(&tiles, (x1, y2), (x1, y1))
        {
            continue;
        }
        return Some(a);
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
