use itertools::Itertools;
use num::{Integer, Zero};
use rayon::prelude::{ParallelBridge, ParallelIterator};
use std::collections::HashMap;

fn map_nodes(line: &str) -> (&str, (&str, &str)) {
    let (node, nodes) = line
        .split('=')
        .map(|spl| spl.trim())
        .next_tuple()
        .expect("Invalid input");

    let nodes: (&str, &str) = nodes[1..nodes.len() - 1]
        .split(',')
        .map(|spl| spl.trim())
        .next_tuple()
        .expect("Invalid input");

    (node, nodes)
}

macro_rules! exec_instruction {
    ($side:expr, $nodes:expr) => {
        match $side {
            'L' => $nodes.0,
            'R' => $nodes.1,
            a => panic!("co: {a}"),
        }
    };
}

pub fn part_1(input: impl AsRef<str>) -> u64 {
    let mut it = input.as_ref().lines();
    let mut instuctions = it.next().map(|l| l.chars()).expect("Invalid input").cycle();
    let nodes: HashMap<&str, (&str, &str)> = it.skip(1).map(map_nodes).collect();

    let mut count = 0;
    let mut node = String::from("AAA");
    while node.as_str() != "ZZZ" {
        let instruction = instuctions.next().expect("Invalid input");
        let new_node = exec_instruction!(instruction, nodes[node.as_str()]);

        node.clear();
        node.push_str(new_node);

        count += 1;
    }

    count
}

pub fn part_2(input: impl AsRef<str>) -> u64 {
    let mut it = input.as_ref().lines();
    let instuctions = it.next().map(|l| l.chars()).expect("Invalid input").cycle();
    let nodes: HashMap<&str, (&str, &str)> = it.skip(1).map(map_nodes).collect();

    nodes
        .keys()
        .filter(|s| s.ends_with('A'))
        .par_bridge()
        .fold_with(0_u64, |mut acc, key| {
            let mut instuctions = instuctions.clone();
            let mut node = String::from(*key);
            while !node.ends_with('Z') {
                let instruction = instuctions.next().expect("Invalid input");
                let new_node = exec_instruction!(instruction, nodes[node.as_str()]);

                node.clear();
                node.push_str(new_node);

                acc += 1;
            }

            acc
        })
        .filter(|n| !n.is_zero())
        .reduce(|| 1, |acc, count| acc.lcm(&count))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("./test.txt");

    #[test]
    fn part_1_ok() {
        let result = part_1(INPUT);
        let expected = 6;

        assert_eq!(expected, result);
    }

    #[test]
    fn part_2_ok() {
        let result = part_2(INPUT);
        let expected = 0;

        assert_eq!(expected, result);
    }
}
