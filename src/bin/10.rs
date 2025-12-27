use std::collections::HashMap;

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{newline, one_of, u64},
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::{delimited, terminated},
};

advent_of_code::solution!(10);

fn parse_input(input: &str) -> IResult<&str, Vec<(Vec<bool>, Vec<Vec<u64>>, Vec<u64>)>> {
    let mut parser = many1(terminated(
        (
            delimited(tag("["), many1(one_of(".#").map(|x| x == '#')), tag("] ")),
            separated_list1(
                tag(" "),
                delimited(tag("("), separated_list1(tag(","), u64), tag(")")),
            ),
            delimited(tag(" {"), separated_list1(tag(","), u64), tag("}")),
        ),
        opt(newline),
    ));

    parser.parse(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, machines) = parse_input(input).unwrap();

    let mut total = 0;
    for (light_diagram, wires, _) in machines {
        let target_state = light_diagram
            .iter()
            .enumerate()
            .fold(0usize, |acc, (i, &e)| acc + if e { 1 << i } else { 0 });

        let buttons: Vec<usize> = wires
            .iter()
            .map(|w| w.iter().fold(0, |acc, c| acc + (1 << c)))
            .collect();

        total += calculate_min_presses(target_state, &buttons);
    }

    Some(total)
}

fn calculate_min_presses(target_state: usize, buttons: &[usize]) -> u64 {
    // Not supporting one button press here
    let mut sequences: HashMap<usize, u64> = HashMap::from_iter(buttons.iter().map(|&v| (v, 1)));
    loop {
        let mut new_sequences = sequences.clone();
        for (&result, &num_presses) in &sequences {
            for (&s_result, &s_num_presses) in &sequences {
                let new_result = result ^ s_result;
                let new_num_presses = num_presses + s_num_presses;

                if let Some(v) = new_sequences.get(&new_result) {
                    if *v > new_num_presses {
                        new_sequences.insert(new_result, new_num_presses);
                    }
                } else {
                    new_sequences.insert(new_result, new_num_presses);
                }
            }
        }
        if let Some(r) = new_sequences.get(&target_state) {
            return *r;
        }

        sequences = new_sequences;
    }
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
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
