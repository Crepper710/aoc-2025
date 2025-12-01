use std::u32;

advent_of_code::solution!(1);

fn parse_input(input: &str) -> Option<Vec<(bool, i32)>> {
    let mut nums = vec![];
    for line in input.trim_end().lines() {
        nums.push((line.get(0..1)? == "L", line.get(1..)?.parse::<i32>().ok()?));
    }
    Some(nums)
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_input(input)?;
    let mut rotation = 50;
    let mut hit_0_count = 0;
    for (negative, delta) in input {
        rotation = (rotation + (delta * if negative {-1} else {1})).rem_euclid(100);
        if rotation == 0 {
            hit_0_count += 1;
        }
    }
    Some(hit_0_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse_input(input)?;
    let mut rotation: i32 = 50;
    let mut hit_0_count = 0;
    for (negative, delta) in input {
        let step = if negative {-1} else {1};
        for _ in 0..delta {
            rotation += step;
            if !(0..100).contains(&rotation) {
                rotation = rotation.rem_euclid(100);
            }
            if rotation == 0 {
                hit_0_count += 1;
            }
        }
    }
    Some(hit_0_count)
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
        assert_eq!(result, Some(6));
    }
}
