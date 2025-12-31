use itertools::Itertools;

advent_of_code::solution!(2);

fn check_pattern(id: &str, pattern_sizes: &[usize]) -> bool {
    for &i in pattern_sizes {
        if !id.len().is_multiple_of(i) {
            continue;
        }

        let section_size = id.len() / i;
        if id.as_bytes().chunks(section_size).all_equal() {
            return true;
        }
    }

    false
}

fn find_invalid_ids(input: &str, pattern_sizes: &[usize]) -> Option<u64> {
    let ranges = input
        .trim()
        .split(',')
        .filter_map(|s| s.split_once('-'))
        .filter_map(|(a, b)| Some((a.parse::<u64>().ok()?, b.parse::<u64>().ok()?)));

    let mut sum = 0;
    for (start, end) in ranges {
        for i in start..=end {
            if check_pattern(&i.to_string(), pattern_sizes) {
                sum += i;
            }
        }
    }

    Some(sum)
}

pub fn part_one(input: &str) -> Option<u64> {
    find_invalid_ids(input, &[2])
}

pub fn part_two(input: &str) -> Option<u64> {
    find_invalid_ids(input, &[2, 3, 4, 5, 6, 7])
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
