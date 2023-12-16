use itertools::Itertools;
use rayon::prelude::{ParallelBridge, ParallelIterator};
use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn beam(
    map: &[&str],
    mut pos: (usize, usize),
    dir: Dir,
    titles: &mut Vec<(usize, usize)>,
    starts: &mut HashSet<(Dir, (usize, usize))>,
) {
    match dir {
        Dir::Up if pos.0 != 0 => pos.0 -= 1,
        Dir::Down if pos.0 < map.len() - 1 => pos.0 += 1,
        Dir::Left if pos.1 != 0 => pos.1 -= 1,
        Dir::Right if pos.1 != map[0].len() - 1 => pos.1 += 1,
        _ => return,
    }

    titles.push(pos);
    if !starts.insert((dir.clone(), pos)) {
        return;
    };

    match &map[pos.0][pos.1..=pos.1] {
        "|" => match dir {
            Dir::Up | Dir::Down => beam(map, pos, dir, titles, starts),
            Dir::Left | Dir::Right => {
                beam(map, pos, Dir::Up, titles, starts);
                beam(map, pos, Dir::Down, titles, starts);
            }
        },
        "-" => match dir {
            Dir::Up | Dir::Down => {
                beam(map, pos, Dir::Left, titles, starts);
                beam(map, pos, Dir::Right, titles, starts);
            }
            Dir::Left | Dir::Right => beam(map, pos, dir, titles, starts),
        },
        r#"\"# => match dir {
            Dir::Up => beam(map, pos, Dir::Left, titles, starts),
            Dir::Down => beam(map, pos, Dir::Right, titles, starts),
            Dir::Left => beam(map, pos, Dir::Up, titles, starts),
            Dir::Right => beam(map, pos, Dir::Down, titles, starts),
        },
        r#"/"# => match dir {
            Dir::Up => beam(map, pos, Dir::Right, titles, starts),
            Dir::Down => beam(map, pos, Dir::Left, titles, starts),
            Dir::Left => beam(map, pos, Dir::Down, titles, starts),
            Dir::Right => beam(map, pos, Dir::Up, titles, starts),
        },
        "." => beam(map, pos, dir, titles, starts),
        _ => {}
    }
}

pub fn part_1<T: AsRef<str>>(input: T) -> u64 {
    let mut titles: Vec<(usize, usize)> = vec![(0, 0)];
    let mut starts: HashSet<(Dir, (usize, usize))> = HashSet::from([(Dir::Right, (0, 0))]);
    let map = input.as_ref().lines().collect_vec();

    let dir = match &map[0][0..=0] {
        "|" | r#"\"# => Dir::Down,
        "-" | "." => Dir::Right,
        r#"/"# => Dir::Up,
        _ => unreachable!(),
    };

    beam(&map, (0, 0), dir, &mut titles, &mut starts);

    titles.iter().sorted().dedup().count() as u64
}

pub fn part_2<T: AsRef<str>>(input: T) -> u64 {
    let map = input.as_ref().lines().collect_vec();

    let top = (0..map[0].len()).filter_map(|col| {
        if ["|", "."].contains(&&map[0][col..=col]) {
            Some((0, col, Dir::Down))
        } else {
            None
        }
    });
    let bot = (0..map[0].len()).filter_map(|col| {
        if ["|", "."].contains(&&map[map.len() - 1][col..=col]) {
            Some((map.len() - 1, col, Dir::Up))
        } else {
            None
        }
    });

    let left = (0..map.len()).filter_map(|row| {
        if ["-", "."].contains(&&map[row][0..=0]) {
            Some((row, 0, Dir::Right))
        } else {
            None
        }
    });
    let right = (0..map.len()).filter_map(|row| {
        if ["-", "."].contains(&&map[row][map[0].len() - 1..map[0].len()]) {
            Some((row, map[0].len() - 1, Dir::Left))
        } else {
            None
        }
    });

    top.chain(bot)
        .chain(left)
        .chain(right)
        .par_bridge()
        .map(|(row, col, dir)| {
            let mut titles: Vec<(usize, usize)> = vec![(row, col)];

            let mut starts: HashSet<(Dir, (usize, usize))> =
                HashSet::from([(dir.clone(), (row, col))]);

            beam(&map, (row, col), dir, &mut titles, &mut starts);

            titles.iter().sorted().dedup().count() as u64
        })
        .max()
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("./test.txt");

    #[test]
    fn part_1_ok() {
        let result = part_1(INPUT);
        let expected = 46;

        assert_eq!(expected, result);
    }

    #[test]
    fn part_2_ok() {
        let result = part_2(INPUT);
        let expected = 51;

        assert_eq!(expected, result);
    }
}
