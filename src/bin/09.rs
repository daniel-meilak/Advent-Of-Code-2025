use std::cmp::Reverse;

advent_of_code::solution!(9);

fn area(a: (i64, i64), b: (i64, i64)) -> i64 {
    ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1)
}

pub fn part_one(input: &str) -> Option<i64> {
    let tiles: Vec<(i64, i64)> = input
        .lines()
        .filter_map(|l| l.split_once(','))
        .filter_map(|(a, b)| Some((a.parse().ok()?, b.parse().ok()?)))
        .collect();

    let mut max_area = 0;
    for (i, tile_a) in tiles.iter().enumerate() {
        for tile_b in tiles.iter().skip(i + 1) {
            let area = area(*tile_a, *tile_b);
            if area > max_area {
                max_area = area;
            }
        }
    }

    Some(max_area)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut tiles: Vec<(i64, i64)> = input
        .lines()
        .filter_map(|l| l.split_once(','))
        .filter_map(|(a, b)| Some((a.parse().ok()?, b.parse().ok()?)))
        .collect();

    let mut areas = Vec::new();
    for (i, tile_a) in tiles.iter().copied().enumerate() {
        for tile_b in tiles.iter().copied().skip(i + 1) {
            areas.push((tile_a, tile_b, area(tile_a, tile_b)))
        }
    }

    areas.sort_by_key(|a| Reverse(a.2));

    tiles.push(*tiles.first()?);

    areas
        .iter()
        .find(|(p1, p2, _)| {
            tiles.windows(2).all(|edge| {
                p1.1.max(p2.1) <= edge[0].1.min(edge[1].1) ||
                p1.1.min(p2.1) >= edge[0].1.max(edge[1].1) ||
                p1.0.max(p2.0) <= edge[0].0.min(edge[1].0) ||
                p1.0.min(p2.0) >= edge[0].0.max(edge[1].0)
            })
        })
        .map(|max| max.2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
