use itertools::Itertools;

/// Finds first appearance of pattern and it's length
/// -> (start_of_pattern, length_of_pattern)
fn brent<T, F>(f: F, x0: T) -> (u64, u64)
where
    T: Clone + Eq + PartialEq,
    F: Fn(T) -> T,
{
    let mut power = 1;
    let mut lam = 1;
    let mut tortoise = x0.clone();
    let mut hare = f(x0.clone());

    while tortoise != hare {
        if power == lam {
            tortoise = hare.clone();
            power *= 2;
            lam = 0;
        }

        hare = f(hare);
        lam += 1;
    }

    tortoise = x0.clone();
    hare = x0;

    for _ in 0..lam {
        hare = f(hare);
    }

    let mut mu = 1;
    while tortoise != hare {
        tortoise = f(tortoise);
        hare = f(hare);
        mu += 1;
    }

    (mu, lam)
}

fn cycle(mut rows: Vec<String>) -> Vec<String> {
    let mut cols = (0..rows[0].len())
        .map(|col| rows.iter().map(|s| &s[col..=col]).join(""))
        .collect_vec();

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

    rows
}

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

    let (skip, cycle_len) = brent(cycle, rows.clone());
    let left_cycles = (1_000_000_000 - skip) % cycle_len;
    let mut cycles = skip + cycle_len + left_cycles;
    loop {
        if cycles != 0 {
            cycles -= 1;
        } else {
            return rows
                .iter()
                .enumerate()
                .map(|(i, row)| row.chars().filter(|c| *c == 'O').count() * (i + 1))
                .sum::<usize>() as u64;
        }

        rows = cycle(rows);
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
