pub fn part1(input: &str) -> u32 {
    let (words, lines) = parse(input);
    let text = lines.first().unwrap();

    words
        .iter()
        .map(|&word| text.matches(word).count() as u32)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let (words, lines) = parse(input);

    lines.iter().map(|&line| matching_chars(line, &words)).sum()
}

pub fn part3(input: &str) -> u32 {
    let (_words, _lines) = parse(input);

    todo!()
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.lines();

    let words = lines
        .next()
        .unwrap()
        .trim_start_matches("WORDS:")
        .split(',')
        .collect::<Vec<_>>();

    lines.next().unwrap(); // skip blank line

    (words, lines.collect())
}

/// Count how many chars in the line match at least one of the words.
///
/// The same letter could be part of multiple words, but should only count once per match.
/// e.g. words THE and HER with input THERE would return 4 (T H E R).
///
/// The pattern is allowed to match right to left also, e.g. word THE would match EHT in the line.
fn matching_chars(line: &str, words: &[&str]) -> u32 {
    let mut flagged = (0..line.len()).map(|_| false).collect::<Vec<bool>>();

    for &word in words {
        let rev_word = word.chars().rev().collect::<String>();

        for i in 0..line.len() - word.len() + 1 {
            let segment = &line[i..i + word.len()];

            if segment == word || segment == rev_word {
                flagged
                    .iter_mut()
                    .skip(i)
                    .take(word.len())
                    .for_each(|b| *b = true);
            }
        }
    }

    flagged.iter().filter(|&&b| b).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EVENT;

    const QUEST: u32 = 2;

    #[test]
    fn test_part1_real() {
        let input = utils::load_event_input(EVENT, QUEST, 1);
        assert_eq!(part1(&input), 25);
    }

    #[test]
    fn test_part2_real() {
        let input = utils::load_event_input(EVENT, QUEST, 2);
        assert_eq!(part2(&input), 5209);
    }

    #[test]
    fn test_part3_real() {
        let input = utils::load_event_input(EVENT, QUEST, 3);
        assert_eq!(part3(&input), 0);
    }
}
