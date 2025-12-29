use std::u64::MAX;

use fxhash::FxHashMap;
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
    let mut sequences = FxHashMap::from_iter(buttons.iter().map(|&v| (v, 1)));
    loop {
        let mut new_sequences = sequences.clone();
        for (&result, &num_presses) in &sequences {
            for (&s_result, &s_num_presses) in &sequences {
                let new_result = result ^ s_result;
                let new_num_presses = num_presses + s_num_presses;

                // TODO: consider aborting early but requires map to be sorted - test if that is faster or not
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
    let (_, machines) = parse_input(input).unwrap();

    let mut total = 0;
    for (_, wires, joltage_requirements) in machines {
        println!("Testing {:?}", joltage_requirements);
        total += calculate_min_joltage_presses(&joltage_requirements, &wires);
    }

    Some(total)
}

fn apply_button(button: &[u64], state: &Vec<u64>, num: u64) -> Vec<u64> {
    let mut result = state.clone();
    for i in button {
        result[*i as usize] += num;
    }
    result
}

fn calculate_max_presses(button: &Vec<u64>, state: &[u64], joltage_requirements: &[u64]) -> u64 {
    button
        .iter()
        .map(|i| {
            let idx = *i as usize;
            joltage_requirements[idx] - state[idx]
        })
        .min()
        .unwrap()
}

fn calculate_min_joltage_presses(joltage_requirements: &Vec<u64>, buttons: &[Vec<u64>]) -> u64 {
    let state = vec![0; joltage_requirements.len()];

    let mut stack = Vec::new();
    stack.push((state, buttons, 0));

    let mut best = MAX;

    while let Some((cur_state, cur_buttons, cur_presses)) = stack.pop() {
        if let Some(button) = cur_buttons.first() {
            let max_presses = calculate_max_presses(button, &cur_state, joltage_requirements);
            for i in 0..=max_presses {
                // TODO: do I need to iterate? Does reducing by one help solve a fail here or is there a smarter way? Try iterating first see how fast it is

                let amount = max_presses - i;
                let new_presses = cur_presses + amount;
                if new_presses > best {
                    println!("Found solution {new_presses}");
                    continue;
                }
                let new_state = apply_button(button, &cur_state, amount);
                if new_state == *joltage_requirements {
                    best = new_presses;
                }
                if new_state < *joltage_requirements {
                    stack.push((new_state, &cur_buttons[1..], new_presses));
                }
            }
        }
    }

    best
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
        assert_eq!(result, Some(33));
    }

    #[test]
    fn test_calculate_min_presses() {
        assert_eq!(
            calculate_min_joltage_presses(
                &vec![7, 5, 12, 7, 2],
                &[
                    vec![0, 2, 3, 4],
                    vec![2, 3],
                    vec![0, 4],
                    vec![0, 1, 2],
                    vec![1, 2, 3, 4]
                ]
            ),
            12
        );
    }
}
