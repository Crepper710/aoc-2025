use std::cmp::Ordering;

advent_of_code::solution!(3);

fn parse(input: &str) -> Vec<Vec<u64>> {
    input.trim_end().lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u64).collect::<_>()).collect::<_>()
}

fn find_highest_digit(vec: &[u64], start: usize, end: usize) -> (usize, u64) {
    vec[..end].iter().cloned().enumerate().skip(start).max_by(|(i1, b1), (i2, b2)| {
        match b1.cmp(b2) {
            Ordering::Equal => i1.cmp(i2).reverse(), // By default the last match get chosen
            ordering => ordering,
        }
    }).unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    let banks = parse(input);
    Some(banks.into_iter().map(|bank| {
        let (i, first) = find_highest_digit(&bank, 0, bank.len() - 1);
        let second = find_highest_digit(&bank, i + 1, bank.len()).1;
        first * 10 + second
    }).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let banks = parse(input);
    Some(banks.into_iter().map(|bank| {
        let mut joltage = 0;
        let mut skip = 0;
        for i in (0..12).rev() {
            let (index, digit) = find_highest_digit(&bank, skip, bank.len() - i);
            joltage *= 10;
            joltage += digit;
            skip = index + 1;
        }
        joltage
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
