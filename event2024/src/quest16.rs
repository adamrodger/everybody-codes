use itertools::Itertools;
use num::Integer;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct Wheel {
    pub moves: usize,
    pub faces: Vec<String>,
}

pub fn part1(input: &str) -> String {
    let wheels = parse(input);

    wheels
        .iter()
        .map(|w| w.faces[(100 * w.moves) % w.faces.len()].as_str())
        .join(" ")
}

pub fn part2(input: &str, pulls: u64) -> u64 {
    let wheels = parse(input);

    let mut counts: HashMap<u8, u64> = HashMap::new();
    let mut rounds = Vec::new();
    let mut seen = HashSet::new();

    loop {
        counts.clear();

        let faces = wheels
            .iter()
            .map(|w| w.faces[((rounds.len() + 1) * w.moves) % w.faces.len()].as_str())
            .join(" ");

        if !seen.insert(faces.clone()) {
            // loop found
            break;
        }

        let coins = score(&faces, &mut counts);
        rounds.push(coins);
    }

    let loop_total: u64 = rounds.iter().sum();

    let (loops, rem) = pulls.div_rem(&(seen.len() as u64));

    (loops * loop_total) + rounds.iter().take(rem as usize).sum::<u64>()
}

pub fn part3(input: &str) -> String {
    let wheels = parse(input);

    fn solve(
        nudges: i64,
        pulls: usize,
        remaining: usize,
        cache: &mut HashMap<(i64, usize, usize), (u64, u64)>,
        wheels: &[Wheel],
    ) -> (u64, u64) {
        let key = (nudges, pulls, remaining);

        if let Some(&res) = cache.get(&key) {
            return res;
        }

        let faces = wheels
            .iter()
            .map(|w| {
                let i: i64 = (pulls as i64 * w.moves as i64 + nudges) % (w.faces.len() as i64);
                w.faces[i as usize].as_str()
            })
            .join(" ");

        let score = if pulls == 0 {
            0
        } else {
            score(&faces, &mut HashMap::new())
        };

        let result = if remaining > 0 {
            let mut max_max = 0u64;
            let mut min_min = u64::MAX;

            for &offset in &[-1, 0, 1] {
                let (max, min) = solve(nudges + offset, pulls + 1, remaining - 1, cache, wheels);

                max_max = max_max.max(max);
                min_min = min_min.min(min);
            }

            (score + max_max, score + min_min)
        } else {
            (score, score)
        };

        cache.insert(key, result);
        result
    }

    let (max, min) = solve(0, 0, 256, &mut HashMap::new(), &wheels);
    format!("{} {}", max, min)
}

fn parse(input: &str) -> Vec<Wheel> {
    let mut lines = input.lines();

    let mut wheels: Vec<Wheel> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|m| Wheel {
            moves: m.parse().unwrap(),
            faces: Vec::new(),
        })
        .collect();

    // skip blank line
    lines.next().unwrap();

    for line in lines {
        let segments = line
            .as_bytes()
            .chunks(4)
            .map(|c| str::from_utf8(c).unwrap())
            .map(str::trim);

        for (i, segment) in segments.enumerate() {
            if segment.chars().all(|c| c.is_ascii_whitespace()) {
                continue;
            }

            wheels[i].faces.push(segment.to_owned());
        }
    }

    wheels
}

fn score(faces: &str, counts: &mut HashMap<u8, u64>) -> u64 {
    let b = faces.as_bytes();

    // count every other char
    for i in (0..b.len()).step_by(2) {
        counts.entry(b[i]).and_modify(|c| *c += 1).or_insert(1);
    }

    counts.values().map(|c| c.saturating_sub(2)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EVENT;

    const QUEST: u32 = 16;

    #[test]
    fn test_part1_real() {
        let input = utils::load_event_input(EVENT, QUEST, 1);
        assert_eq!(part1(&input), "^,- ^,- >.^ >.^");
    }

    #[test]
    fn test_part2_real() {
        let input = utils::load_event_input(EVENT, QUEST, 2);
        assert_eq!(part2(&input, 202420242024), 136592882720);
    }

    #[test]
    fn test_part3_real() {
        let input = utils::load_event_input(EVENT, QUEST, 3);
        assert_eq!(part3(&input), "602 76");
    }
}
