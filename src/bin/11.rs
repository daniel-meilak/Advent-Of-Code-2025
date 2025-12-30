use std::collections::HashMap;

advent_of_code::solution!(11);

fn parse(input: &str) -> Option<HashMap<&str, Vec<&str>>> {
    let mut graph = HashMap::new();

    for line in input.lines() {
        let mut nodes = line.split_whitespace();

        let mut parent = nodes.next()?;
        parent = &parent[..parent.len() - 1];

        let mut children = Vec::new();

        for child in nodes {
            children.push(child);
        }

        graph.insert(parent, children);
    }

    Some(graph)
}

fn count_paths<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    start: &'a str,
    end: &'a str,
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    if start == end {
        return 1;
    }

    if let Some(&total_paths) = memo.get(start) {
        return total_paths;
    }

    let mut total_paths = 0;
    if let Some(children) = graph.get(start) {
        for &child in children {
            total_paths += count_paths(graph, child, end, memo);
        }
    }

    memo.insert(start, total_paths);

    total_paths
}

pub fn part_one(input: &str) -> Option<u64> {
    let graph = parse(input)?;

    Some(count_paths(&graph, "you", "out", &mut HashMap::new()))
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph = parse(input)?;

    let mut midpoint_a = "fft";
    let mut midpoint_b = "dac";
    let mut middle = count_paths(&graph, midpoint_a, midpoint_b, &mut HashMap::new());

    if middle == 0 {
        (midpoint_a, midpoint_b) = (midpoint_b, midpoint_a);
        middle = count_paths(&graph, midpoint_a, midpoint_b, &mut HashMap::new());
    }

    let start = count_paths(&graph, "svr", midpoint_a, &mut HashMap::new());
    let end = count_paths(&graph, midpoint_b, "out", &mut HashMap::new());

    Some(start * middle * end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
