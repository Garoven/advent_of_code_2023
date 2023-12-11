use std::thread::available_parallelism;

use itertools::Itertools;
use rayon::{prelude::ParallelIterator, slice::ParallelSlice};

const PART_ONE: usize = 1;
const PART_TWO: usize = 999999;

fn both<const PART: usize, T: AsRef<str>>(input: T) -> u64 {
    let space = input
        .as_ref()
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let expanded_rows = space
        .iter()
        .positions(|row| !row.contains(&'#'))
        .collect_vec();

    let expanded_cols = (0..space[0].len())
        .positions(|i| !space.iter().map(|v| v[i]).any(|c| c == '#'))
        .collect_vec();

    let galaxies = space
        .iter()
        .flatten()
        .positions(|c| *c == '#')
        .map(|pos| (pos / space.len(), pos % space[0].len()))
        .collect_vec();

    let galaxies_pairs = galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, gal)| galaxies[i + 1..].iter().map(|other_gal| (*gal, other_gal)))
        .collect_vec();

    let available_cpus = available_parallelism()
        .map_or_else(|_| 1, |c| c.get())
        .max(8);
    let chunk_len = (galaxies_pairs.len() / available_cpus).max(1);

    galaxies_pairs
        .par_chunks(chunk_len)
        .fold(
            || 0,
            |acc, chunk| {
                chunk.iter().fold(0, |acc, (a, b)| {
                    let rows_count = expanded_rows
                        .iter()
                        .filter(|row| a.0.min(b.0) <= **row && **row < a.0.max(b.0))
                        .count();

                    let cols_count = expanded_cols
                        .iter()
                        .filter(|col| a.1.min(b.1) <= **col && **col < a.1.max(b.1))
                        .count();

                    acc + b.0.abs_diff(a.0)
                        + b.1.abs_diff(a.1)
                        + rows_count * PART
                        + cols_count * PART
                }) as u64
                    + acc
            },
        )
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
        let expected = 374;

        assert_eq!(expected, result);
    }

    #[test]
    fn part_2_ok() {
        let result = part_2(INPUT);
        let expected = 82000210;

        assert_eq!(expected, result);
    }
}
