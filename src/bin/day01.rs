use std::{io::{BufReader, Result, BufRead, ErrorKind}, fs::File, path::Path};
use std::str::FromStr;

fn main() -> Result<()> {
    let path = Path::new("./data/day01.txt");
    let lines = BufReader::new(File::open(path)?).lines();

    println!("max_calories = {}", max_calories(lines)?);
    Result::Ok(())
}

fn max_calories<I: Iterator<Item = Result<String>>>(input: I) -> Result<usize> {
    let mut total_max_reindeer: usize = 0;
    let mut total_this_reindeer: usize = 0;
    for s in input {
        let line = s?;
        if line.is_empty() {
            println!("total_this_reindeer = {}", total_this_reindeer);
            total_max_reindeer = total_max_reindeer.max(total_this_reindeer);
            total_this_reindeer = 0;
        } else {
            let calories: usize = usize::from_str(&line).map_err(|_| ErrorKind::InvalidData)?;
            total_this_reindeer += calories;
        }
    }
    Result::Ok(total_max_reindeer.max(total_this_reindeer))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn problem1() {
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

        let max = max_calories(INPUT.lines().map(|s| Result::Ok(String::from(s)))).unwrap();

        assert_eq!(max, 24000);
    }
}
