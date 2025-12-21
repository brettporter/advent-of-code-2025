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

#[derive(Debug, Eq, PartialEq, Hash)]
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

fn find_shortest_distances(locations: &Vec<Vec<i64>>) -> Vec<(Point, Point, i64)> {
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
        // .take(max)
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    part_one_max(input, 1000)
}

fn part_one_max(input: &str, max: usize) -> Option<u64> {
    let (_, locations) = parse_input(input).unwrap();

    let shortest_distance_pairs = find_shortest_distances(&locations);

    let mut circuits: Vec<FxHashSet<Point>> = Vec::new();
    let mut connections = 0;

    for (p1, p2, _) in shortest_distance_pairs {
        if let Some(c) = circuits
            .iter_mut()
            .find(|c| c.contains(&p1) || c.contains(&p2))
        {
            if !c.contains(&p1) {
                c.insert(p1);
                connections += 1;
            } else if !c.contains(&p2) {
                c.insert(p2);
                connections += 1;
            }
        } else {
            let mut c = FxHashSet::default();
            c.insert(p1);
            c.insert(p2);
            connections += 1;
            circuits.push(c);
        }

        if connections == max {
            break;
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
    None
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
        assert_eq!(result, None);
    }
}
