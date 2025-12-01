fn part1(input: &str) -> u32 {
    input
        .chars()
        .map(|c| match c {
            'A' => 0,
            'B' => 1,
            'C' => 3,
            _ => unreachable!(),
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .as_bytes()
        .chunks_exact(2)
        .map(|chunk| {
            let score: u32 = chunk
                .iter()
                .map(|c| match c {
                    b'A' => 0,
                    b'B' => 1,
                    b'C' => 3,
                    b'D' => 5,
                    b'x' => 0,
                    _ => unreachable!(),
                })
                .sum();

            if chunk[0] != b'x' && chunk[1] != b'x' {
                score + 2
            } else {
                score
            }
        })
        .sum()
}

fn part3(input: &str) -> u32 {
    input
        .as_bytes()
        .chunks_exact(3)
        .map(|chunk| {
            let score: u32 = chunk
                .iter()
                .map(|c| match c {
                    b'A' => 0,
                    b'B' => 1,
                    b'C' => 3,
                    b'D' => 5,
                    b'x' => 0,
                    _ => unreachable!(),
                })
                .sum();

            let missing = chunk.iter().filter(|&c| *c == b'x').count();

            match missing {
                0 => score + 6,
                1 => score + 2,
                _ => score,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_real() {
        let input = utils::load_event_input(2024, 1, 1);
        assert_eq!(part1(&input), 1322);
    }

    #[test]
    fn test_part2_example() {
        let input = "AxBCDDCAxD";
        assert_eq!(part2(input), 28);
    }

    #[test]
    fn test_part2_real() {
        let input = utils::load_event_input(2024, 1, 2);
        assert_eq!(part2(&input), 5584);
    }

    #[test]
    fn test_part3_real() {
        let input = utils::load_event_input(2024, 1, 3);
        assert_eq!(part3(&input), 5584);
    }
}
