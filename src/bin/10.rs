use std::collections::{HashMap, HashSet};

use itertools::Itertools;

// Code translated from python impl by u/4HbQ
advent_of_code::solution!(10);

type Machine = (Vec<bool>, Vec<HashSet<usize>>, Vec<isize>);

fn parse(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();
    for line in input.lines() {
        let temp: Vec<_> = line
            .split_whitespace()
            .map(|s| &s[1..s.len() - 1])
            .collect();

        let lights: Vec<_> = temp[0].chars().map(|c| c == '#').collect();
        let joltages: Vec<isize> = temp[temp.len() - 1]
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect();
        let buttons: Vec<HashSet<usize>> = temp[1..temp.len() - 1]
            .iter()
            .map(|&s| s.split(',').filter_map(|s| s.parse().ok()).collect())
            .collect();

        machines.push((lights, buttons, joltages));
    }

    machines
}

fn powerset(set: &[usize]) -> impl Iterator<Item = Vec<usize>> {
    (0..=set.len()).flat_map(move |r| set.iter().cloned().combinations(r))
}

fn opt(
    joltages: &[isize],
    options: &HashMap<Vec<bool>, Vec<Vec<usize>>>,
    output: &HashMap<Vec<usize>, Vec<usize>>,
) -> usize {
    if let Some(&min) = joltages.iter().min()
        && min < 0
    {
        return usize::MAX;
    }

    if joltages.iter().sum::<isize>() == 0 {
        return 0;
    }

    let mut answer = usize::MAX;
    let parity: Vec<_> = joltages.iter().map(|j| j % 2 != 0).collect();

    if let Some(combinations) = options.get(&parity) {
        for pressed in combinations {
            let remain: Vec<_> = joltages
                .iter()
                .zip(output[pressed].iter())
                .map(|(j, s)| (j - *s as isize).div_euclid(2))
                .collect();

            answer = answer.min(
                opt(&remain, options, output)
                    .saturating_mul(2)
                    .saturating_add(pressed.len()),
            );
        }
    }

    answer
}

pub fn part_one(input: &str) -> Option<usize> {
    let machines = parse(input);

    let mut sum = 0;
    for (lights, buttons, joltages) in machines {
        let mut options = HashMap::new();
        let mut output = HashMap::new();

        for pressed in powerset(&(0..buttons.len()).collect::<Vec<_>>()) {
            let mut supply = Vec::new();
            for j in 0..joltages.len() {
                let mut count = 0;
                for button in &pressed {
                    if buttons[*button].contains(&j) {
                        count += 1;
                    }
                }
                supply.push(count);
            }

            let parity: Vec<_> = supply.iter().map(|j| j % 2 != 0).collect();

            options
                .entry(parity)
                .or_insert(Vec::new())
                .push(pressed.clone());
            output.insert(pressed, supply);
        }

        sum += options
            .get(&lights)
            .map(|v| v.iter().map(|l| l.len()).min().unwrap_or(0))
            .unwrap_or(0);
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let machines = parse(input);

    let mut sum = 0;
    for (_, buttons, joltages) in machines {
        let mut options = HashMap::new();
        let mut output = HashMap::new();

        for pressed in powerset(&(0..buttons.len()).collect::<Vec<_>>()) {
            let mut supply = Vec::new();
            for j in 0..joltages.len() {
                let mut count = 0;
                for button in &pressed {
                    if buttons[*button].contains(&j) {
                        count += 1;
                    }
                }
                supply.push(count);
            }

            let parity: Vec<_> = supply.iter().map(|j| j % 2 != 0).collect();

            options
                .entry(parity)
                .or_insert(Vec::new())
                .push(pressed.clone());
            output.insert(pressed, supply);
        }

        sum += opt(&joltages, &options, &output);
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
