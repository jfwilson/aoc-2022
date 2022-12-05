use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    ops::IndexMut,
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

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct State {
    stacks: Vec<Vec<char>>,
}

impl State {
    fn run(&mut self, instruction: &Instruction, preserve_order: bool) {
        if preserve_order {
            let from = self.stacks.index_mut(instruction.from - 1);
            let items = from.drain((from.len() - instruction.count)..).collect_vec();
            self.stacks[instruction.to - 1].extend(items);
        } else {
            for _ in 0..instruction.count {
                let c = self.stacks[instruction.from - 1].pop().unwrap();
                self.stacks[instruction.to - 1].push(c);
            }
        }
    }

    fn to_string(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().unwrap_or(&' '))
            .collect()
    }
}

fn parse_input(input: &Vec<String>) -> (State, Vec<Instruction>) {
    let mut lines = input.into_iter();
    let stacks: Vec<Vec<char>> =
        lines
            .by_ref()
            .take_while(|line| line.contains("["))
            .fold(Vec::new(), |mut stacks, row| {
                for (index, char) in row.chars().skip(1).step_by(4).enumerate() {
                    while stacks.len() <= index {
                        stacks.push(Vec::new())
                    }
                    if char != ' ' {
                        stacks[index].insert(0, char);
                    }
                }
                stacks
            });
    let initial_state = State { stacks };
    assert!(lines.next().unwrap().is_empty());
    let instructions = lines
        .map(|line| {
            let (count, from, to) = line
                .split_ascii_whitespace()
                .filter_map(|line| usize::from_str(line).ok())
                .collect_tuple()
                .unwrap();
            Instruction { count, from, to }
        })
        .collect_vec();
    (initial_state, instructions)
}

fn run(input: &Vec<String>, preserve_order: bool) -> String {
    let (mut state, instructions) = parse_input(input);
    // println!("initial_state = {:?}", state);
    // println!("instructions = {:?}", instructions);
    for instruction in instructions {
        state.run(&instruction, preserve_order);
        // println!("state = {:?}", state);
    }
    // println!("final_state = {:?}", state);
    state.to_string()
}

fn problem1_solution(input: &Vec<String>) -> String {
    run(input, false)
}

fn problem2_solution(input: &Vec<String>) -> String {
    run(input, true)
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, "CMZ");
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());
        assert_eq!(answer, "MCD");
    }
}
