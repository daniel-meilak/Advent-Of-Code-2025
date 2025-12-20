advent_of_code::solution!(5);

fn parse_input(input: &str) -> Option<(Vec<(u64, u64)>, &str)> {
    let (unparsed_ranges, ids) = input.split_once("\n\n")?;

    let mut ranges: Vec<_> = unparsed_ranges
        .lines()
        .filter_map(|s| s.split_once('-'))
        .filter_map(|(start, end)| Some((start.parse::<u64>().ok()?, end.parse::<u64>().ok()?)))
        .collect();

    ranges.sort_unstable();

    let mut filtered_ranges = vec![ranges[0]];
    for (new_start, new_end) in &ranges[1..] {
        let (_, end) = filtered_ranges.last_mut()?;

        if new_start <= end {
            if new_end <= end {
                continue;
            }

            *end = *new_end;
            continue;
        }

        filtered_ranges.push((*new_start, *new_end));
    }

    Some((filtered_ranges, ids))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, ids) = parse_input(input)?;

    let mut fresh_count = 0;
    for id in ids.lines().filter_map(|s| s.parse::<u64>().ok()) {
        for (start, end) in &ranges {
            if id < *end && id > *start {
                fresh_count += 1;
                break;
            }
        }
    }

    Some(fresh_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = parse_input(input)?;

    let mut sum = 0;
    for (start, end) in ranges {
        sum += end - start + 1;
    }

    Some(sum)
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
        assert_eq!(result, Some(14));
    }
}
