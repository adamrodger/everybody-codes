use std::collections::{HashSet, VecDeque};
use utils::grid::{Grid, Point};

pub fn part1(input: &str) -> u32 {
    solve(input)
}

pub fn part2(input: &str) -> u32 {
    solve(input)
}

pub fn part3(input: &str) -> u32 {
    solve(input)
}

fn solve(input: &str) -> u32 {
    let grid: Grid<char> = input.parse().unwrap();

    let targets: HashSet<&char> = grid
        .rows()
        .flat_map(|r| r.iter().filter(|&c| c.is_alphabetic()))
        .collect();
    let all_collected = targets
        .iter()
        .fold(0u32, |acc, &t| acc + (1 << (*t as u8 - b'A')));

    let start_x = grid
        .rows()
        .next()
        .unwrap()
        .iter()
        .position(|&c| c == '.')
        .unwrap() as i32;
    let start = Point::new(start_x, 0);

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_front((0, start, 0u32));

    let mut most_collected: u32 = 0;

    while let Some((distance, current, mut collected)) = queue.pop_front() {
        if !seen.insert((current, collected)) {
            continue;
        }

        if collected == all_collected && current == start {
            return distance;
        }

        if collected.count_ones() + 3 < most_collected {
            // heuristic - no point carrying on down an underperforming path
            continue;
        }

        let Some(tile) = grid.at(current) else {
            unreachable!();
        };

        if tile.is_alphabetic() {
            collected |= 1 << (*tile as u8 - b'A');
            most_collected = most_collected.max(collected.count_ones());
        }

        for neighbour in current.neighbours4() {
            let Some(&cell) = grid.at(neighbour) else {
                continue;
            };

            if cell == '.' || cell.is_alphabetic() {
                queue.push_back((distance + 1, neighbour, collected));
            }
        }
    }

    unreachable!("No path to any target")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EVENT;

    const QUEST: u32 = 15;

    #[test]
    fn test_part1_real() {
        let input = utils::load_event_input(EVENT, QUEST, 1);
        assert_eq!(part1(&input), 184);
    }

    #[test]
    fn test_part2_real() {
        let input = utils::load_event_input(EVENT, QUEST, 2);
        assert_eq!(part2(&input), 526);
    }

    #[test]
    fn test_part3_real() {
        let input = utils::load_event_input(EVENT, QUEST, 3);
        assert_eq!(part3(&input), 1580);
    }
}
