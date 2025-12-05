use std::ops::RangeInclusive;

advent_of_code::solution!(5);

fn parse(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let (ranges, values) = input.split_once("\n\n").unwrap();
    (
        ranges.lines().map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()
        }).collect(),
        values.lines().map(|value| value.parse::<u64>().unwrap()).collect()
    )
}

pub fn part_one(input: &str) -> Option<usize> {
    let (ranges, values) = parse(input);
    Some(values.into_iter().filter(|value| ranges.iter().any(|range| range.contains(value))).count())
}

fn merge_ranges(mut ranges: Vec<RangeInclusive<u64>>, range: RangeInclusive<u64>) -> Vec<RangeInclusive<u64>> {
    let mut overlaping = vec![];
    for (i, other_range) in ranges.iter().enumerate() {
        if (range.start() <= other_range.end()) && (other_range.start() <= range.end()) {
            overlaping.push(i);
        }
    }
    if !overlaping.is_empty() {
        let mut start = *range.start();
        let mut end = *range.end();
        // reverse order to remove the last entry first!
        for i in overlaping.into_iter().rev() {
            let range = ranges.remove(i);
            start = start.min(*range.start());
            end = end.max(*range.end());
        }
        ranges.push(start..=end);
    } else {
        ranges.push(range);
    }
    ranges
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = parse(input);
    Some(ranges.into_iter().fold(vec![], merge_ranges).into_iter().map(|range| range.end() - range.start() + 1).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
