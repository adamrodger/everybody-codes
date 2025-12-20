pub fn downward_strikes(input: &str) -> u32 {
    let targets = input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let min = targets.iter().min().unwrap();

    targets.iter().map(|t| t - min).sum()
}

pub fn downward_and_upward_strikes(input: &str) -> u32 {
    let mut targets = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    targets.sort();

    let level = targets[targets.len() / 2];

    targets.iter().map(|t| t.abs_diff(level)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EVENT;

    const QUEST: u32 = 4;

    #[test]
    fn test_part1_real() {
        let input = utils::load_event_input(EVENT, QUEST, 1);
        assert_eq!(downward_strikes(&input), 72);
    }

    #[test]
    fn test_part2_real() {
        let input = utils::load_event_input(EVENT, QUEST, 2);
        assert_eq!(downward_strikes(&input), 891023);
    }

    #[test]
    fn test_part3_real() {
        let input = utils::load_event_input(EVENT, QUEST, 3);
        assert_eq!(downward_and_upward_strikes(&input), 122111051);
    }
}
