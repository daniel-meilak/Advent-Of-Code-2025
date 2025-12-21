advent_of_code::solution!(6);

// fn accumulate()

pub fn part_one(input: &str) -> Option<u64> {
    let equations: Vec<Vec<_>> = input
        .trim()
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect();

    let mut sum: u64 = 0;
    for (x, &sign) in equations.last()?.iter().enumerate() {
        let operands = equations[..equations.len() - 1]
            .iter()
            .filter_map(|row| row[x].parse::<u64>().ok());

        match sign {
            "+" => sum += operands.sum::<u64>(),
            "*" => sum += operands.product::<u64>(),
            _ => {}
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines: Vec<_> = input.lines().collect();
    let mut signs = lines.pop()?.split_whitespace().rev();

    let mut numbers: Vec<_> = (0..lines[0].len())
        .rev()
        .map(|col| {
            lines
                .iter()
                .filter_map(|row| row.chars().nth(col))
                .collect::<String>()
                .trim()
                .parse::<u64>()
                .ok()
        })
        .collect();

    numbers.push(None);

    let mut sum = 0;
    let mut operands = Vec::new();
    for operand in numbers {
        
        if let Some(op) = operand {
            operands.push(op);
        } else {
            match signs.next()? {
                "+" => sum += operands.iter().sum::<u64>(),
                "*" => sum += operands.iter().product::<u64>(),
                _ => {}
            }
            operands.clear();
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
