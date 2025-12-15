advent_of_code::solution!(1);

fn count_zeroes(start: i32, end: i32, modulus: i32) -> i32 {
    let mut result = ((start / modulus) - (end / modulus)).abs();

    if end == 0 || (end < 0 && start > 0) {
        result += 1;
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut dial = 50;
    let mut count = 0;

    for instruction in input.lines() {
        let direction = instruction.chars().next()?;

        if !direction.is_ascii_alphanumeric() {
            return None;
        }

        let value = instruction.get(1..)?.parse::<u32>().ok()?;

        match direction {
            'R' => dial = (dial + value).rem_euclid(100),
            'L' => dial = (dial as i32 - value as i32).rem_euclid(100) as u32,
            _ => return None
        }

        if dial == 0 {
            count += 1;
        }
    }
    
    Some(count)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut dial = 50;
    let mut count = 0;

    for instruction in input.lines() {
        let direction = instruction.chars().next()?;

        if !direction.is_ascii_alphanumeric() {
            return None;
        }

        let value = instruction.get(1..)?.parse::<i32>().ok()?;

        let mut intermediate = dial;
        match direction {
            'R' => intermediate += value,
            'L' => intermediate -= value,
            _ => return None
        }

        count += count_zeroes(dial, intermediate, 100);
        dial = intermediate.rem_euclid(100);
    }
    
    Some(count)
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
