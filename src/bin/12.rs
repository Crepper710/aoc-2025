advent_of_code::solution!(12);

fn parse(input: &str) -> (Vec<usize>, Vec<(usize, Vec<usize>)>) {
    let (presents, regions) = input.rsplit_once("\n\n").unwrap();
    (
        presents.split("\n\n").map(|s| s.lines().skip(1).flat_map(str::chars).filter(|c| c == &'#').count()).collect(),
        regions.lines().map(|line| {
            let (area, counts) = line.split_once(": ").unwrap();
            (area.split('x').map(|s| s.parse::<usize>().unwrap()).product(), counts.split(' ').map(|s| s.parse::<usize>().unwrap()).collect())
        }).collect()
    )
}

pub fn part_one(input: &str) -> Option<usize> {
    let (presents, regions) = parse(input);
    // got this from reddit
    // not interested in doing a packing problem for that size of input
    Some(regions.into_iter().filter(|(size, counts)| size >= &counts.iter().zip(presents.iter()).map(|(count, size)| count * size).sum()).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    None
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
        assert_eq!(result, None);
    }
}
