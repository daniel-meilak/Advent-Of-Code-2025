use std::collections::{HashSet, VecDeque};

use regex::Regex;
use z3::{Optimize, SatResult, ast::Int};

advent_of_code::solution!(10);

type Instruction = (Vec<usize>, Vec<Vec<usize>>, Vec<usize>);

fn instruction_parser(input: &str) -> Option<Vec<Instruction>> {
    let light_re = Regex::new(r"\[(.*)\]").ok()?;
    let button_re = Regex::new(r"\(([\d,]*)\)").ok()?;
    let joltage_re = Regex::new(r"\{(.*)\}").ok()?;

    let mut machines = Vec::new();
    for line in input.lines() {
        let light: Vec<usize> = light_re
            .captures(line)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().chars().map(|c| if c == '.' {0} else {1}).collect())?;

        let buttons: Vec<Vec<usize>> = button_re
            .captures_iter(line)
            .map(|cap| {
                cap[1]
                    .split(',')
                    .filter_map(|s| s.parse().ok())
                    .collect()
            })
            .collect();

        let joltage: Vec<usize> = joltage_re.captures(line).and_then(|c| c.get(1)).map(|m| {
            m.as_str()
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect()
        })?;

        machines.push((light, buttons, joltage));
    }

    Some(machines)
}

fn solver(joltages: Vec<usize>, button_schematics: Vec<Vec<usize>>) -> Option<u64> {
    let opt = Optimize::new();
    let total_presses = Int::fresh_const("total_presses");

    let button_presses: Vec<Int> = (0..button_schematics.len())
        .map(|i| Int::fresh_const(&format!("button_{i}")))
        .collect();

    button_presses.iter().for_each(|b| opt.assert(&b.ge(0)));

    for (button_index, &target_presses) in joltages.iter().enumerate() {
        let mut terms = Vec::new();

        for (i, button_schematic) in button_schematics.iter().enumerate() {
            if button_schematic.contains(&button_index) {
                terms.push(button_presses[i].clone());
            }
        }

        let sum = Int::add(&terms.iter().collect::<Vec<&Int>>());
        opt.assert(&sum.eq(Int::from_u64(target_presses as u64)));
    }

    opt.assert(&total_presses.eq(Int::add(&button_presses)));
    opt.minimize(&total_presses);

    match opt.check(&[]) {
        SatResult::Sat => {
            let model = opt.get_model()?;
            model.eval(&total_presses, true).and_then(|t| t.as_u64())
        }
        _ => None,
    }
}

fn bfs(target: Vec<usize>, button_schematics: Vec<Vec<usize>>) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([(vec![0;target.len()], 0)]);

    while let Some(current) = queue.pop_front() {
        if visited.contains(&current.0) {
            continue;
        } else {
            visited.insert(current.0.clone());
        }
        
        if current.0 == target {
            return current.1;
        }

        for button_group in &button_schematics {
            let mut new = current.0.clone();
            for &button in button_group {
                new[button] ^= 1;
            }

            queue.push_back((new, current.1 + 1));
        }
    }

    0
}

pub fn part_one(input: &str) -> Option<usize> {
    let instructions = instruction_parser(input)?;

    let mut button_presses = 0;
    for (lights, buttons, _) in instructions {
        button_presses += bfs(lights, buttons)
    }

    Some(button_presses)
}

pub fn part_two(input: &str) -> Option<u64> {
    let instructions = instruction_parser(input)?;

    let mut button_presses = 0;
    for (_, buttons, joltages) in instructions {
        button_presses += solver(joltages, buttons)?;
    }

    Some(button_presses)
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
