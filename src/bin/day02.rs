use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

fn main() -> Result<()> {
    let path = Path::new("./data/day02.txt");
    let lines: Vec<String> = BufReader::new(File::open(path)?)
        .lines()
        .collect::<Result<Vec<String>>>()?;

    println!("problem1 = {}", problem1_score(&lines));
    println!("problem2 = {}", problem2_score(&lines));
    Result::Ok(())
}

fn problem1_score(strategy: &Vec<String>) -> u64 {
    // X = rock, Y = paper, Z = scissors
    strategy
        .into_iter()
        .map(|line| match line.as_str() {
            // A = Rock vs
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,
            // B = Paper vs
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            // C = Scissors vs
            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            _ => panic!("Unexpected input"),
        })
        .sum()
}

fn problem2_score(strategy: &Vec<String>) -> u64 {
    // X = lose, Y = draw, Z = win
    strategy
        .into_iter()
        .map(|line| match line.as_str() {
            // A = Rock vs
            "A X" => 3 + 0,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            // B = Paper vs
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            // C = Scissors vs
            "C X" => 2 + 0,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
            _ => panic!("Unexpected input"),
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "A Y
B X
C Z";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_score(&load_test_data());

        assert_eq!(answer, 15);
    }

    #[test]
    fn problem2() {
        let answer = problem2_score(&load_test_data());

        assert_eq!(answer, 12);
    }
}
