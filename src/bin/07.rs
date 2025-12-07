use std::collections::HashSet;

advent_of_code::solution!(7);

fn parse(input: &str) -> Option<(usize, Vec<Vec<bool>>)> {
    Some((
        input.lines().next()?.chars().position(|c| c == 'S')?,
        input.lines().map(|line| line.chars().map(|c| c == '^').collect()).collect()
    ))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (start_pos, splitters) = parse(input)?;
    let mut splits = 0;
    let mut beams = HashSet::new();
    beams.insert(start_pos);
    for splitter_line in splitters {
        for (x, splitter) in splitter_line.into_iter().enumerate() {
            if splitter && beams.remove(&x) {
                beams.insert(x - 1);
                beams.insert(x + 1);
                splits += 1;
            }
        }
    }
    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (start_pos, splitters) = parse(input)?;
    let width = splitters.first()?.len();
    let mut super_positions = vec![0; width];
    super_positions[start_pos] = 1;
    for splitter_line in splitters {
        let mut new_super_positions = vec![0; width];
        for (x, splitter) in splitter_line.into_iter().enumerate() {
            if splitter {
                new_super_positions[x - 1] += super_positions[x];
                new_super_positions[x + 1] += super_positions[x];
            } else {
                new_super_positions[x] += super_positions[x];
            }
        }
        super_positions = new_super_positions;
    }
    Some(super_positions.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
