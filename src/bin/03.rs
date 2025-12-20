advent_of_code::solution!(3);

fn maximum_joltage(input: &str, num_batteries: usize) -> Option<u64> {
    let banks = input.lines();

    let mut sum = 0;
    for bank in banks {
        let mut batteries = vec!['0'; num_batteries];
        for (j, joltage) in bank.chars().enumerate() {
            for (b, battery) in batteries.iter_mut().enumerate() {
                if (bank.len() - (num_batteries - b)) < j {
                    continue;
                }

                if joltage > *battery {
                    *battery = joltage;
                    batteries[b + 1..].fill('0');
                    break;
                }
            }
        }

        sum += batteries.iter().collect::<String>().parse::<u64>().ok()?;
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
