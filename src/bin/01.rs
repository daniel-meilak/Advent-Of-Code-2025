advent_of_code::solution!(1);

const DIAL_SIZE: i32 = 100;

fn count_all_zeroes(start: i32, end: i32) -> i32 {
    let mut result = ((start / DIAL_SIZE) - (end / DIAL_SIZE)).abs();

    if end == 0 || (end < 0 && start > 0) {
        result += 1;
    }

    result
}

fn solve<F: FnMut(i32, i32) -> i32>(input: &str, mut count_fn: F) -> Option<i32> {
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
            _ => return None,
        }

        count += count_fn(dial, intermediate);
        dial = intermediate.rem_euclid(DIAL_SIZE);
    }

    Some(count)
}

pub fn part_one(input: &str) -> Option<i32> {
    solve(input, |_, end| (end % DIAL_SIZE == 0) as i32)
}

pub fn part_two(input: &str) -> Option<i32> {
    solve(input, count_all_zeroes)
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
