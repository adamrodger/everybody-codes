use std::collections::{HashMap, HashSet};

const NUM_COLUMNS: usize = 4;

pub fn part1(input: &str) -> u64 {
    let mut columns = parse_input(input);

    for round in 0..10 {
        simulate_round(&mut columns, round);
    }

    shout(&columns)
}

pub fn part2(input: &str) -> u64 {
    let mut columns = parse_input(input);
    let mut seen = HashMap::new();

    (0usize..)
        .find_map(|round| {
            simulate_round(&mut columns, round);

            let shout = shout(&columns);
            let count = seen.entry(shout).and_modify(|e| *e += 1).or_insert(1);

            if *count == 2024 {
                Some((round as u64 + 1) * shout)
            } else {
                None
            }
        })
        .unwrap()
}

pub fn part3(input: &str) -> u64 {
    (0usize..)
        .scan(
            (parse_input(input), HashSet::new()),
            |(columns, seen), round| {
                simulate_round(columns, round);

                let shout = shout(columns);

                if seen.insert(columns.clone()) {
                    Some(shout)
                } else {
                    // entered a cycle
                    None
                }
            },
        )
        .max()
        .unwrap()
}

/// Parse the input into columns of clappers
fn parse_input(input: &str) -> Vec<Vec<u64>> {
    let mut columns: Vec<Vec<u64>> = (0..NUM_COLUMNS).map(|_| Vec::new()).collect();

    for line in input.lines() {
        for (i, c) in line.split_ascii_whitespace().enumerate() {
            columns[i].push(c.parse().unwrap());
        }
    }

    columns
}

/// Simulate a single round of clapping
fn simulate_round(columns: &mut [Vec<u64>], round: usize) {
    let clapper = columns[round % NUM_COLUMNS].remove(0);

    let line = &mut columns[(round + 1) % NUM_COLUMNS];

    // Work out how we walk around the line to decide where to join the line.
    // The clapper walks around the back of the line and comes back.
    // Use `rem_euclid` to avoid negative values instead of taking an absolute value.
    let cycle = line.len() * 2;
    let moves = clapper as usize % cycle;

    let mut index = if moves == 0 {
        // exactly walked there and back
        1
    } else {
        moves - 1
    };

    if index > line.len() {
        // walking backwards
        index = cycle - index;
    }

    line.insert(index, clapper);
}

/// Shout out the numbers at the front of each column
fn shout(columns: &[Vec<u64>]) -> u64 {
    columns
        .iter()
        .map(|c| c[0].to_string())
        .collect::<String>()
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EVENT;

    const QUEST: u32 = 5;

    #[test]
    fn test_part1_real() {
        let input = utils::load_event_input(EVENT, QUEST, 1);
        assert_eq!(part1(&input), 2322);
    }

    #[test]
    fn test_part2_real() {
        let input = utils::load_event_input(EVENT, QUEST, 2);
        assert_eq!(part2(&input), 19241503317936);
    }

    #[test]
    fn test_part3_real() {
        let input = utils::load_event_input(EVENT, QUEST, 3);
        assert_eq!(part3(&input), 9342100410031003);
    }
}
