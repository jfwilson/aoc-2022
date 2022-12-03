use std::str::FromStr;
use std::{
    fs::File,
    io::{BufRead, BufReader, ErrorKind, Result},
    path::Path,
};

fn main() -> Result<()> {
    let path = Path::new("./data/day01.txt");
    let lines = BufReader::new(File::open(path)?).lines();

    let totals = count_calories(lines)?;

    println!("max_calories  = {}", max_calories(&totals));
    println!("top3_calories = {}", top3_calories(&totals));
    Result::Ok(())
}

fn count_calories<I: Iterator<Item = Result<String>>>(input: I) -> Result<Vec<usize>> {
    let mut reindeer_totals: Vec<usize> = Vec::new();
    let mut total_this_reindeer: usize = 0;
    for s in input {
        let line = s?;
        if line.is_empty() {
            reindeer_totals.push(total_this_reindeer);
            total_this_reindeer = 0;
        } else {
            let calories: usize = usize::from_str(&line).map_err(|_| ErrorKind::InvalidData)?;
            total_this_reindeer += calories;
        }
    }
    reindeer_totals.push(total_this_reindeer);
    reindeer_totals.sort();
    Result::Ok(reindeer_totals)
}

fn max_calories(totals: &Vec<usize>) -> usize {
    totals[totals.len() - 1]
}

fn top3_calories(totals: &Vec<usize>) -> usize {
    totals[totals.len() - 3..].into_iter().sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    fn parse_test_data() -> Vec<usize> {
        count_calories(INPUT.lines().map(|s| Result::Ok(String::from(s)))).unwrap()
    }

    #[test]
    fn problem1() {
        let answer = max_calories(&parse_test_data());

        assert_eq!(answer, 24000);
    }

    #[test]
    fn problem2() {
        let answer = top3_calories(&parse_test_data());

        assert_eq!(answer, 45000);
    }
}
