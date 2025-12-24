use rayon::prelude::*;
use std::collections::{BTreeMap, HashSet};

pub fn part1(input: &str) -> String {
    let plans = input
        .lines()
        .map(|line| {
            line.split_once(':')
                .map(|(name, rest)| (name, rest.split(',').collect::<Vec<&str>>()))
                .unwrap()
        })
        .collect::<Vec<(&str, Vec<&str>)>>();

    let results: BTreeMap<u32, &str> =
        plans
            .par_iter()
            .map(|(id, instructions)| {
                let result = instructions.iter().cycle().take(10).fold(
                    (10u32, 0u32),
                    |(power, acc), &inst| {
                        let new_power = match inst {
                            "+" => power + 1,
                            "-" => power.saturating_sub(1),
                            "=" => power,
                            _ => unreachable!(),
                        };

                        (new_power, acc + new_power)
                    },
                );

                (result.1, *id)
            })
            .collect();

    results
        .into_iter()
        .rev()
        .map(|(_, id)| id)
        .collect::<String>()
}

pub fn part2(input: &str, track: &str) -> String {
    const ROUNDS: usize = 10;

    let plans = input
        .lines()
        .map(|line| {
            line.split_once(':')
                .map(|(name, rest)| (name, rest.split(',').collect::<Vec<&str>>()))
                .unwrap()
        })
        .collect::<Vec<(&str, Vec<&str>)>>();

    let track_loop = get_track_loop(track);

    let results: BTreeMap<u64, &str> = plans
        .par_iter()
        .map(|(id, instructions)| {
            let total = score(&track_loop, instructions, ROUNDS);
            (total, *id)
        })
        .collect();

    results
        .into_iter()
        .rev()
        .map(|(_, id)| id)
        .collect::<String>()
}

pub fn part3(input: &str, track: &str) -> u32 {
    const ROUNDS: usize = 2024;

    let opponent = input
        .lines()
        .map(|line| {
            line.split_once(':')
                .map(|(name, rest)| (name, rest.split(',').collect::<Vec<&str>>()))
                .unwrap()
        })
        .next()
        .unwrap();

    let track_loop = get_track_loop(track);

    let opponent_score = score(&track_loop, &opponent.1, ROUNDS);

    possible_plans()
        .par_iter()
        .filter_map(|plan| {
            let my_score = score(&track_loop, plan, ROUNDS);

            if my_score > opponent_score {
                Some(())
            } else {
                None
            }
        })
        .count() as u32
}

/// Extract the track loop from the given track string
///
/// The track could be completely jagged (see part 3), thus we need to follow it
/// around to extract the loop from (1, 0) until we reach the S node
fn get_track_loop(track: &str) -> String {
    const DIRECTIONS: &[(isize, isize)] = &[(1, 0), (0, 1), (0, -1), (-1, 0)]; // RDUL

    // collect to a grid of char first to make indexing easier
    let grid: Vec<Vec<char>> = track.lines().map(|line| line.chars().collect()).collect();
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    let mut path = String::new();
    let mut visited = HashSet::new();
    let mut current = grid[0][1];
    let mut x = 1_isize;
    let mut y = 0_isize;

    while current != 'S' {
        path.push(current);
        visited.insert((x, y));

        for &(dx, dy) in DIRECTIONS {
            let nx = x + dx;
            let ny = y + dy;

            if ny >= 0
                && ny < height
                && nx >= 0
                && nx < width
                && grid[ny as usize][nx as usize] != ' '
                && !visited.contains(&(nx, ny))
            {
                current = grid[ny as usize][nx as usize];
                x = nx;
                y = ny;
                break;
            }
        }
    }

    path.push('S');
    path
}

