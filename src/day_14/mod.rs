use itertools::Itertools;

pub fn part_1<T: AsRef<str>>(input: T) -> u64 {
    let rows = input
        .as_ref()
        .lines()
        .rev()
        .map(|l| l.to_string())
        .collect_vec();

    (0..rows[0].len())
        .map(|col| {
            rows.iter()
                .map(|s| &s[col..=col])
                .join("")
                .split('#')
                .map(|s| s.chars().sorted().join(""))
                .join("#")
        })
        .fold(0, |acc, col| {
            col.chars()
                .enumerate()
                .fold(acc, |acc, (i, c)| if c == 'O' { acc + i + 1 } else { acc })
        }) as u64
}

pub fn part_2<T: AsRef<str>>(input: T) -> u64 {
    let mut rows = input
        .as_ref()
        .lines()
        .rev()
        .map(|l| l.to_string())
        .collect_vec();

    let mut cols = (0..rows[0].len())
        .map(|col| rows.iter().map(|s| &s[col..=col]).join(""))
        .collect_vec();

    let mut i = 1000;
    loop {
        if i != 0 {
            i -= 1;
        } else {
            return rows
                .iter()
                .enumerate()
                .map(|(i, row)| row.chars().filter(|c| *c == 'O').count() * (i + 1))
                .sum::<usize>() as u64;
        }

        cols.iter_mut().enumerate().for_each(|(i, col)| {
            *col = rows
                .iter()
                .map(|s| &s[i..=i])
                .join("")
                .split('#')
                .map(|s| s.chars().sorted().join(""))
                .join("#");
        });

        rows.iter_mut().enumerate().for_each(|(i, row)| {
            *row = cols
                .iter()
                .map(|col| &col[i..=i])
                .join("")
                .split('#')
                .map(|s| s.chars().sorted_by(|a, b| b.cmp(a)).join(""))
                .join("#")
        });

        cols.iter_mut().enumerate().for_each(|(i, col)| {
            *col = rows
                .iter()
                .map(|row| &row[i..=i])
                .join("")
                .split('#')
                .map(|s| s.chars().sorted_by(|a, b| b.cmp(a)).join(""))
                .join("#")
        });

        rows.iter_mut().enumerate().for_each(|(i, row)| {
            *row = cols
                .iter()
                .map(|col| &col[i..=i])
                .join("")
                .split('#')
                .map(|s| s.chars().sorted().join(""))
                .join("#")
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("./test.txt");

    #[test]
    fn part_1_ok() {
        let result = part_1(INPUT);
        let expected = 136;

        assert_eq!(expected, result);
    }

    #[test]
    fn part_2_ok() {
        let result = part_2(INPUT);
        let expected = 64;

        assert_eq!(expected, result);
    }
}
