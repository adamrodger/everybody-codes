use std::collections::{HashSet, VecDeque};
use utils::{graph::Graph, grid::*};

pub fn part1_and_2(input: &str) -> u32 {
    let grid = parse_grid(input);

    let start = input
        .lines()
        .enumerate()
        .find_map(|(y, line)| {
            line.chars().enumerate().find_map(|(x, c)| {
                if c == 'S' {
                    Some(Point::new(x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let target = input
        .lines()
        .enumerate()
        .find_map(|(y, line)| {
            line.chars().enumerate().find_map(|(x, c)| {
                if c == 'E' {
                    Some(Point::new(x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let graph = build_graph(&grid, start);

    graph
        .dijkstra(start, target)
        .expect("Unable to find a valid path") as u32
}

pub fn part3(input: &str) -> u32 {
    let grid = parse_grid(input);

    let start: Vec<Point> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == 'S' {
                    Some(Point::new(x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect();

    let target = input
        .lines()
        .enumerate()
        .find_map(|(y, line)| {
            line.chars().enumerate().find_map(|(x, c)| {
                if c == 'E' {
                    Some(Point::new(x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let graph = build_graph(&grid, *start.first().unwrap());

    graph.dijkstra_many(&start, target).expect("No paths found") as u32
}

fn parse_grid(input: &str) -> Grid<Option<u32>> {
    Grid::from_rows(
        input
            .lines()
            .map(|line| {
                line.bytes()
                    .map(|b| match b {
                        b'0'..=b'9' => Some((b - b'0') as u32),
                        b'S' => Some(0),
                        b'E' => Some(0),
                        _ => None,
                    })
                    .collect()
            })
            .collect(),
    )
}

fn build_graph(grid: &Grid<Option<u32>>, start: Point) -> Graph {
    let mut graph = Graph::new();
    let mut queue = VecDeque::new();
    let mut done = HashSet::new();

    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        if !done.insert(current) {
            continue;
        }

        let height = grid
            .at(current)
            .expect("Somehow navigated out of bounds")
            .expect("Somehow navigated into a wall");

        for neighbour in current.neighbours4() {
            if let Some(Some(next)) = grid.at(neighbour) {
                let height_diff = ((height + 10 - next) % 10).min((next + 10 - height) % 10);

                graph.add_edge(current, neighbour, height_diff as usize + 1);
                queue.push_back(neighbour);
            }
        }
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EVENT;

    const QUEST: u32 = 13;

    #[test]
    fn test_part1_real() {
        let input = utils::load_event_input(EVENT, QUEST, 1);
        assert_eq!(part1_and_2(&input), 157);
    }

    #[test]
    fn test_part1_example() {
        #[rustfmt::skip]
        let input = [
            "#######",
            "#6769##",
            "S50505E",
            "#97434#",
            "#######",
        ]
        .join("\n")
        .to_string();

        assert_eq!(part1_and_2(&input), 28);
    }

    #[test]
    fn test_part2_real() {
        let input = utils::load_event_input(EVENT, QUEST, 2);
        assert_eq!(part1_and_2(&input), 634);
    }

    #[test]
    fn test_part3_real() {
        let input = utils::load_event_input(EVENT, QUEST, 3);
        assert_eq!(part3(&input), 507);
    }
}