/// Score the given instructions on the track loop for the specified number of rounds
fn score(track_loop: &str, instructions: &[&str], rounds: usize) -> u64 {
    let mut power = 10u64;
    let mut total = 0u64;

    for (t, i) in track_loop
        .chars()
        .cycle()
        .take(track_loop.len() * rounds)
        .zip(instructions.iter().cycle())
    {
        match t {
            '+' => power += 1,
            '-' => power = power.saturating_sub(1),
            '=' | 'S' => match *i {
                "+" => power += 1,
                "-" => power = power.saturating_sub(1),
                "=" => {}
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }

        total += power;
    }

    total
}

/// Generate all possible plans of length 11 with a total of 5 + moves, 3 - moves, and 3 = moves
/// in all possible combinations
fn possible_plans() -> Vec<Vec<&'static str>> {
    let mut plans = Vec::new();

    fn backtrack(
        plan: &mut Vec<&'static str>,
        plus: usize,
        minus: usize,
        equal: usize,
        plans: &mut Vec<Vec<&'static str>>,
    ) {
        if plan.len() == 11 {
            if plus == 5 && minus == 3 && equal == 3 {
                plans.push(plan.clone());
            }
            return;
        }

        if plus < 5 {
            plan.push("+");
            backtrack(plan, plus + 1, minus, equal, plans);
            plan.pop();
        }

        if minus < 3 {
            plan.push("-");
            backtrack(plan, plus, minus + 1, equal, plans);
            plan.pop();
        }

        if equal < 3 {
            plan.push("=");
            backtrack(plan, plus, minus, equal + 1, plans);
            plan.pop();
        }
    }

    backtrack(&mut Vec::new(), 0, 0, 0, &mut plans);
    plans
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EVENT;

    const QUEST: u32 = 7;

    #[test]
    fn test_part1_real() {
        let input = utils::load_event_input(EVENT, QUEST, 1);
        assert_eq!(part1(&input), "KFDJBAHCE");
    }

    #[test]
    fn test_part1_example() {
        let input = r#"A:+,-,=,=
B:+,=,-,+
C:=,-,+,+
D:=,=,=,+"#;

        assert_eq!(part1(input), "BDCA");
    }

    #[test]
    fn test_part2_real() {
        let tracx = [
            "S-=++=-==++=++=-=+=-=+=+=--=-=++=-==++=-+=-=+=-=+=+=++=-+==++=++=-=-=--",
            "-                                                                     -",
            "=                                                                     =",
            "+                                                                     +",
            "=                                                                     +",
            "+                                                                     =",
            "=                                                                     =",
            "-                                                                     -",
            "--==++++==+=+++-=+=-=+=-+-=+-=+-=+=-=+=--=+++=++=+++==++==--=+=++==+++-",
        ]
        .join("\n")
        .to_string();

        let input = utils::load_event_input(EVENT, QUEST, 2);
        assert_eq!(part2(&input, &tracx), "AIDGJCEBH");
    }

    #[test]
    fn test_part2_example() {
        let input = r#"A:+,-,=,=
B:+,=,-,+
C:=,-,+,+
D:=,=,=,+"#;

        let track = r#"S+===
-   +
=+=-+"#;

        assert_eq!(part2(input, track), "DCBA");
    }

    #[test]
    fn test_part3_real() {
        let track = [
            "S+= +=-== +=++=     =+=+=--=    =-= ++=     +=-  =+=++=-+==+ =++=-=-=--",
            "- + +   + =   =     =      =   == = - -     - =  =         =-=        -",
            "= + + +-- =-= ==-==-= --++ +  == == = +     - =  =    ==++=    =++=-=++",
            "+ + + =     +         =  + + == == ++ =     = =  ==   =   = =++=       ",
            "= = + + +== +==     =++ == =+=  =  +  +==-=++ =   =++ --= + =          ",
            "+ ==- = + =   = =+= =   =       ++--          +     =   = = =--= ==++==",
            "=     ==- ==+-- = = = ++= +=--      ==+ ==--= +--+=-= ==- ==   =+=    =",
            "-               = = = =   +  +  ==+ = = +   =        ++    =          -",
            "-               = + + =   +  -  = + = = +   =        +     =          -",
            "--==++++==+=+++-= =-= =-+-=  =+-= =-= =--   +=++=+++==     -=+=++==+++-",
        ]
        .join("\n")
        .to_string();

        let input = utils::load_event_input(EVENT, QUEST, 3);
        assert_eq!(part3(&input, &track), 3702);
    }
}
