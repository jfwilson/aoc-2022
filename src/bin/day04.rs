use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
    str::FromStr,
};

const INPUT_FILE: &str = concat!("./data/", env!("CARGO_BIN_NAME"), ".txt");

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
        .into_iter()
        .filter(|line| {
            let (elf1, elf2) = parse_line(line);
            is_second_contained_by(elf1, elf2) || is_second_contained_by(elf2, elf1)
        })
        .count()
}

fn problem2_solution(input: &Vec<String>) -> usize {
    input
        .into_iter()
        .filter(|line| {
            let (elf1, elf2) = parse_line(line);
            elf1.1 >= elf2.0 && elf1.0 <= elf2.1
        })
        .count()
}

fn parse_line(line: &str) -> ((u32, u32), (u32, u32)) {
    line.split(',')
        .map(|assignment| {
            assignment
                .split('-')
                .map(|section| u32::from_str(section).unwrap())
                .collect_tuple::<(u32, u32)>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap()
}

fn is_second_contained_by(outer: (u32, u32), inner: (u32, u32)) -> bool {
    outer.0 <= inner.0 && outer.1 >= inner.1
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, 2);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());

        assert_eq!(answer, 4);
    }
}
