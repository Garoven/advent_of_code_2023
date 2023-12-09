use itertools::Itertools;

pub fn part_1<T: AsRef<str>>(input: T) -> i64 {
    input.as_ref().lines().fold(0_i64, |mut acc, line| {
        let mut buf: Vec<i64> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i64>().ok())
            .collect_vec();

        while buf.len() != 1 {
            acc += *buf.last().unwrap_or(&0);
            buf = buf
                .into_iter()
                .tuple_windows::<(i64, i64)>()
                .map(|(x, y)| y - x)
                .collect_vec();
        }

        acc + buf[0]
    })
}

pub fn part_2<T: AsRef<str>>(input: T) -> i64 {
    input.as_ref().lines().fold(0_i64, |mut acc, line| {
        let mut buf: Vec<i64> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i64>().ok())
            .rev()
            .collect_vec();

        while buf.len() != 1 {
            acc += buf[buf.len() - 1];
            buf = buf
                .into_iter()
                .tuple_windows::<(i64, i64)>()
                .map(|(x, y)| y - x)
                .collect_vec();
        }

        acc + buf[0]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("./test.txt");

    #[test]
    fn part_1_ok() {
        let result = part_1(INPUT);
        let expected = 114;

        assert_eq!(expected, result);
    }
    #[test]
    fn part_2_ok() {
        let result = part_2(INPUT);
        let expected = 2;

        assert_eq!(expected, result);
    }
}
