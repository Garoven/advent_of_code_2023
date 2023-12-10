use itertools::Itertools;

const PART_ONE: usize = 1;
const PART_TWO: usize = 2;

enum Direction {
    Up,
    Left,
    Right,
    Down,
}

fn both<const PART: usize, T: AsRef<str>>(input: T) -> u64 {
    let pipe_map = input
        .as_ref()
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let (start_c, start_r) = pipe_map
        .iter()
        .flatten()
        .position(|c| c == &'S')
        .map(|p| (p / pipe_map.len(), p % pipe_map[0].len()))
        .expect("Input does not contain S");

    let u = pipe_map[start_c.saturating_sub(1)][start_r];
    let l = pipe_map[start_c][start_r.saturating_sub(1)];

    let (mut c, mut r, mut dir) = if u == '|' || u == 'F' || u == '7' {
        (start_c - 1, start_r, Direction::Up)
    } else if l == '-' || l == 'L' || l == 'F' {
        (start_c, start_r - 1, Direction::Left)
    } else {
        (start_c, start_r + 1, Direction::Right)
    };

    let mut count = 0;
    let mut points = vec![(start_c, start_r), (c, r)];
    while pipe_map[c][r] != 'S' {
        match pipe_map[c][r] {
            '|' => match dir {
                Direction::Up => c -= 1,
                Direction::Down => c += 1,
                _ => unreachable!(),
            },
            '-' => match dir {
                Direction::Left => r -= 1,
                Direction::Right => r += 1,
                _ => unreachable!(),
            },
            'F' => match dir {
                Direction::Up => (dir, r) = (Direction::Right, r + 1),
                Direction::Left => (dir, c) = (Direction::Down, c + 1),
                _ => unreachable!(),
            },
            '7' => match dir {
                Direction::Up => (dir, r) = (Direction::Left, r - 1),
                Direction::Right => (dir, c) = (Direction::Down, c + 1),
                _ => unreachable!(),
            },
            'L' => match dir {
                Direction::Down => (dir, r) = (Direction::Right, r + 1),
                Direction::Left => (dir, c) = (Direction::Up, c - 1),
                _ => unreachable!(),
            },
            'J' => match dir {
                Direction::Down => (dir, r) = (Direction::Left, r - 1),
                Direction::Right => (dir, c) = (Direction::Up, c - 1),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
        if pipe_map[c][r] != '|' || pipe_map[c][r] != '-' {
            points.push((c, r));
        }
        count += 1;
    }

    if PART == PART_ONE {
        count / 2 + count % 2
    } else {
        points
            .iter()
            .tuple_windows()
            .fold(0_i64, |acc, (a, b)| {
                acc + (a.0 * b.1) as i64 - (a.1 * b.0) as i64
            })
            .unsigned_abs()
            .div_euclid(2)
            .saturating_sub(count / 2 + count % 2 - 1)
    }
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
        let expected = 8;

        assert_eq!(expected, result);
    }

    #[test]
    fn part_2_ok() {
        let result = part_2(INPUT);
        let expected = 1;

        assert_eq!(expected, result);
    }
}
