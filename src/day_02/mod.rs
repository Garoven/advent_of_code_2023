pub fn part_1(input: impl AsRef<str>) -> u32 {
    let f = |l: &str| {
        let mut it = l.split(':');
        let id = it
            .next()
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|c| c.parse::<u32>().ok());

        it.next().and_then(|l| {
            let b = l.split(';').any(|g| {
                g.split(',').any(|c| {
                    let mut it = c.split_whitespace();
                    let v = it.next().and_then(|v| v.parse().ok()).unwrap_or(0);
                    match it.next() {
                        Some("red") => v > 12,
                        Some("green") => v > 13,
                        Some("blue") => v > 14,
                        _ => unreachable!(),
                    }
                })
            });

            if b {
                None
            } else {
                id
            }
        })
    };

    input.as_ref().lines().filter_map(f).sum()
}

pub fn part_2(input: impl AsRef<str>) -> u32 {
    let f = |l: &str| {
        let mut it = l.split(':');
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        it.nth(1).map(|l| {
            l.split(';').for_each(|g| {
                g.split(',').for_each(|c| {
                    let mut it = c.split_whitespace();
                    let v = it.next().and_then(|v| v.parse().ok()).unwrap_or(0);
                    match it.next() {
                        Some("red") => red = red.max(v),
                        Some("green") => green = green.max(v),
                        Some("blue") => blue = blue.max(v),
                        _ => unreachable!(),
                    };
                })
            });

            red * green * blue
        })
    };

    input.as_ref().lines().filter_map(f).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("./test.txt");

    #[test]
    fn part_1_ok() {
        let result = part_1(INPUT);
        let expected = 8;

        assert_eq!(expected, result);
    }

    #[test]
    fn part_2_ok() {
        let result = part_2(INPUT);
        let expected = 2286;

        assert_eq!(expected, result);
    }
}
