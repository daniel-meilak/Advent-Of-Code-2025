advent_of_code::solution!(7);

fn simulate(input: &str) -> Vec<Vec<(char, usize)>> {
    let mut map: Vec<Vec<_>> = input.trim().lines().map(|l| l.chars().map(|c| (c, 0)).collect()).collect();

    for j in 1..map.len() {
        for i in 0..map[j].len() {
            match map[j-1][i].0 {
                '|' => {
                    if map[j][i].0 == '^' {
                        map[j][i].0 = 'X';
                    } else {
                        map[j][i] = ('|', map[j][i].1 + map[j-1][i].1);
                    }
                }
                'X' => {
                    map[j][i-1] = ('|', map[j][i-1].1 + map[j-2][i].1);
                    map[j][i+1] = ('|', map[j][i+1].1 + map[j-2][i].1);
                },
                'S' => map[j][i] = ('|', 1),
                _ => {}
            }
        }
    }

    map
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = simulate(input);

    let splits = map
        .iter()
        .flatten()
        .filter(|&&(c, _)| c == 'X')
        .count();
    
    Some(splits)
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = simulate(input);

    let timelines = map
        .last()?
        .iter()
        .map(|(_, n)| n)
        .sum();
    
    Some(timelines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
