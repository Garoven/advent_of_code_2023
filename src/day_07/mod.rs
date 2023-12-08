use itertools::Itertools;

const PART_ONE: usize = 1;
const PART_TWO: usize = 2;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Types {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

macro_rules! map_char {
    ( $( $char:expr ),+ ) => {
        ($( match $char {
        'A' => 'D',
        'K' => 'C',
        'Q' => 'B',
        'J' => 'A',
        'T' => ':',
        c => c,
    } ),+)
    };
}

fn map_hand<const PART: usize>(line: &str) -> Option<(&str, u32, Types)> {
    let (hand, bid) = line.split_whitespace().next_tuple()?;
    let parsed_bid = bid.parse::<u32>().ok()?;
    let mut hand_type = hand.chars().sorted().dedup_with_count().fold(
        Types::HighCard,
        |acc, (count, _)| match acc {
            Types::HighCard => match count {
                2 => Types::OnePair,
                3 => Types::ThreeOfKind,
                4 => Types::FourOfKind,
                5 => Types::FiveOfKind,
                _ => acc,
            },
            Types::OnePair => match count {
                2 => Types::TwoPair,
                3 => Types::FullHouse,
                _ => acc,
            },
            Types::ThreeOfKind => {
                if count == 2 {
                    Types::FullHouse
                } else {
                    acc
                }
            }
            _ => acc,
        },
    );

    if PART == PART_TWO {
        let count = hand.matches('J').count();
        if count != 0 {
            hand_type = match hand_type {
                Types::HighCard => Types::OnePair,
                Types::OnePair => Types::ThreeOfKind,
                Types::TwoPair => match count {
                    1 => Types::FullHouse,
                    2 => Types::FourOfKind,
                    _ => Types::TwoPair,
                },
                Types::ThreeOfKind => Types::FourOfKind,
                Types::FullHouse => Types::FiveOfKind,
                Types::FourOfKind => Types::FiveOfKind,
                Types::FiveOfKind => Types::FiveOfKind,
            }
        };
    }

    Some((hand, parsed_bid, hand_type))
}

fn sort_hands<const PART: usize>(
    (a, _, a_type): &(&str, u32, Types),
    (b, _, b_type): &(&str, u32, Types),
) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    match Types::cmp(a_type, b_type) {
        Ordering::Equal => Iterator::zip(a.chars(), b.chars())
            .filter(|(a_ch, b_ch)| a_ch != b_ch)
            .map(|(a, b)| map_char!(a, b))
            .map(|(mut a_ch, mut b_ch)| {
                if PART == PART_TWO {
                    if a_ch == 'A' {
                        a_ch = '1';
                    } else if b_ch == 'A' {
                        b_ch = '1';
                    }
                }

                a_ch.cmp(&b_ch)
            })
            .next()
            .unwrap_or(Ordering::Equal),
        ord => ord,
    }
}

pub fn part_1(input: impl AsRef<str>) -> u32 {
    input
        .as_ref()
        .lines()
        .filter_map(map_hand::<PART_ONE>)
        .sorted_by(sort_hands::<PART_ONE>)
        .enumerate()
        .fold(0_u32, |acc, (rank, (_, bid, _))| {
            acc + bid * (rank + 1) as u32
        })
}

pub fn part_2(input: impl AsRef<str>) -> u32 {
    input
        .as_ref()
        .lines()
        .filter_map(map_hand::<PART_TWO>)
        .sorted_by(sort_hands::<PART_TWO>)
        .enumerate()
        .fold(0_u32, |acc, (rank, (_, bid, _))| {
            acc + bid * (rank + 1) as u32
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("./test.txt");

    #[test]
    fn part_1_ok() {
        let result = part_1(INPUT);
        let expected = 6592;

        assert_eq!(expected, result);
    }

    #[test]
    fn part_2_ok() {
        let result = part_2(INPUT);
        let expected = 6839;

        assert_eq!(expected, result);
    }
}
