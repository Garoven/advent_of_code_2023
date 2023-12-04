pub fn part_1(input: impl AsRef<str>) -> u32 {
    let mut win_numbers: Vec<u32> = Vec::with_capacity(64);
    let mut sum = 0;

    input
        .as_ref()
        .lines()
        .filter_map(|l| l.split(':').nth(1))
        .for_each(|l| {
            let mut it = l.trim().split('|');
            let new_win_numbers = it
                .next()
                .expect("Inalid input")
                .split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok());

            win_numbers.extend(new_win_numbers);

            let count = it
                .next()
                .expect("Invalid input")
                .split_whitespace()
                .filter_map(|s| match s.parse::<u32>().ok() {
                    v @ Some(n) if win_numbers.contains(&n) => v,
                    _ => None,
                })
                .count() as u32;

            win_numbers.clear();

            sum += 2_u32.pow(count.saturating_sub(1)) * count.clamp(0, 1);
        });

    sum
}

pub fn part_2(input: impl AsRef<str>) -> u32 {
    let mut win_numbers: Vec<u32> = Vec::new();
    let mut cards = std::collections::HashMap::<usize, u32>::with_capacity(512);
    let mut sum = 0;

    input
        .as_ref()
        .lines()
        .filter_map(|l| l.split(':').nth(1))
        .enumerate()
        .for_each(|(i, l)| {
            let cur_card = *cards.entry(i).or_insert(1);
            let mut it = l.trim().split('|');
            let new_win_numbers = it
                .next()
                .expect("Inalid input")
                .split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok());

            win_numbers.extend(new_win_numbers);

            let count = it
                .next()
                .expect("Invalid input")
                .split_whitespace()
                .filter_map(|s| match s.parse::<u32>().ok() {
                    v @ Some(n) if win_numbers.contains(&n) => v,
                    _ => None,
                })
                .count();

            (i + 1..=i + count).for_each(|i| *cards.entry(i).or_insert(1) += cur_card);

            win_numbers.clear();

            sum += cur_card;
        });

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("./test.txt");

    #[test]
    fn part_1_ok() {
        let result = part_1(INPUT);
        let expected = 13;

        assert_eq!(expected, result);
    }

    #[test]
    fn part_2_ok() {
        let result = part_2(INPUT);
        let expected = 30;

        assert_eq!(expected, result);
    }
}
