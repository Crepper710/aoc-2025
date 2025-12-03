use std::ops::RangeInclusive;

advent_of_code::solution!(2);

fn parse(input: &str) -> Vec<RangeInclusive<u64>> {
    input.trim_end().split(',').map(|s| {
        let (start, end) = s.split_once('-').unwrap();
        start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()
    }).collect::<_>()
}

fn is_repeating_2(num: &u64) -> bool {
    let digits = num.ilog10() + 1;
    if !digits.is_multiple_of(2) {
        return false;
    }
    let left = num / 10u64.pow(digits / 2);
    let right = num % 10u64.pow(digits / 2);
    left == right
}

fn is_repeating_n(num: &u64) -> bool {
    let digits = num.ilog10() + 1;
    for i in 1..=(digits / 2) {
        if !digits.is_multiple_of(i) {
            continue;
        }
        let pattern = num % 10u64.pow(i);
        let expected: u64 = (0..(digits / i)).map(|k| pattern * 10u64.pow(i * k)).sum();
        if expected == *num {
            return true;
        }
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
