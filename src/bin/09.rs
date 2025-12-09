use std::ops::Add;

use itertools::Itertools;

advent_of_code::solution!(9);

#[derive(Clone, Copy)]
struct Vec2(u64, u64);

impl Vec2 {
    fn from_str(s: &str) -> Option<Self> {
        let (x, y) = s.split_once(',')?;
        Some(Self(x.parse().ok()?, y.parse().ok()?))
    }

    fn area(&self, other: &Self) -> u64 {
        self.0.abs_diff(other.0).add(1) * self.1.abs_diff(other.1).add(1)
    }

    fn min(&self, other: &Self) -> Self {
        Self(self.0.min(other.0), self.1.min(other.1))
    }

    fn max(&self, other: &Self) -> Self {
        Self(self.0.max(other.0), self.1.max(other.1))
    }
}

#[derive(Clone, Copy)]
struct AABB {
    min: Vec2,
    max: Vec2,
}

impl AABB {
    fn new(a: Vec2, b: Vec2) -> Self {
        Self {
            min: a.min(&b),
            max: a.max(&b),
        }
    }

    fn area(&self) -> u64 {
        self.min.area(&self.max)
    }

    fn fully_contains(&self, other: &Self) -> bool {
        self.min.0 < other.max.0 && self.max.0 > other.min.0 && self.min.1 < other.max.1 && self.max.1 > other.min.1
    }
}

fn parse(input: &str) -> Vec<Vec2> {
    input.trim_end().lines().map(|line| Vec2::from_str(line).unwrap()).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse(input);
    points.into_iter().combinations(2).map(|v| v[0].area(&v[1])).max()
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse(input);
    let edges = points.iter().circular_tuple_windows().map(|(a, b)| AABB::new(*a, *b)).collect::<Vec<_>>();
    points.into_iter().combinations(2).filter_map(|v| {
        let aabb = AABB::new(v[0], v[1]);
        if edges.iter().any(|edge| aabb.fully_contains(edge)) {
            None
        } else {
            Some(aabb.area())
        }
    }).max()
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
        assert_eq!(result, None);
    }
}
