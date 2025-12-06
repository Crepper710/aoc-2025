advent_of_code::solution!(6);

enum Operation {
    Add,
    Mul,
}

impl Operation {
    fn apply(&self, nums: Vec<u64>) -> u64 {
        match self {
            Self::Add => nums.into_iter().reduce(|a, b| a + b).unwrap(),
            Self::Mul => nums.into_iter().reduce(|a, b| a * b).unwrap(),
        }
    }
}

fn parse_one(input: &str) -> Option<Vec<(Vec<u64>, Operation)>> {
    let lines = input.trim_end().lines().collect::<Vec<_>>();
    let operations_index = lines.len() - 1;
    let operations = lines[operations_index].split_whitespace().map(|s| if s == "+" {Operation::Add} else {Operation::Mul});
    let mut number_iters = lines[..operations_index].iter().map(|s| s.split_whitespace().map(|s| s.parse::<u64>().unwrap())).collect::<Vec<_>>();
    let mut out = vec![];
    for operation in operations {
        let mut numbers = vec![];
        for number_iter in number_iters.iter_mut() {
            numbers.push(number_iter.next()?);
        }
        out.push((numbers, operation));
    }
    Some(out)
}

pub fn part_one(input: &str) -> Option<u64> {
    let problems = parse_one(input)?;
    Some(problems.into_iter().map(|(numbers, operation)| operation.apply(numbers)).sum())
}

fn parse_two(input: &str) -> Option<Vec<(Vec<u64>, Operation)>> {
    let lines = input.trim_end().lines().collect::<Vec<_>>();
    let operations_index = lines.len() - 1;
    let operations = lines[operations_index].split_whitespace().map(|s| if s == "+" {Operation::Add} else {Operation::Mul});
    let accumulator = lines[0].chars().map(|c| c.to_string()).collect::<Vec<_>>();
    let rotated_number_strings = lines[1..operations_index].iter().fold(accumulator, |accumulator, s| accumulator.into_iter().zip(s.chars()).map(|(mut s, c)| {s.push(c); s}).collect()).into_iter().map(|s| s.trim().to_string()).collect::<Vec<_>>();
    let mut number_str_iters = rotated_number_strings.split(|s| s.is_empty());
    let mut out = vec![];
    for operation in operations {
        out.push((number_str_iters.next()?.iter().map(|s| s.parse::<u64>().unwrap()).collect(), operation));
    }
    Some(out)
}

pub fn part_two(input: &str) -> Option<u64> {
    let problems = parse_two(input)?;
    Some(problems.into_iter().map(|(numbers, operation)| operation.apply(numbers)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
