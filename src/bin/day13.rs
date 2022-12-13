use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

const INPUT_FILE: &str = concat!("./data/", env!("CARGO_BIN_NAME"), ".txt");

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum Packet {
    Num(i32),
    Array(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Num(l), Packet::Num(r)) => l.cmp(r),
            (Packet::Array(ls), Packet::Array(rs)) => ls.cmp(rs),
            (Packet::Num(l), Packet::Array(rs)) => vec![Packet::Num(*l)].cmp(rs),
            (Packet::Array(ls), Packet::Num(r)) => ls.cmp(&vec![Packet::Num(*r)]),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> Result<()> {
    let input_file = File::open(Path::new(INPUT_FILE))?;
    let lines: Vec<String> = BufReader::new(input_file)
        .lines()
        .collect::<Result<Vec<String>>>()?;

    println!("problem1 = {}", problem1_solution(&lines));
    println!("problem2 = {}", problem2_solution(&lines));
    Ok(())
}

fn problem1_solution(input: &Vec<String>) -> usize {
    input
        .iter()
        .filter_map(|line| serde_json::from_str::<Packet>(line).ok())
        .tuples()
        .positions(|(l, r)| l <= r)
        .map(|p| p + 1)
        .sum()
}

fn problem2_solution(input: &Vec<String>) -> usize {
    let mut packets: Vec<Packet> = input
        .iter()
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect();
    let divider1 = Packet::Array(vec![Packet::Num(2)]);
    let divider2 = Packet::Array(vec![Packet::Num(6)]);
    packets.push(divider1.clone());
    packets.push(divider2.clone());
    packets.sort();
    packets
        .iter()
        .positions(|p| *p == divider1 || *p == divider2)
        .map(|p| p + 1)
        .product()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, 13);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());
        assert_eq!(answer, 140);
    }
}
