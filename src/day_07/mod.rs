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

macro_rules! set_type {
    ($hand:expr, $count: expr) => {
        $hand = match $hand {
            Types::HighCard => match $count {
                2 => Types::OnePair,
                3 => Types::ThreeOfKind,
                4 => Types::FourOfKind,
                5 => Types::FiveOfKind,
                _ => $hand,
            },
            Types::OnePair => match $count {
                2 => Types::TwoPair,
                3 => Types::FullHouse,
                _ => $hand,
            },
            Types::ThreeOfKind => {
                if $count == 2 {
                    Types::FullHouse
                } else {
                    $hand
                }
            }
            hand => hand,
        }
    };
}

macro_rules! use_jokers {
    ($hand:expr, $count: expr) => {
        if $count != 0 {
            $hand = match $hand {
                Types::HighCard => Types::OnePair,
                Types::OnePair => Types::ThreeOfKind,
                Types::TwoPair => match $count {
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
    };
}

fn sort_hands<const PART: usize>(a: &(&str, u32), b: &(&str, u32)) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    let mut cards = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];

    if PART == PART_TWO {
        cards = [
            'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
        ];
    };

    let mut a_type = Types::HighCard;
    let mut b_type = Types::HighCard;

    for card in cards {
        let a_count = a.0.matches(card).count();
        let b_count = b.0.matches(card).count();

        set_type!(a_type, a_count);
        set_type!(b_type, b_count);
    }

    if PART == PART_TWO {
        let a_count = a.0.matches('J').count();
        let b_count = b.0.matches('J').count();

        use_jokers!(a_type, a_count);
        use_jokers!(b_type, b_count);
    }

    let mut order = a_type.cmp(&b_type);

    if order == Ordering::Equal {
        'outer: for (a_c, b_c) in a.0.chars().zip(b.0.chars()) {
            if a_c == b_c {
                continue;
            }

            for card in cards {
                if card == a_c {
                    order = Ordering::Greater;
                    break 'outer;
                } else if card == b_c {
                    order = Ordering::Less;
                    break 'outer;
                }
            }
        }
    }

    order
}

pub fn part_1(input: impl AsRef<str>) -> u32 {
    input
        .as_ref()
        .lines()
        .filter_map(|line| line.split_whitespace().next_tuple::<(&str, &str)>())
        .filter_map(|(hand, bid)| bid.parse::<u32>().ok().map(|bid| (hand, bid)))
        .sorted_by(sort_hands::<PART_ONE>)
        .enumerate()
        .fold(0_u32, |acc, (rank, (_, bid))| acc + bid * (rank + 1) as u32)
}

pub fn part_2(input: impl AsRef<str>) -> u32 {
    input
        .as_ref()
        .lines()
        .filter_map(|line| line.split_whitespace().next_tuple::<(&str, &str)>())
        .filter_map(|(hand, bid)| bid.parse::<u32>().ok().map(|bid| (hand, bid)))
        .sorted_by(sort_hands::<PART_TWO>)
        .enumerate()
        .fold(0_u32, |acc, (rank, (_, bid))| acc + bid * (rank + 1) as u32)
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
