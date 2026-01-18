use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};
use utils::grid::Point;

pub fn part1(input: &str) -> usize {
    let stars = parse(input);

    constellation_size(
        &stars,
        *stars.iter().next().unwrap(),
        &mut HashSet::new(),
        usize::MAX,
    )
}

pub fn part2(input: &str) -> usize {
    part1(input)
}

pub fn part3(input: &str) -> usize {
    let stars = parse(input);

    let mut seen = HashSet::new();
    let mut constellation_sizes = vec![];

    while seen.len() < stars.len() {
        // try to start a new constellation
        let start = stars.iter().find(|&s| !seen.contains(s)).unwrap();

        let size = constellation_size(&stars, *start, &mut seen, 6);
        constellation_sizes.push(size);
    }

    constellation_sizes.sort();
    constellation_sizes.iter().rev().take(3).product()
}

fn parse(input: &str) -> HashSet<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().filter_map(move |(x, c)| {
                if c == '*' {
                    Some(Point::new(x as i32 + 1, y as i32 + 1))
                } else {
                    None
                }
            })
        })
        .collect()
}

/// Build the minimum spanning tree like Prim's algorithm, except we only need to sum the weights
fn constellation_size(
    stars: &HashSet<Point>,
    start: Point,
    seen: &mut HashSet<Point>,
    limit: usize,
) -> usize {
    seen.insert(start);

    let mut min_heap = BinaryHeap::new();
    let mut constellation_weight = 0_usize;
    let mut constellation_stars = 1_usize;

    for next in stars.iter().filter(|&p| !seen.contains(p)) {
        let distance = next.manhattan_distance(start) as usize;

        if distance >= limit {
            continue;
        }

        min_heap.push(Reverse((distance, next, start)));
    }

    while let Some(Reverse((cost, next, _))) = min_heap.pop() {
        if !seen.insert(*next) {
            continue;
        }

        constellation_weight += cost;
        constellation_stars += 1;

        for other in stars.iter().filter(|&p| !seen.contains(p)) {
            let distance = other.manhattan_distance(*next) as usize;

            if distance >= limit {
                // not part of this constellation
                continue;
            }

            min_heap.push(Reverse((distance, other, *next)));
        }
    }

    constellation_weight + constellation_stars
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EVENT;

    const QUEST: u32 = 17;

    #[test]
    fn test_part1_real() {
        let input = utils::load_event_input(EVENT, QUEST, 1);
        assert_eq!(part1(&input), 140);
    }

    #[test]
    fn test_part2_real() {
        let input = utils::load_event_input(EVENT, QUEST, 2);
        assert_eq!(part2(&input), 1307);
    }

    #[test]
    fn test_part3_real() {
        let input = utils::load_event_input(EVENT, QUEST, 3);
        assert_eq!(part3(&input), 4915245972);
    }

    #[test]
    fn test_part1_example() {
        let input = r#".......................................
..*.......*...*.....*...*......**.**...
....*.................*.......*..*..*..
..*.........*.......*...*.....*.....*..
......................*........*...*...
..*.*.....*...*.....*...*........*.....
......................................."#;
        assert_eq!(part3(input), 15624);
    }
}
