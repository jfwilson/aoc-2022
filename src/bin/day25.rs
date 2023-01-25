use num_integer::Integer;
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
    Ok(())
}

fn problem1_solution(input: &Vec<String>) -> String {
    let mut total: i64 = input
        .iter()
        .map(|line| line.chars().map(value_of_char).fold(0, |t, v| t * 5 + v))
        .sum();
    let mut output = String::new();
    while total != 0 {
        let offset = total.signum() << 1;
        let (quot, rem) = (total + offset).div_rem(&5);
        output.insert(0, ['=', '-', '0', '1', '2'][(2 + rem - offset) as usize]);
        total = quot;
    }
    output
}

fn value_of_char(c: char) -> i64 {
    match c {
        '=' => -2,
        '-' => -1,
        _ => c.to_digit(3).unwrap() as i64,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, "2=-1=0");
    }
}
