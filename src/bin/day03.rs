#![feature(iter_next_chunk)]
use std::{io::{BufReader, Result, BufRead}, fs::File, path::Path};

const INPUT_FILE: &str = concat!("./data/", env!("CARGO_BIN_NAME"), ".txt");

fn main() -> Result<()> {
    let input_file = File::open(Path::new(INPUT_FILE))?;
    let lines: Vec<String> = BufReader::new(input_file).lines().collect::<Result<Vec<String>>>()?;

    println!("problem1 = {}", problem1_solution(&lines));
    println!("problem2 = {}", problem2_solution(&lines));
    Ok(())
}

fn problem1_solution(input: &Vec<String>) -> u32 {
    input.into_iter().map(|line| {
        let chars = line.as_bytes();
        let half_len = chars.len() >> 1;
        
        let rucksack1 = to_priority_bitset(&chars[0..half_len]);
        let rucksack2 = to_priority_bitset(&chars[half_len..]);
        score_bitset(rucksack1 & rucksack2)
    }).sum()
}

fn problem2_solution(input: &Vec<String>) -> u32 {
    let mut iter = input.into_iter().map(|line| line.as_bytes());
    let mut score: u32 = 0;
    while let Ok(chunk) = iter.next_chunk::<3>() {
        let common_items = chunk.into_iter().fold(u64::MAX, |acc, rucksack| {
            acc & to_priority_bitset(rucksack)
        });
        score += score_bitset(common_items);
    }
    score
}

fn score_bitset(duplicates: u64) -> u32 {
    duplicates.trailing_zeros()
}

fn to_priority(item: u8) -> u8 {
    match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
        _ => panic!("Unexpected char in rucksack")
    }
}

fn to_priority_bitset(items: &[u8]) -> u64 {
    // println!("item = {}, {:?}", String::from_utf8_lossy(items), items);
    items.into_iter().map(|item| to_priority(*item)).fold(0, |acc, p| {
        acc | (1 << p)
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    
    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        assert_eq!(problem1_solution(&vec!("vJrwpWtwJgWrhcsFMMfFFhFp".to_owned())), 16);
        assert_eq!(to_priority(b'B'), 28);
        assert_eq!(problem1_solution(&vec!("hngprFFhFDFhrDpzzQDhtnBJJRJZbZvTcvbfRCJfBRcBJl".to_owned())), 28);

        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, 157);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());

        assert_eq!(answer, 70);
    }
}
