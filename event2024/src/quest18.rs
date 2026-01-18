use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};
use utils::grid::Point;

pub fn part1(input: &str) -> usize {
    let (start, points, mut trees) = parse(input);

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    for s in start {
        queue.push_back((s, 0));
        seen.insert(s);
    }

    while let Some((current, distance)) = queue.pop_front() {
        if trees.contains(&current) {
            trees.remove(&current);

            if trees.is_empty() {
                return distance;
            }
        }

        for neighbour in current.neighbours4() {
            if points.contains(&neighbour) && !seen.contains(&neighbour) {
                seen.insert(neighbour);
                queue.push_back((neighbour, distance + 1));
            }
        }
    }

    unreachable!("Not all trees could be reached")
}

pub fn part2(input: &str) -> usize {
    part1(input)
}

pub fn part3(input: &str) -> usize {
    let (_, points, trees) = parse(input);

    points
        .par_iter()
        .filter(|&p| !trees.contains(p))
        .copied()
        .map(|start| time_sum(start, trees.clone(), &points))
        .min()
        .expect("At least one starting point should be possible")
}

fn time_sum(start: Point, mut trees: HashSet<Point>, points: &HashSet<Point>) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let mut time_sum = 0;

    queue.push_back((start, 0));
    seen.insert(start);

    while let Some((current, distance)) = queue.pop_front() {
        if trees.contains(&current) {
            trees.remove(&current);
            time_sum += distance;

            if trees.is_empty() {
                return time_sum;
            }
        }

        for neighbour in current.neighbours4() {
            if points.contains(&neighbour) && !seen.contains(&neighbour) {
                seen.insert(neighbour);
                queue.push_back((neighbour, distance + 1));
            }
        }
    }

    unreachable!("Not all trees could be reached")
}

fn parse(input: &str) -> (Vec<Point>, HashSet<Point>, HashSet<Point>) {
    let mut points = HashSet::new();
    let mut trees = HashSet::new();
    let mut start = vec![];

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            let p = Point::new(x as i32, y as i32);

            match c {
                '.' => {
                    points.insert(p);

                    if x == 0 || y == 0 || x == line.len() - 1 || y == input.lines().count() - 1 {
                        // on an edge
                        start.push(p);
                    }
                }
                'P' => {
                    points.insert(p);
                    trees.insert(p);
                }
                _ => {}
            }
        }
    }

    (start, points, trees)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EVENT;

    const QUEST: u32 = 18;

    #[test]
    fn test_part1_real() {
        let input = utils::load_event_input(EVENT, QUEST, 1);
        assert_eq!(part1(&input), 101);
    }

    #[test]
    fn test_part2_real() {
        let input = utils::load_event_input(EVENT, QUEST, 2);
        assert_eq!(part2(&input), 1513);
    }

    #[test]
    fn test_part3_real() {
        let input = utils::load_event_input(EVENT, QUEST, 3);
        assert_eq!(part3(&input), 259858);
    }
}
