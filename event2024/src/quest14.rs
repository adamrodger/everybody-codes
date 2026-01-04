use core::panic;
use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
    string::ParseError,
};
use utils::grid::Point3D;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction3D {
    Left,
    Right,
    Up,
    Down,
    Forwards,
    Backwards,
}

impl Direction3D {
    fn delta(&self) -> Point3D {
        match self {
            Direction3D::Left => Point3D { x: -1, y: 0, z: 0 },
            Direction3D::Right => Point3D { x: 1, y: 0, z: 0 },
            Direction3D::Up => Point3D { x: 0, y: 1, z: 0 },
            Direction3D::Down => Point3D { x: 0, y: -1, z: 0 },
            Direction3D::Forwards => Point3D { x: 0, y: 0, z: 1 },
            Direction3D::Backwards => Point3D { x: 0, y: 0, z: -1 },
        }
    }
}

impl From<char> for Direction3D {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction3D::Left,
            'R' => Direction3D::Right,
            'U' => Direction3D::Up,
            'D' => Direction3D::Down,
            'F' => Direction3D::Forwards,
            'B' => Direction3D::Backwards,
            _ => panic!("Unexpected char"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    direction: Direction3D,
    steps: i32,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = s.chars().next().unwrap().into();
        let steps = s[1..].parse().unwrap();

        Ok(Instruction { direction, steps })
    }
}

pub fn part1(input: &str) -> u32 {
    let mut point = Point3D { x: 0, y: 0, z: 0 };
    let mut highest = 0;

    for instruction in input
        .split(',')
        .filter_map(|m| m.parse::<Instruction>().ok())
    {
        point += instruction.direction.delta() * instruction.steps;

        if instruction.direction == Direction3D::Up {
            highest = highest.max(point.y as u32);
        }
    }

    highest
}

pub fn part2(input: &str) -> usize {
    let mut segments = HashSet::new();

    for line in input.lines() {
        let mut point = Point3D { x: 0, y: 0, z: 0 };

        for instruction in line
            .split(',')
            .filter_map(|m| m.parse::<Instruction>().ok())
        {
            for _ in 0..instruction.steps {
                point += instruction.direction.delta();
                segments.insert(point);
            }
        }
    }

    segments.len()
}

pub fn part3(input: &str) -> u32 {
    let mut leaves = HashSet::new();
    let mut trunk = HashSet::new();
    let mut segments = HashSet::new();

    for line in input.lines() {
        let mut point = Point3D { x: 0, y: 0, z: 0 };

        for instruction in line
            .split(',')
            .filter_map(|m| m.parse::<Instruction>().ok())
        {
            for _ in 0..instruction.steps {
                point += instruction.direction.delta();
                segments.insert(point);

                if point.x == 0 && point.z == 0 {
                    trunk.insert(point);
                }
            }
        }

        leaves.insert(point);
    }

    let mut best = u32::MAX;

    for t in trunk {
        // walk along the branches to each leaf
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        queue.push_front((0, t));

        let mut total_distance = 0;

        while let Some((distance, current)) = queue.pop_front() {
            if !seen.insert(current) {
                continue;
            }

            if leaves.contains(&current) {
                total_distance += distance;
                continue;
            }

            for next in current.neighbours6() {
                if segments.contains(&next) {
                    queue.push_back((distance + 1, next));
                }
            }
        }

        assert_ne!(total_distance, 0);
        best = best.min(total_distance);
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EVENT;

    const QUEST: u32 = 14;

    #[test]
    fn test_part1_real() {
        let input = utils::load_event_input(EVENT, QUEST, 1);
        assert_eq!(part1(&input), 145);
    }

    #[test]
    fn test_part2_real() {
        let input = utils::load_event_input(EVENT, QUEST, 2);
        assert_eq!(part2(&input), 5178);
    }

    #[test]
    fn test_part3_real() {
        let input = utils::load_event_input(EVENT, QUEST, 3);
        assert_eq!(part3(&input), 1680);
    }

    #[test]
    fn test_part3_example_1() {
        #[rustfmt::skip]
        let input = [
            "U5,R3,D2,L5,U4,R5,D2",
            "U6,L1,D2,R3,U2,L1"
        ]
        .join("\n")
        .to_string();

        assert_eq!(part3(&input), 5);
    }

    #[test]
    fn test_part3_example_2() {
        #[rustfmt::skip]
        let input = [
            "U20,L1,B1,L2,B1,R2,L1,F1,U1",
            "U10,F1,B1,R1,L1,B1,L1,F1,R2,U1",
            "U30,L2,F1,R1,B1,R1,F2,U1,F1",
            "U25,R1,L2,B1,U1,R2,F1,L2",
            "U16,L1,B1,L1,B3,L1,B1,F1",
        ]
        .join("\n")
        .to_string();

        assert_eq!(part3(&input), 46);
    }
}
