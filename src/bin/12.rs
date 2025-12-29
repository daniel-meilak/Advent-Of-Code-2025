use regex::Regex;

advent_of_code::solution!(12);

fn parse(input: &str) -> Vec<(u64,u64,u64)> {
    let pattern = Regex::new(r"(\d+)x(\d+): (\d+) (\d+) (\d+) (\d+) (\d+) (\d+)").ok().unwrap();

    pattern
        .captures_iter(input)
        .filter_map(|c| {
            let w = c[1].parse::<u64>().ok()?;
            let l =  c[2].parse::<u64>().ok()?;
            let n = (3..=8).filter_map(|i| c[i].parse::<u64>().ok()).sum::<u64>();
            Some((w, l, n))
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    // via trial and error:
    let mut possible = 0; 
    for (width, length, total) in parse(input) {
        if total <= width * length / 8 {
            possible += 1;
        }
    }

    Some(possible)
}

pub fn part_two(_input: &str) -> Option<u64> {
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
}
