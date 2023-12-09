pub fn part_1(input: impl AsRef<str>) -> u32 {
    input.as_ref().lines().fold(0, |acc, line| {
        let a = line
            .chars()
            .find_map(|c| c.to_digit(10))
            .expect("Invalid input");
        let b = line
            .chars()
            .rev()
            .find_map(|c| c.to_digit(10))
            .expect("Invalid input");

        acc + (a * 10 + b)
    })
}

pub fn part_2(input: impl AsRef<str>) -> u32 {
    input.as_ref().lines().fold(0, |acc, line| {
        let mut end = 0;
        let a = line
            .chars()
            .find_map(|c| {
                c.to_digit(10).or_else(|| {
                    end += 1;
                    word_to_digit(&line[0..end])
                })
            })
            .expect("Invalid input");

        let mut start = line.len();
        let b = line
            .chars()
            .rev()
            .find_map(|c| {
                c.to_digit(10).or_else(|| {
                    start -= 1;
                    word_to_digit(&line[start..line.len()])
                })
            })
            .expect("Invalid input");

        acc + (a * 10 + b)
    })
}

fn word_to_digit(slice: &str) -> Option<u32> {
    const WORDS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    if slice.len() < 3 {
        return None;
    }

    WORDS
        .iter()
        .position(|w| slice.contains(*w))
        .map(|n| (n + 1) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_ok() {
        const INPUT: &str = include_str!("./test_1.txt");

        let result = part_1(INPUT);
        let expected = 142;

        assert_eq!(expected, result);
    }

    #[test]
    fn part_2_ok() {
        const INPUT: &str = include_str!("./test_2.txt");

        let result = part_2(INPUT);
        let expected = 281;

        assert_eq!(expected, result);
    }
}
