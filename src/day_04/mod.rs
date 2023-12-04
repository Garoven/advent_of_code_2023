pub fn part_1(input: impl AsRef<str>) -> u32 {
    let mut win_numbers: Vec<u32> = Vec::with_capacity(32);
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
    let mut win_numbers: Vec<u32> = Vec::with_capacity(32);
    let mut cards: Vec<u32> = Vec::with_capacity(256);
    let mut sum = 0;

    input
        .as_ref()
        .lines()
        .filter_map(|l| l.split(':').nth(1))
        .enumerate()
        .for_each(|(i, l)| {
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

            let cur_card = match cards.get(i) {
                Some(v) => *v,
                None => {
                    cards.push(1);
                    1
                }
            };

            (i + 1..=i + count).for_each(|i| match cards.get_mut(i) {
                Some(v) => *v += cur_card,
                None => cards.push(1 + cur_card),
            });

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
