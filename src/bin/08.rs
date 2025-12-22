use std::usize;

use fxhash::FxHashSet;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{i64, newline},
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::terminated,
};

advent_of_code::solution!(8);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn from_vec(v: &Vec<i64>) -> Self {
        Self {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }

    fn distance(&self, p: &Point) -> i64 {
        let dx = p.x - self.x;
        let dy = p.y - self.y;
        let dz = p.z - self.z;
        dx * dx + dy * dy + dz * dz
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    let mut parser = many1(terminated(separated_list1(tag(","), i64), opt(newline)));
    parser.parse(input)
}

fn find_shortest_distances(locations: &Vec<Vec<i64>>, max: usize) -> Vec<(Point, Point, i64)> {
    locations
        .iter()
        .combinations(2)
        .map(|v| {
            let p1 = Point::from_vec(&v[0]);
            let p2 = Point::from_vec(&v[1]);
            let distance = p1.distance(&p2);
            (p1, p2, distance)
        })
        .sorted_by_key(|&(_, _, distance)| distance)
        .take(max)
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    part_one_max(input, 1000)
}

fn part_one_max(input: &str, max: usize) -> Option<u64> {
    let (_, locations) = parse_input(input).unwrap();

    let shortest_distance_pairs = find_shortest_distances(&locations, max);

    let mut circuits: Vec<FxHashSet<Point>> = Vec::new();

    for (p1, p2, _) in shortest_distance_pairs {
        let c1 = circuits.iter().enumerate().find(|(_, c)| c.contains(&p1));
        let c2 = circuits.iter().enumerate().find(|(_, c)| c.contains(&p2));

        if let Some((idx1, c)) = c1 {
            if let Some((idx2, other)) = c2 {
                if idx1 != idx2 {
                    // combine circuits
                    circuits[idx1] = FxHashSet::from_iter(c.union(other).map(|&x| x));
                    circuits.remove(idx2);
                }
            } else {
                circuits.get_mut(idx1).unwrap().insert(p2);
            }
        } else {
            if let Some((idx2, _)) = c2 {
                circuits.get_mut(idx2).unwrap().insert(p1);
            } else {
                let c = FxHashSet::from_iter(vec![p1, p2]);
                circuits.push(c);
            }
        }
    }

    circuits
        .iter()
        .map(|c| c.len() as u64)
        .sorted()
        .rev()
        .take(3)
        .reduce(|acc, e| acc * e)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, locations) = parse_input(input).unwrap();

    let shortest_distance_pairs = find_shortest_distances(&locations, usize::MAX);

    let mut circuits: Vec<FxHashSet<Point>> = Vec::new();

    for (p1, p2, _) in shortest_distance_pairs {
        let c1 = circuits.iter().enumerate().find(|(_, c)| c.contains(&p1));
        let c2 = circuits.iter().enumerate().find(|(_, c)| c.contains(&p2));

        if let Some((idx1, c)) = c1 {
            if let Some((idx2, other)) = c2 {
                if idx1 != idx2 {
                    // combine circuits
                    circuits[idx1] = FxHashSet::from_iter(c.union(other).map(|&x| x));
                    circuits.remove(idx2);
                }
            } else {
                circuits.get_mut(idx1).unwrap().insert(p2);
            }
        } else {
            if let Some((idx2, _)) = c2 {
                circuits.get_mut(idx2).unwrap().insert(p1);
            } else {
                let c = FxHashSet::from_iter(vec![p1, p2]);
                circuits.push(c);
            }
        }
        if circuits.len() == 1 && circuits.first()?.len() == locations.len() {
            return Some((p2.x * p1.x) as u64);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_max(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
