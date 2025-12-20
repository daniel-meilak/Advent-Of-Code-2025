advent_of_code::solution!(2);

fn check_pattern(id: &str, pattern_sizes: &[usize]) -> bool {
    'next_pattern: for i in pattern_sizes {
        let section_size = id.len() / i;

        if !id.len().is_multiple_of(*i) {
            continue;
        }

        let section = &id[0..section_size];
        for j in 1..*i {
            if &id[(section_size * j)..(section_size * (j + 1))] != section {
                continue 'next_pattern;
            }
        }

        return true;
    }

    false
}

fn find_invalid_ids(input: &str, pattern_sizes: &[usize]) -> Option<u64> {
    let ranges = input.trim().split(',').filter_map(|s| s.split_once('-'));

    let mut sum = 0;
    for (start, end) in ranges {
        for i in start.parse::<u64>().ok()?..=end.parse::<u64>().ok()? {
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
