pub fn part_1(input: impl AsRef<str>) -> u32 {
    let f = |l: &str| {
        let mut it = l.chars().filter_map(|c| c.to_digit(10));

        let a = it.next()?;
        let b = it.last().unwrap_or(a);

        Some(a * 10 + b)
    };

    input.as_ref().lines().filter_map(f).sum()
}

pub fn part_2(input: impl AsRef<str>) -> u32 {
    let mut s = String::new();
    let mut rs = String::new();

    let f = |l: &str| {
        let a = l.chars().find_map(|c| match c.to_digit(10) {
            d @ Some(_) => d,
            None => {
                s.push(c);
                word_to_digit(&mut s)
            }
        })?;

        s.clear();
        let b = l.chars().rev().find_map(|c| match c.to_digit(10) {
            d @ Some(_) => d,
            None => {
                s.push(c);
                rs.clear();
                rs.extend(s.chars().rev());
                word_to_digit(&mut rs).map(|d| {
                    s.clear();

                    d
                })
            }
        })?;

        Some(a * 10 + b)
    };

    input.as_ref().lines().filter_map(f).sum()
}

fn word_to_digit(s: &mut String) -> Option<u32> {
    const WORDS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    if s.len() < 3 {
        return None;
    }

    for (i, w) in WORDS.iter().enumerate() {
        if s.contains(w) {
            s.clear();
            return Some((i + 1) as u32);
        }
    }

    None
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
