advent_of_code::solution!(4);

fn parse(input: &str) -> Vec<Vec<bool>> {
    input.trim_end().lines().map(|line| line.chars().map(|c| c == '@').collect::<_>()).collect::<_>()
}

const ADJACENT_TILES: [(usize, usize); 8] = [(2, 1), (2, 2), (1, 2), (0, 2), (0, 1), (0, 0), (1, 0), (2, 0)];

pub fn part_one(input: &str) -> Option<u64> {
    let paper_rolls = parse(input);
    let dimensions = (paper_rolls[0].len(), paper_rolls.len());
    let mut accessable_paper_rolls = 0;
    for y in 0..dimensions.1 {
        for x in 0..dimensions.0 {
            if !paper_rolls[y][x] {
                continue;
            }
            let mut adjacent_paper_rolls = 0u8;
            for (dx, dy) in ADJACENT_TILES {
                if let Some((x, y)) = (x + dx).checked_sub(1).and_then(|x| (y + dy).checked_sub(1).map(|y| (x, y))) {
                    if x >= dimensions.0 || y >= dimensions.1 {
                        continue;
                    }
                    if paper_rolls[y][x] {
                        adjacent_paper_rolls += 1;
                    }
                }
            }
            if adjacent_paper_rolls < 4 {
                accessable_paper_rolls += 1;
            }
        }
    }
    Some(accessable_paper_rolls)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut paper_rolls = parse(input);
    let dimensions = (paper_rolls[0].len(), paper_rolls.len());
    let mut accessable_paper_rolls = 0;
    let mut last_accessable_paper_rolls = u64::MAX;
    while accessable_paper_rolls != last_accessable_paper_rolls {
        last_accessable_paper_rolls = accessable_paper_rolls;
        for y in 0..dimensions.1 {
            for x in 0..dimensions.0 {
                if !paper_rolls[y][x] {
                    continue;
                }
                let mut adjacent_paper_rolls = 0u8;
                for (dx, dy) in ADJACENT_TILES {
                    if let Some((x, y)) = (x + dx).checked_sub(1).and_then(|x| (y + dy).checked_sub(1).map(|y| (x, y))) {
                        if x >= dimensions.0 || y >= dimensions.1 {
                            continue;
                        }
                        if paper_rolls[y][x] {
                            adjacent_paper_rolls += 1;
                        }
                    }
                }
                if adjacent_paper_rolls < 4 {
                    paper_rolls[y][x] = false;
                    accessable_paper_rolls += 1;
                }
            }
        }
    }
    Some(accessable_paper_rolls)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
