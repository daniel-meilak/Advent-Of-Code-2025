use std::collections::HashSet;

use rust_utils::utils::pad;

advent_of_code::solution!(4);

fn remove_rolls(input: &str, iter: usize) -> Option<usize> {
    let mut grid = pad(input, '.')?;

    let mut sum = 0;
    let mut to_remove = HashSet::new();
    for _ in 0..iter {
        for y in 1..grid.len() - 1 {
            for x in 1..grid[0].len() - 1 {
                if grid[y][x] == '.' {
                    continue;
                }

                let neighbors = [
                    grid[y + 1][x],
                    grid[y - 1][x],
                    grid[y + 1][x + 1],
                    grid[y + 1][x - 1],
                    grid[y - 1][x + 1],
                    grid[y - 1][x - 1],
                    grid[y][x + 1],
                    grid[y][x - 1],
                ];

                let count = neighbors.iter().filter(|&&c| c == '@').count();
                if count < 4 {
                    to_remove.insert((x, y));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for &(x, y) in &to_remove {
            grid[y][x] = '.';
        }

        sum += to_remove.len();
        to_remove.clear();
    }

    Some(sum)
}

pub fn part_one(input: &str) -> Option<usize> {
    remove_rolls(input, 1)
}

pub fn part_two(input: &str) -> Option<usize> {
    remove_rolls(input, usize::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
