use rayon::prelude::{IntoParallelIterator, ParallelIterator};

#[derive(Debug, Clone)]
struct Map {
    dst: u64,
    src: u64,
    len: u64,
}

impl std::str::FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_whitespace().filter_map(|n| n.parse::<u64>().ok());

        Ok(Self {
            dst: it.next().ok_or(())?,
            src: it.next().ok_or(())?,
            len: it.next().ok_or(())?,
        })
    }
}

pub fn part_1(input: impl AsRef<str>) -> u64 {
    let mut seeds: Vec<u64> = Vec::new();
    let mut almanac: Vec<Vec<Map>> = vec![vec![]; 7];

    input
        .as_ref()
        .lines()
        .rev()
        .fold(almanac.len(), |mut i, line| {
            if i < 1 && !line.is_empty() {
                let it = line
                    .split(':')
                    .nth(1)
                    .map(|l| l.split_whitespace())
                    .expect("Invalid input")
                    .filter_map(|string| string.parse::<u64>().ok());
                seeds.extend(it);
            } else if let Ok(map) = line.parse::<Map>() {
                almanac[i - 1].push(map);
            } else if !line.is_empty() {
                almanac[i - 1].reverse();
                i -= 1;
            }

            i
        });

    seeds.iter_mut().fold(u64::MAX, |min, seed| {
        almanac.iter().enumerate().fold(min, |min, (i, maps)| {
            maps.iter().any(|map| match seed.checked_sub(map.src) {
                Some(dif) if dif < map.len => {
                    *seed = map.dst + dif;
                    true
                }
                _ => false,
            });

            if i == almanac.len() - 1 {
                min.min(*seed)
            } else {
                min
            }
        })
    })
}

pub fn part_2(input: impl AsRef<str>) -> u64 {
    let mut seeds: Vec<(u64, u64)> = Vec::new();
    let mut almanac: Vec<Vec<Map>> = vec![vec![]; 7];

    input
        .as_ref()
        .lines()
        .rev()
        .fold(almanac.len(), |mut i, line| {
            if i < 1 && !line.is_empty() {
                let mut it = line
                    .split(':')
                    .nth(1)
                    .map(|l| l.split_whitespace())
                    .expect("Invalid input")
                    .filter_map(|string| string.parse::<u64>().ok());

                loop {
                    let Some(start) = it.next() else { break };
                    let Some(len) = it.next() else { break };

                    seeds.push((start, start + len));
                }
            } else if let Ok(map) = line.parse::<Map>() {
                almanac[i - 1].push(map);
            } else if !line.is_empty() {
                almanac[i - 1].reverse();
                i -= 1;
            }

            i
        });

    seeds
        .iter()
        .inspect(|s| println!("{s:?}"))
        .fold(u64::MAX, |min, (start, end)| {
            (*start..*end)
                .into_par_iter()
                .fold(
                    || min,
                    |min, mut seed| {
                        almanac.iter().enumerate().fold(min, |min, (i, maps)| {
                            maps.iter().any(|map| match seed.checked_sub(map.src) {
                                Some(dif) if dif < map.len => {
                                    seed = map.dst + dif;
                                    true
                                }
                                _ => false,
                            });

                            if i == almanac.len() - 1 {
                                min.min(seed)
                            } else {
                                min
                            }
                        })
                    },
                )
                .min()
                .unwrap_or_default()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("./test.txt");

    #[test]
    fn part_1_ok() {
        let result = part_1(INPUT);
        let expected = 35;

        assert_eq!(expected, result);
    }

    #[test]
    fn part_2_ok() {
        let result = part_2(INPUT);
        let expected = 46;

        assert_eq!(expected, result);
    }
}
