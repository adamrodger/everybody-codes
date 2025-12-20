use std::collections::HashMap;

#[rustfmt::skip]
pub const DELTAS: [(i32, i32); 4] = [
             (0, -1),
    (-1, 0),          (1, 0),
             (0,  1)
];

#[rustfmt::skip]
pub const DELTAS_WITH_DIAGONALS: [(i32, i32); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

pub fn solve(input: &str, deltas: &[(i32, i32)]) -> u32 {
    let mut excavations = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                // we can always excavate the entire first layer
                excavations.insert((x as i32, y as i32), 1);
            }
        }
    }

    let mut count = excavations.len() as u32;

    // keep digging until we can't dig any more
    while !excavations.is_empty() {
        let mut new_excavations = HashMap::new();

        for (&(x, y), &depth) in excavations.iter() {
            let same_depth = deltas
                .iter()
                .filter(|&d| *excavations.get(&(x + d.0, y + d.1)).unwrap_or(&0) == depth)
                .count();

            // we can excavate this square if every adjacent square is the same depth
            if same_depth == deltas.len() {
                new_excavations.insert((x, y), depth + 1);
            }
        }

        count += new_excavations.len() as u32;
        excavations = new_excavations;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EVENT;

    const QUEST: u32 = 3;

    #[test]
    fn test_part1_real() {
        let input = utils::load_event_input(EVENT, QUEST, 1);
        assert_eq!(solve(&input, &DELTAS), 131);
    }

    #[test]
    fn test_part2_real() {
        let input = utils::load_event_input(EVENT, QUEST, 2);
        assert_eq!(solve(&input, &DELTAS), 2720);
    }

    #[test]
    fn test_part3_real() {
        let input = utils::load_event_input(EVENT, QUEST, 3);
        assert_eq!(solve(&input, &DELTAS_WITH_DIAGONALS), 10112);
    }
}
