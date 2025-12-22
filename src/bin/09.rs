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

fn determinant(p1: &(i64, i64), p2: &(i64, i64)) -> i64 {
    p1.0 * p2.1 - p2.0 * p1.1
}

fn is_inside_polygon(tiles: &Vec<(i64, i64)>, p: &(i64, i64)) -> bool {
    let origin = (0, 0);

    let mut intersections = 0;
    for (idx, next) in tiles.iter().enumerate() {
        let prev = if idx > 0 {
            &tiles[idx - 1]
        } else {
            tiles.last().unwrap()
        };

        let line1_dx = prev.0 - next.0;
        let line1_dy = prev.1 - next.1;
        // compare to (0, 0) for ray cast
        let line2_dx = p.0;
        let line2_dy = p.1;

        // Cramer's rule
        let divisor = determinant(&(line1_dx, line2_dx), &(line1_dy, line2_dy));

        let d = (determinant(prev, next), determinant(p, &origin));
        let x = determinant(&d, &(line1_dx, line2_dx)) / divisor;
        let y = determinant(&d, &(line1_dy, line2_dy)) / divisor;

        if x >= prev.0 && x <= next.0 || x >= next.0 && x <= prev.0 {
            if y >= prev.1 && y <= next.1 || y >= next.1 && y <= prev.1 {
                intersections += 1;
            }
        }
    }
    // odd number means inside the polygon - ray casting
    println!("test {:?} => {}", p, intersections);
    intersections % 2 == 1
}

pub fn part_two(input: &str) -> Option<i64> {
    let (_, tiles) = parse_input(input).unwrap();

    tiles
        .iter()
        .combinations(2)
        .filter(|v| {
            [
                (v[0].0, v[0].1),
                (v[1].0, v[0].1),
                (v[0].0, v[1].1),
                (v[1].0, v[1].1),
            ]
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
