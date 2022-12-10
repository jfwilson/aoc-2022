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
    println!("problem2:");
    println!("{}", problem2_solution(&lines));
    Ok(())
}

fn problem1_solution(input: &Vec<String>) -> i32 {
    input
        .into_iter()
        .map(|line| {
            line.split(' ')
                .skip(1)
                .next()
                .map(|v| i32::from_str(v).unwrap())
        })
        .fold((0, 0, 1), |(sum, cycle, x), v| {
            let next = match v {
                Some(v) => (cycle + 2, x + v),
                None => (cycle + 1, x),
            };
            let old_20 = cycle / 20;
            let new_20 = next.0 / 20;
            if (new_20 > old_20) && (new_20 & 1 == 1) {
                (sum + (new_20 * 20 * x), next.0, next.1)
            } else {
                (sum, next.0, next.1)
            }
        })
        .0
}

fn problem2_solution(input: &Vec<String>) -> String {
    input
        .into_iter()
        .map(|line| {
            line.split(' ')
                .skip(1)
                .next()
                .map(|v| i32::from_str(v).unwrap())
        })
        .scan((0, 1), |(cycle, x), v| {
            let char0 = pixel_output(*cycle, *x);
            match v {
                Some(v) => {
                    let char1 = pixel_output(*cycle + 1, *x);
                    *x += v;
                    *cycle += 2;
                    Some([char0, char1])
                }
                None => {
                    *cycle += 1;
                    Some([char0, ' '])
                }
            }
        })
        .flatten()
        .filter(|c| *c != ' ')
        .chunks(40)
        .into_iter()
        .flat_map(|row| row.chain("\n".chars()))
        .collect()
}

fn pixel_output(cycle: i32, x: i32) -> char {
    if i32::abs((cycle % 40) - x) <= 1 {
        '#'
    } else {
        '.'
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, 13140);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());

        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
        assert_eq!(&answer, expected);
    }
}
