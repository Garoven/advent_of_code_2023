const SYMBOLS: [char; 14] = [
    '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '+', '=', '/',
];

pub fn part_1(input: impl AsRef<str>) -> u32 {
    let input = input
        .as_ref()
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut total = 0;
    let mut tmp = String::with_capacity(8);
    let mut valid = false;
    for (l_index, line) in input.iter().enumerate() {
        for (c_index, ch) in line.iter().enumerate() {
            if ch.is_ascii_digit() {
                tmp.push(*ch);

                if !valid {
                    let l_len = line.len() - 1;
                    let c_left = c_index.saturating_sub(1);
                    let c_right = l_len.min(c_index + 1);

                    let top = &input[l_index.saturating_sub(1)][c_left..=c_right];
                    let bot = &input[(input.len() - 1).min(l_index + 1)][c_left..=c_right];

                    valid = top.iter().any(|c| SYMBOLS.contains(c))
                        || SYMBOLS.contains(&line[c_left])
                        || SYMBOLS.contains(&line[c_right])
                        || bot.iter().any(|c| SYMBOLS.contains(c));
                }
            } else if valid {
                total += tmp.parse::<u32>().unwrap();
                valid = false;
                tmp.clear();
            } else if !tmp.is_empty() {
                tmp.clear();
            }
        }
    }

    total
}

pub fn part_2(input: impl AsRef<str>) -> u32 {
    let input = input
        .as_ref()
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut ratios = std::collections::HashMap::<usize, (u32, u32)>::new();
    let mut index = 0;
    let mut tmp = String::with_capacity(8);
    let mut valid = false;
    for (l_index, line) in input.iter().enumerate() {
        for (c_index, ch) in line.iter().enumerate() {
            if ch.is_ascii_digit() {
                tmp.push(*ch);

                if !valid {
                    let l_len = line.len() - 1;
                    let c_left = c_index.saturating_sub(1);
                    let c_right = l_len.min(c_index + 1);

                    let top = &input[l_index.saturating_sub(1)][c_left..=c_right];
                    let bot = &input[(input.len() - 1).min(l_index + 1)][c_left..=c_right];

                    if let Some((i, _)) = top.iter().enumerate().find(|(_, c)| **c == '*') {
                        index = l_index.saturating_sub(1) * l_len + c_left + i;
                        valid = true;
                    } else if line[c_left] == '*' {
                        index = l_index * l_len + c_left;
                        valid = true;
                    } else if line[c_right] == '*' {
                        index = l_index * l_len + c_right;
                        valid = true;
                    } else if let Some((i, _)) = bot.iter().enumerate().find(|(_, c)| **c == '*') {
                        index = (input.len() - 1).min(l_index + 1) * l_len + c_left + i;
                        valid = true;
                    }
                }
            } else if valid {
                let v = tmp.parse::<u32>().unwrap();
                ratios
                    .entry(index)
                    .and_modify(|e| e.1 = v)
                    .or_insert((v, 0));

                valid = false;
                index = 0;
                tmp.clear();
            } else if !tmp.is_empty() {
                tmp.clear();
            }
        }
    }

    ratios.iter().map(|(_, (a, b))| a * b).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("./test.txt");

    #[test]
    fn part_1_ok() {
        let result = part_1(INPUT);
        let expected = 4361;

        assert_eq!(expected, result);
    }

    #[test]
    fn part_2_ok() {
        let result = part_2(INPUT);
        let expected = 467835;

        assert_eq!(expected, result);
    }
}
