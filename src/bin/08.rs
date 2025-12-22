use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

#[cfg(test)]
const BOXES: usize = 10;

#[cfg(not(test))]
const BOXES: usize = 1000;

fn distance(a: &[usize], b: &[usize]) -> usize {
    let dx = a[0].abs_diff(b[0]);
    let dy = a[1].abs_diff(b[1]);
    let dz = a[2].abs_diff(b[2]);
    dx * dx + dy * dy + dz * dz
}

fn connect_boxes(input: &str, max_connections: usize) -> (HashMap<usize, HashSet<usize>>, usize) {
    let boxes: Vec<Vec<usize>> = input
        .lines()
        .filter_map(|l| l.split(',').map(|s| s.parse().ok()).collect())
        .collect();

    let mut distances: Vec<_> = Vec::new();
    for (i, box_i) in boxes.iter().enumerate() {
        for (j, box_j) in boxes.iter().enumerate().skip(i + 1) {
            distances.push((i, j, distance(box_i, box_j)));
        }
    }

    distances.sort_by(|a, b: &(usize, usize, usize)| a.2.cmp(&b.2));

    let mut count = 0;
    let mut groups: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut circuit: HashMap<usize, usize> = HashMap::new();
    for (i, j, _) in distances.iter().take(max_connections) {
        if circuit.contains_key(i) && circuit.contains_key(j) {
            if circuit[i] != circuit[j] {
                let to_remove = circuit[i];
                for junction_box in &groups[&to_remove] {
                    let new_box = circuit[j];
                    if let Some(value) = circuit.get_mut(junction_box) {
                        *value = new_box;
                    }
                }
                let boxes = groups.remove(&to_remove).unwrap();
                if let Some(value) = groups.get_mut(&circuit[j]) {
                    value.extend(boxes);
                }
            }
        } else if circuit.contains_key(i) && !circuit.contains_key(j) {
            circuit.insert(*j, circuit[i]);
            if let Some(value) = groups.get_mut(&circuit[i]) {
                value.insert(*j);
            }
        } else if circuit.contains_key(j) && !circuit.contains_key(i) {
            circuit.insert(*i, circuit[j]);
            if let Some(value) = groups.get_mut(&circuit[j]) {
                value.insert(*i);
            }
        } else {
            let index = count;
            count += 1;
            circuit.insert(*i, index);
            circuit.insert(*j, index);
            groups.insert(index, HashSet::from([*i, *j]));
        }

        if groups.len() == 1 && circuit.len() == boxes.len() {
            return (groups, boxes[*i][0] * boxes[*j][0]);
        }
    }

    (groups, 0)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (groups, _) = connect_boxes(input, BOXES);

    let mut circuits: Vec<_> = groups.iter().collect();
    circuits.sort_by_key(|(_, value)| value.len());

    let mut sum = 1;
    for (_, value) in circuits.iter().rev().take(3) {
        sum *= value.len();
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, x_coords_multiplied) = connect_boxes(input, usize::MAX);

    Some(x_coords_multiplied)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
