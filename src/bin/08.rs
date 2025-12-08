use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct JunctionBox(i64, i64, i64);

impl JunctionBox {
    fn from_str(s: &str) -> Option<Self> {
        let (x, yz) = s.split_once(',')?;
        let (y, z) = yz.split_once(',')?;
        Some(Self(x.parse::<i64>().ok()?, y.parse::<i64>().ok()?, z.parse::<i64>().ok()?))
    }

    fn dist_squared(&self, other: &Self) -> i64 {
        (self.0 - other.0) * (self.0 - other.0) + (self.1 - other.1) * (self.1 - other.1) + (self.2 - other.2) * (self.2 - other.2)
    }
}

fn parse(input: &str) -> Vec<JunctionBox> {
    input.trim_end().lines().map(|line| JunctionBox::from_str(line).unwrap()).collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_for(1000, input)
}

fn part_one_for(connections: usize, input: &str) -> Option<usize> {
    let junction_boxes = parse(input);
    let clostest_connections = junction_boxes.iter().combinations_with_replacement(2).filter_map(|combinations| {
        if let Some(a) = combinations.first() && let Some(b) = combinations.get(1) && a != b {
            Some((**a, **b))
        } else {
            None
        }
    }).sorted_by(|(a, b), (c, d)| a.dist_squared(b).cmp(&c.dist_squared(d))).take(connections);
    let mut connections_table = HashMap::<JunctionBox, Vec<JunctionBox>>::with_capacity(connections * 2);
    for connection in clostest_connections {
        connections_table.entry(connection.0).and_modify(|v| v.push(connection.1)).or_insert(vec![connection.1]);
        connections_table.entry(connection.1).and_modify(|v| v.push(connection.0)).or_insert(vec![connection.0]);
    }
    let mut to_check = HashSet::new();
    junction_boxes.into_iter().for_each(|junction_box| {to_check.insert(junction_box);});
    let mut circuits = vec![];
    while let Some(junction_box) = to_check.iter().next() {
        let mut current_circuit = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(*junction_box);
        while let Some(current) = queue.pop_front() {
            if to_check.remove(&current) {
                current_circuit.push(current);
                if let Some(next_junction_boxes) = connections_table.get(&current) {
                    for next_junction_box in next_junction_boxes.iter().cloned() {
                        queue.push_back(next_junction_box);
                    }
                }
            }
        }
        circuits.push(current_circuit);
    }
    circuits.iter().map(|v| v.len()).sorted().rev().take(3).reduce(|a, b| a * b)
}

pub fn part_two(input: &str) -> Option<i64> {
    let junction_boxes = parse(input);
    let clostest_connections = junction_boxes.iter().combinations_with_replacement(2).filter_map(|combinations| {
        if let Some(a) = combinations.first() && let Some(b) = combinations.get(1) && a != b {
            Some((**a, **b))
        } else {
            None
        }
    }).sorted_by(|(a, b), (c, d)| a.dist_squared(b).cmp(&c.dist_squared(d)));
    let mut to_check = HashSet::new();
    junction_boxes.into_iter().for_each(|junction_box| {to_check.insert(junction_box);});
    for closest_connection in clostest_connections {
        to_check.remove(&closest_connection.0);
        to_check.remove(&closest_connection.1);
        if to_check.is_empty() {
            return Some(closest_connection.0.0 * closest_connection.1.0);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_for(10, &advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
