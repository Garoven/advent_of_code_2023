use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

fn hash(step: &str) -> u64 {
    step.chars().fold(0, |acc, ch| (acc + (ch as u64)) * 17) % 256
}

pub fn part_1<T: AsRef<str>>(input: T) -> u64 {
    input
        .as_ref()
        .trim()
        .split(',')
        .fold(0, |acc, step| acc + hash(step))
}

pub fn part_2<T: AsRef<str>>(input: T) -> u64 {
    let mut boxes: HashMap<u64, VecDeque<(&str, u64)>> = HashMap::new();

    input.as_ref().trim().split(',').for_each(|step| {
        let Some((label, lens)) = step.split_once('=') else {
            let label = &step[0..step.len() - 1];
            boxes.entry(hash(label)).and_modify(|lenses| {
                let Some(pos) = lenses.iter().find_position(|c| c.0 == label) else {
                    return;
                };

                lenses.remove(pos.0);
            });

            return;
        };

        let lens = lens.parse::<u64>().unwrap();
        let box_with_lenses = boxes.entry(hash(label)).or_default();

        if let Some((_, old_lens)) = box_with_lenses.iter_mut().find(|x| x.0 == label) {
            *old_lens = lens
        } else {
            box_with_lenses.push_back((label, lens));
        }
    });

    boxes.iter().fold(0, |acc, (box_number, lenses)| {
        lenses
            .iter()
            .enumerate()
            .fold(acc, |acc, (lens_pos, (_, lens))| {
                acc + (box_number + 1) * (lens_pos + 1) as u64 * *lens
            })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("./test.txt");

    #[test]
    fn part_1_ok() {
        let result = part_1(INPUT);
        let expected = 1320;

        assert_eq!(expected, result);
    }

    #[test]
    fn part_2_ok() {
        let result = part_2(INPUT);
        let expected = 145;

        assert_eq!(expected, result);
    }
}
