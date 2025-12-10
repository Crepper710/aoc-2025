use std::collections::VecDeque;

use itertools::Itertools;
use z3::{Optimize, SatResult, ast::Int};

advent_of_code::solution!(10);

struct Machine {
    target_state: usize,
    buttons: Vec<usize>,
    joltages: Vec<usize>,
}

impl Machine {
    fn from_str(s: &str) -> Option<Self> {
        let (target_state, rest) = s[1..].split_once("] (")?;
        let (buttons, joltages) = rest[..(rest.len() - 1)].split_once(") {")?;
        let target_state = target_state.chars().rev().fold(0, |acc, c| (acc << 1) + if c == '#' {1} else {0});
        let buttons = buttons.split(") (").map(|s| s.split(',').map(|s| 1 << s.parse::<usize>().unwrap()).sum()).collect::<Vec<_>>();
        let joltages = joltages.split(',').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
        Some(Self { target_state, buttons, joltages })
    }

    fn fewest_presses(&self) -> u64 {
        self.buttons.iter().powerset().filter_map(|buttons| if buttons.iter().fold(0, |acc, n| acc ^ *n) == self.target_state {Some(buttons.len())} else {None}).min().unwrap() as u64
    }

    // Heavily inspired by: https://www.reddit.com/r/adventofcode/comments/1pity70/comment/nta1om6/
    fn fewest_joltage_presses(&self) -> u64 {
        let opt = Optimize::new();
        let total_presses = Int::fresh_const("total_presses");

        let button_presses = (0..self.buttons.len()).map(|i| Int::fresh_const(&format!("button_{i}"))).collect::<Vec<_>>();
        button_presses.iter().for_each(|b| opt.assert(&b.ge(0)));

        for (pos, &target) in self.joltages.iter().enumerate() {
            let mut terms = vec![];

            for (button_presses, button) in button_presses.iter().zip(self.buttons.iter()) {
                if button & (1 << pos) != 0 {
                    terms.push(button_presses.clone());
                }
            }

            let sum = Int::add(&terms.iter().collect::<Vec<_>>());
            opt.assert(&sum.eq(Int::from_u64(target as u64)));
        }

        opt.assert(&total_presses.eq(Int::add(&button_presses)));
        opt.minimize(&total_presses);

        match opt.check(&[]) {
            SatResult::Sat => opt.get_model().unwrap().eval(&total_presses, true).and_then(|t| t.as_u64()).unwrap(),
            _ => panic!("No solution found"),
        }
    }
}

fn parse(input: &str) -> Vec<Machine> {
    input.lines().map(|line| Machine::from_str(line).unwrap()).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse(input);
    Some(machines.iter().map(Machine::fewest_presses).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse(input);
    Some(machines.iter().map(Machine::fewest_joltage_presses).sum())
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
}
