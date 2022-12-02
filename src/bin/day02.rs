use std::{io::{BufReader, Result, BufRead}, fs::File, path::Path};

fn main() -> Result<()> {
    let path = Path::new("./data/day02.txt");
    let lines: Vec<String> = BufReader::new(File::open(path)?).lines().collect::<Result<Vec<String>>>()?;

    let score = total_score(&lines);

    println!("max_calories  = {}", score);
    Result::Ok(())
}

fn total_score(strategy: &Vec<String>) -> u64 {
    strategy.into_iter().map(|line| match line.as_str() {
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
        _ => panic!("Unexpected input")
    }).sum()
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
        let answer = total_score(&load_test_data());

        assert_eq!(answer, 15);
    }

    #[test]
    fn problem2() {
        let answer = total_score(&load_test_data());

        assert_eq!(answer, 45000);
    }
}
