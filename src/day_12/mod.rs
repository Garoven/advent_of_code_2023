use std::collections::HashMap;

use itertools::Itertools;
use rayon::{prelude::ParallelIterator, str::ParallelString};

const PART_ONE: usize = 1;
const PART_TWO: usize = 5;

fn solve<'a>(
    springs: &'a str,
    groups: &[usize],
    cache: &mut HashMap<(&'a str, usize), u64>,
) -> u64 {
    match (groups.is_empty(), springs.contains('#')) {
        (true, true) => return 0,
        (true, false) => return 1,
        _ => {}
    };

    if springs.is_empty() {
        return 0;
    }

    if let Some(count) = cache.get(&(springs, groups.len())) {
        return *count;
    }

    let mut count = 0;

    if !springs[0..1].contains('#') {
        count += solve(&springs[1..], groups, cache);
    }

    if groups[0] < springs.len()
        && !springs[..groups[0]].contains('.')
        && !springs[groups[0]..=groups[0]].contains('#')
    {
        count += solve(&springs[groups[0] + 1..], &groups[1..], cache);
    }

    cache.insert((springs, groups.len()), count);

    count
}

fn both<const PART: usize>(line: &str) -> u64 {
    let (springs, groups) = line
        .split_whitespace()
        .next_tuple::<(&str, &str)>()
        .expect("Invalid input");

    let groups: Vec<usize> = groups
        .split(',')
        .filter_map(|n| n.parse().ok())
        .collect_vec()
        .repeat(PART);

    let mut springs = (springs.to_owned() + "?").repeat(PART);
    springs.replace_range(springs.len() - 1..springs.len(), ".");

    solve(&springs, &groups, &mut HashMap::new())
}

#[inline]
pub fn part_1<T: AsRef<str>>(input: T) -> u64 {
    input.as_ref().par_lines().map(both::<PART_ONE>).sum()
}

#[inline]
pub fn part_2<T: AsRef<str>>(input: T) -> u64 {
    input.as_ref().par_lines().map(both::<PART_TWO>).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("./test.txt");

    #[test]
    fn part_1_ok() {
        let result = part_1(INPUT);
        let expected = 21;

        assert_eq!(expected, result);
    }

    #[test]
    fn part_2_ok() {
        let result = part_2(INPUT);
        let expected = 525152;

        assert_eq!(expected, result);
    }
}
