advent_of_code::solution!(3);

fn maximum_joltage(input: &str, num_batteries: usize) -> Option<u64> {
    let banks = input.lines();

    let mut sum = 0;
    for bank in banks {
        let mut batteries = vec!['0'; num_batteries];
        for (j, joltage) in bank.chars().enumerate() {
            for (b, battery) in batteries.iter_mut().enumerate() {
                if j > bank.len().saturating_sub(num_batteries - b) {
                    continue;
                }

                if joltage > *battery {
                    *battery = joltage;
                    batteries[b + 1..].fill('0');
                    break;
                }
            }
        }

        sum += batteries
            .iter()
            .fold(0u64, |acc, &c| acc * 10 + (c as u8 - b'0') as u64);
    }

    Some(sum)
}

pub fn part_one(input: &str) -> Option<u64> {
    maximum_joltage(input, 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    maximum_joltage(input, 12)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
