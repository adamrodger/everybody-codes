use std::collections::VecDeque;

pub fn part1(input: &str) -> u32 {
    let target = input.trim().parse::<u32>().unwrap();
    let mut width = 1;
    let mut blocks = 1;

    while blocks < target {
        // add a new layer to the bottom
        width += 2;
        blocks += width;
    }

    (blocks - target) * width
}

pub fn part2(input: &str) -> u32 {
    // you can observe from the sample input that each time you add a new layer,
    // the number of blocks added in each "column" is always equal to the thickness
    // of that layer. So the number of additional blocks is the width of the base
    // multiplied by the thickness of the current layer.

    let priests = input.trim().parse::<u32>().unwrap();
    let acolytes = 1111;
    let target = 20240000;

    additional_blocks(priests, acolytes, target)
}

/// Calculates the number of additional blocks needed once exceeding the available count
/// multiplied by the width of the base of the structure at that point
fn additional_blocks(priests: u32, acolytes: u32, available: u32) -> u32 {
    let mut thickness = 1;
    let mut width = 1;
    let mut blocks = 1;

    while blocks < available {
        thickness = (thickness * priests) % acolytes;

        // add a new layer
        width += 2;
        blocks += width * thickness;
    }

    (blocks - available) * width
}

pub fn part3(input: &str) -> u64 {
    let priests = input.trim().parse::<u64>().unwrap();
    let acolytes = 10;
    let target = 202400000;

    additional_blocks_2(priests, acolytes, target)
}

/// Similar to part 2, but now blocks are removed from the columns to make space for attendees.
/// The thickness calculation is also modified.
///
/// The answer is only the number of additional blocks, not multiplied by the width of the base
fn additional_blocks_2(priests: u64, acolytes: u64, available: u64) -> u64 {
    let mut thickness = 1;
    let mut width = 1;
    let mut blocks = 1;
    let mut columns = VecDeque::from(vec![1u64]);

    while blocks < available {
        thickness = ((thickness * priests) % acolytes) + acolytes;

        // add a new layer, which raises all existing columns
        width += 2;
        columns.iter_mut().for_each(|col| *col += thickness);
        columns.push_front(thickness);
        columns.push_back(thickness);

        blocks = columns.iter().sum::<u64>();

        // remove blocks from inner columes to make space for attendees
        columns
            .iter()
            .skip(1)
            .take(columns.len() - 2)
            .for_each(|col| blocks -= (width * col * priests) % acolytes);
    }

    blocks - available
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EVENT;

    const QUEST: u32 = 8;

    #[test]
    fn test_part1_real() {
        let input = utils::load_event_input(EVENT, QUEST, 1);
        assert_eq!(part1(&input), 7170779);
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(part1("13"), 21);
    }

    #[test]
    fn test_part2_real() {
        let input = utils::load_event_input(EVENT, QUEST, 2);
        assert_eq!(part2(&input), 133388862);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(additional_blocks(3, 5, 50), 27);
    }

    #[test]
    fn test_part3_real() {
        let input = utils::load_event_input(EVENT, QUEST, 3);
        assert_eq!(part3(&input), 37396);
    }

    #[test]
    fn test_part3_example() {
        assert_eq!(additional_blocks_2(2, 5, 160), 2);
    }
}
