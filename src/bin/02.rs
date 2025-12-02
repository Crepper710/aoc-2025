use std::ops::RangeInclusive;

advent_of_code::solution!(2);

fn parse(input: &str) -> Vec<RangeInclusive<u64>> {
    input.trim_end().split(',').map(|s| {
        let (start, end) = s.split_once('-').unwrap();
        start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()
    }).collect::<_>()
}

fn is_repeating_2(num: &u64) -> bool {
    let s = num.to_string();
    let len = s.len();
    if len % 2 == 1 {
        return false;
    }
    let (left, right) = s.split_at(len / 2);
    left == right
}

fn is_repeating_n(num: &u64) -> bool {
    let s = num.to_string();
    let len = s.len();
    'outer: for i in 1..=(len / 2) {
        if !len.is_multiple_of(i) {
            continue;
        }
        let (pattern, mut rest) = s.split_at(i);
        while !rest.is_empty() {
            let (to_check, new_rest) = rest.split_at(i);
            if to_check != pattern {
                continue 'outer;
            }
            rest = new_rest;
        }
        return true;
    }
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = parse(input);
    Some(ranges.into_iter().flat_map(|range| range.filter(is_repeating_2)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = parse(input);
    Some(ranges.into_iter().flat_map(|range| range.filter(is_repeating_n)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
