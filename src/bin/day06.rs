use core::panic;
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

    println!("problem1 = {}", problem1_solution(&lines[0]));
    println!("problem2 = {}", problem2_solution(&lines[0]));
    Ok(())
}

fn problem1_solution(input: &str) -> usize {
    find_marker(input, 4)
}

fn problem2_solution(input: &str) -> usize {
    find_marker(input, 14)
}

fn find_marker(input: &str, chunk_size: usize) -> usize {
    let mut chars: Vec<u8> = Vec::new();
    for i in chunk_size..(input.len()) {
        chars.clear();
        chars.extend_from_slice(&input.as_bytes()[(i - chunk_size)..i]);
        chars.sort();
        chars.dedup();
        if chars.len() == chunk_size {
            return i;
        }
    }
    panic!("Not found");
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT_A: &'static str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const INPUT_B: &'static str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const INPUT_C: &'static str = "nppdvjthqldpwncqszvftbrmjlhg";
    const INPUT_D: &'static str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const INPUT_E: &'static str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn problem1() {
        assert_eq!(problem1_solution(INPUT_A), 7);
        assert_eq!(problem1_solution(INPUT_B), 5);
        assert_eq!(problem1_solution(INPUT_C), 6);
        assert_eq!(problem1_solution(INPUT_D), 10);
        assert_eq!(problem1_solution(INPUT_E), 11);
    }

    #[test]
    fn problem2() {
        assert_eq!(problem2_solution(INPUT_A), 19);
        assert_eq!(problem2_solution(INPUT_B), 23);
        assert_eq!(problem2_solution(INPUT_C), 23);
        assert_eq!(problem2_solution(INPUT_D), 29);
        assert_eq!(problem2_solution(INPUT_E), 26);
    }
}
