pub fn part1(input: &str) -> u32 {
    part2(input)
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .rev()
        .skip(1)
        .enumerate()
        .map(|(height, line)| {
            line.chars()
                .skip(1)
                .enumerate()
                .map(|(width, c)| {
                    if c == 'T' {
                        let power = (width + height) / 3;
                        let start = (width + height) % 3;
                        (start as u32 + 1) * power as u32
                    } else if c == 'H' {
                        let power = (width + height) / 3;
                        let start = (width + height) % 3;
                        2 * ((start as u32 + 1) * power as u32)
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum()
}

pub fn part3(input: &str) -> i32 {
    // basically stole this one from the solution thread
    input
        .lines()
        .map(|line| {
            let split = line.split_once(' ').unwrap();
            (
                split.0.parse::<i32>().unwrap(),
                split.1.parse::<i32>().unwrap(),
            )
        })
        .map(|(x, y)| intercept(x / 2, y - (x / 2) - (x % 2)))
        .sum()
}

fn intercept(x: i32, y: i32) -> i32 {
    for start in 0..3 {
        let y = y - start;

        if x < y {
            continue;
        }

        if x <= 2 * y {
            return (start + 1) * y;
        }

        if (x + y) % 3 == 0 {
            return (start + 1) * ((x + y) / 3);
        }
    }

    unreachable!("Unable to hit target")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EVENT;

    const QUEST: u32 = 12;

    #[test]
    fn test_part1_real() {
        let input = utils::load_event_input(EVENT, QUEST, 1);
        assert_eq!(part1(&input), 193);
    }

    #[test]
    fn test_part1_example() {
        #[rustfmt::skip]
        let input = [
            ".............",
            ".C...........",
            ".B......T....",
            ".A......T.T..",
            "=============",
        ]
        .join("\n")
        .to_string();

        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part2_real() {
        let input = utils::load_event_input(EVENT, QUEST, 2);
        assert_eq!(part2(&input), 21994);
    }

    #[test]
    fn test_part3_real() {
        let input = utils::load_event_input(EVENT, QUEST, 3);
        assert_eq!(part3(&input), 726080);
    }

    #[test]
    fn test_part3_example() {
        let input = ["6 5", "6 7", "10 5"].join("\n").to_string();
        assert_eq!(part3(&input), 11);
    }
}
