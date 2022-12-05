use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
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
    input.into_iter().dedup().count()
}

fn problem2_solution(input: &Vec<String>) -> usize {
    input
        .into_iter()
        .dedup_with_count()
        .map(|tuple| tuple.0)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "A
A
B
B
B";

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

        assert_eq!(answer, 3);
    }
}
