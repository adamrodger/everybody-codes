use std::collections::HashMap;

pub fn part1(input: &str) -> u32 {
    const STAMPS: [u32; 4] = [1, 3, 5, 10];

    let mut cache = HashMap::new();

    input
        .lines()
        .filter_map(|line| line.parse::<u32>().ok())
        .map(|t| fewest_stamps(t, &STAMPS, &mut cache))
        .sum::<u32>()
}

pub fn part2(input: &str) -> u32 {
    const STAMPS: [u32; 10] = [1, 3, 5, 10, 15, 16, 20, 24, 25, 30];

    let mut cache = HashMap::new();

    input
        .lines()
        .filter_map(|line| line.parse::<u32>().ok())
        .map(|t| fewest_stamps(t, &STAMPS, &mut cache))
        .sum::<u32>()
}

pub fn part3(input: &str) -> u32 {
    const STAMPS: [u32; 18] = [
        1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101,
    ];

    let mut cache = HashMap::new();

    input
        .lines()
        .filter_map(|line| line.parse::<u32>().ok())
        .map(|t| split_and_check(t, &STAMPS, &mut cache))
        .sum::<u32>()
}

/// Find the fewest stamps needed to get to 0 from the current target
///
/// Assumption: it's always possible to get to 0 with the given stamps
fn fewest_stamps(target: u32, stamps: &[u32], cache: &mut HashMap<u32, u32>) -> u32 {
    if target == 0 {
        return 0;
    }

    if cache.contains_key(&target) {
        return cache[&target];
    }

    let mut best = u32::MAX;

    for &stamp in stamps {
        if stamp > target {
            continue;
        }

        let needed = 1 + fewest_stamps(target - stamp, stamps, cache);
        best = best.min(needed);
    }

    cache.insert(target, best);
    best
}

/// split the number into two parts which are within 100 of each other, and for each pair
/// find the optimal solution
fn split_and_check(target: u32, stamps: &[u32], cache: &mut HashMap<u32, u32>) -> u32 {
    let mut bottom = target / 2;
    let mut top = target - bottom;
    let mut best = u32::MAX;

    while top - bottom < 101 {
        let total = fewest_stamps(bottom, stamps, cache) + fewest_stamps(top, stamps, cache);
        best = best.min(total);

        bottom -= 1;
        top += 1;
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EVENT;

    const QUEST: u32 = 9;

    #[test]
    fn test_part1_real() {
        let input = utils::load_event_input(EVENT, QUEST, 1);
        assert_eq!(part1(&input), 13266);
    }

    #[test]
    fn test_part2_real() {
        let input = utils::load_event_input(EVENT, QUEST, 2);
        assert_eq!(part2(&input), 5077);
    }

    #[test]
    fn test_part2_example() {
        let input = "33\n41\n55\n99";
        assert_eq!(part2(input), 10);
    }

    #[test]
    fn test_part3_real() {
        // this is just cheeky... make the stack bigger so we can actually run this
        // surprised this even runs fast enough to be feasible (<1s on my machine)
        let thread = std::thread::Builder::new()
            .name("q09p3".into())
            .stack_size(8 * 1024 * 1024)
            .spawn(|| {
                let input = utils::load_event_input(EVENT, QUEST, 3);
                part3(&input)
            })
            .unwrap();

        let result = thread.join().unwrap();
        assert_eq!(result, 146301);
    }
}
