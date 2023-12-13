use itertools::Itertools;

const PART_ONE: usize = 1;
const PART_TWO: usize = 2;

fn calc<const PART: usize, T: AsRef<str>>(slice: &[T]) -> Option<u64> {
    (0..slice.len() - 1)
        .find_map(|line| {
            let mut count = 0;
            let conditon = (0..=line).all(|offset| {
                if offset + line + 1 >= slice.len() {
                    return true;
                }

                let a = slice[line - offset].as_ref();
                let b = (slice[line + offset + 1]).as_ref();

                if PART == 2 {
                    count += a.chars().zip(b.chars()).filter(|(a, b)| a != b).count();
                }

                match PART {
                    1 => a == b,
                    _ => true,
                }
            });

            if PART == 1 && conditon || PART == 2 && count == 1 {
                Some(line + 1)
            } else {
                None
            }
        })
        .map(|val| val as u64)
}

fn both<const PART: usize, T: AsRef<str>>(input: T) -> u64 {
    input
        .as_ref()
        .lines()
        .group_by(|line| !line.is_empty())
        .into_iter()
        .map(|a| a.1.collect_vec())
        .filter(|v| !v[0].is_empty())
        .map(|rows| {
            calc::<PART, _>(&rows).map_or_else(
                || {
                    let cols = (0..rows[0].len())
                        .map(|i| rows.iter().map(|row| &row[i..=i]).join(""))
                        .collect_vec();

                    calc::<PART, _>(&cols).unwrap_or_default()
                },
                |val| val * 100,
            )
        })
        .sum()
}

pub fn part_1<T: AsRef<str>>(input: T) -> u64 {
    both::<PART_ONE, T>(input)
}

pub fn part_2<T: AsRef<str>>(input: T) -> u64 {
    both::<PART_TWO, T>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("./test.txt");

    #[test]
    fn part_1_ok() {
        let result = part_1(INPUT);
        let expected = 405;

        assert_eq!(expected, result);
    }

    #[test]
    fn part_2_ok() {
        let result = part_2(INPUT);
        let expected = 400;

        assert_eq!(expected, result);
    }
}
