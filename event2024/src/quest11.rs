use std::collections::HashMap;

pub fn part1(input: &str) -> u64 {
    let rules = get_rules(input);
    let mut cache = HashMap::new();
    calculate("A", 4, &rules, &mut cache)
}

pub fn part2(input: &str) -> u64 {
    let rules = get_rules(input);
    let mut cache = HashMap::new();
    calculate("Z", 10, &rules, &mut cache)
}

pub fn part3(input: &str) -> u64 {
    let rules = get_rules(input);
    let mut cache = HashMap::new();

    let counts: Vec<u64> = rules
        .keys()
        .map(|&k| calculate(k, 20, &rules, &mut cache))
        .collect();

    counts.iter().max().unwrap() - counts.iter().min().unwrap()
}

/// Parse the termite evolution rules
fn get_rules(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let (head, tail) = line.split_once(':').unwrap();
            (head, tail.split(',').collect())
        })
        .collect()
}

/// Calculate how many termites we'll have from the current termite after the
/// given number of days, by following the evolution rules
fn calculate<'a>(
    current: &'a str,
    day: u32,
    rules: &'a HashMap<&str, Vec<&str>>,
    cache: &mut HashMap<(&'a str, u32), u64>,
) -> u64 {
    if day == 0 {
        return 1;
    }

    let key = (current, day);

    if cache.contains_key(&key) {
        return cache[&key];
    }

    let next = rules[current]
        .iter()
        .map(|&c| calculate(c, day - 1, rules, cache))
        .sum();

    cache.insert(key, next);

    next
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EVENT;

    const QUEST: u32 = 11;

    #[test]
    fn test_part1_real() {
        let input = utils::load_event_input(EVENT, QUEST, 1);
        assert_eq!(part1(&input), 41);
    }

    #[test]
    fn test_part2_real() {
        let input = utils::load_event_input(EVENT, QUEST, 2);
        assert_eq!(part2(&input), 219124);
    }

    #[test]
    fn test_part3_real() {
        let input = utils::load_event_input(EVENT, QUEST, 3);
        assert_eq!(part3(&input), 1039745269860);
    }

    #[test]
    fn test_part3_example() {
        let input = r#"A:B,C
B:C,A,A
C:A"#;
        assert_eq!(part3(input), 268815);
    }
}
