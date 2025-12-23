use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node<'a>(&'a str, Vec<&'a str>);

#[derive(Debug, Clone, Copy)]
enum PathMode {
    Full,
    FirstLetter,
}

pub fn part1(input: &str) -> String {
    let connections = parse_input(input);
    let visited = calculate_depth(&connections, PathMode::Full);
    get_unique_path(&visited)
}

pub fn part2(input: &str) -> String {
    let connections = parse_input(input);
    let visited = calculate_depth(&connections, PathMode::FirstLetter);
    get_unique_path(&visited)
}

pub fn part3(input: &str) -> String {
    let connections = {
        let mut connections = parse_input(input);
        connections.remove("BUG");
        connections.remove("ANT");

        connections
    };

    let visited = calculate_depth(&connections, PathMode::FirstLetter);
    get_unique_path(&visited)
}

/// Parses the input into a HashMap of node names to Node structs.
fn parse_input(input: &str) -> HashMap<&str, Node<'_>> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(":");

            let node = parts.next().unwrap();
            let neighbours = parts.next().unwrap().split(",").collect::<Vec<&str>>();

            (node, Node(node, neighbours))
        })
        .collect()
}

/// Calculates the depth of each node from the starting node "RR".
///
/// Returns a HashMap where the keys are depths and the values are
/// vectors of paths leading to "@" at that depth.
///
/// One depth level may have multiple paths, and there should be a
/// a depth that only has one path.
fn calculate_depth(nodes: &HashMap<&str, Node>, mode: PathMode) -> HashMap<usize, Vec<String>> {
    let mut visited = HashMap::new();

    let start_path = match mode {
        PathMode::Full => "RR".to_string(),
        PathMode::FirstLetter => "R".to_string(),
    };

    visit(&nodes["RR"], start_path, 0, &mut visited, nodes, mode);

    visited
}

/// Recursively visits nodes, building paths and recording them in the visited HashMap.
///
/// - `current`: The current node being visited.
/// - `path`: The path taken to reach the current node.
/// - `depth`: The current depth in the traversal.
/// - `visited`: A mutable reference to the HashMap recording visited paths by depth.
/// - `nodes`: The complete map of nodes for reference.
/// - `mode`: The mode for path construction (full names or first letters).
fn visit(
    current: &Node,
    path: String,
    depth: usize,
    visited: &mut HashMap<usize, Vec<String>>,
    nodes: &HashMap<&str, Node>,
    mode: PathMode,
) {
    for &neighbour in &current.1 {
        if neighbour == "@" {
            visited
                .entry(depth)
                .and_modify(|p: &mut Vec<String>| p.push(path.clone() + "@"))
                .or_insert_with(|| vec![path.clone() + "@"]);
            continue;
        }

        if !nodes.contains_key(neighbour) {
            continue;
        }

        let next = &nodes[neighbour];
        let next_path = match mode {
            PathMode::Full => path.clone() + next.0,
            PathMode::FirstLetter => path.clone() + &next.0.chars().next().unwrap().to_string(),
        };

        visit(next, next_path, depth + 1, visited, nodes, mode);
    }
}

/// From the visited HashMap, finds the unique path (the one with only one entry)
fn get_unique_path(visited: &HashMap<usize, Vec<String>>) -> String {
    visited
        .iter()
        .filter(|v| v.1.len() == 1)
        .map(|v| v.1.first().unwrap())
        .next()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EVENT;

    const QUEST: u32 = 6;

    #[test]
    fn test_part1_real() {
        let input = utils::load_event_input(EVENT, QUEST, 1);
        assert_eq!(part1(&input), "RRXNMWPGTXQW@");
    }

    #[test]
    fn test_part2_real() {
        let input = utils::load_event_input(EVENT, QUEST, 2);
        assert_eq!(part2(&input), "RZSPWCMTRC@");
    }

    #[test]
    fn test_part3_real() {
        let input = utils::load_event_input(EVENT, QUEST, 3);
        assert_eq!(part3(&input), "RKXHJDGNTMGB@");
    }
}
