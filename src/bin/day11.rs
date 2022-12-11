use itertools::Itertools;
use num_integer::Integer;
use std::{
    collections::VecDeque,
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
    solve(input, 20, true)
}

fn problem2_solution(input: &Vec<String>) -> usize {
    solve(input, 10000, false)
}

type WorryLevel = usize;

fn solve(input: &Vec<String>, rounds: usize, divide_by_three: bool) -> usize {
    let mut monkeys = parse(input);
    let modulo = monkeys.iter().fold(1, |acc, m| acc * m.divisor);
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let Monkey {
                operation,
                divisor,
                monkey_index_false,
                monkey_index_true,
                ..
            } = monkeys[i];
            while let Some(item) = monkeys[i].items.pop_front() {
                let mut new = operation.apply_to(item);
                if divide_by_three {
                    new /= 3;
                } else {
                    new %= modulo;
                }
                if new.is_multiple_of(&divisor) {
                    monkeys[monkey_index_true].items.push_back(new);
                } else {
                    monkeys[monkey_index_false].items.push_back(new);
                }
                monkeys[i].inspection_count += 1;
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));
    monkeys[0].inspection_count * monkeys[1].inspection_count
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Operation {
    Add(WorryLevel),
    Square,
    Multiply(WorryLevel),
}

impl Operation {
    fn apply_to(&self, old: WorryLevel) -> WorryLevel {
        match self {
            Operation::Add(n) => old + n,
            Operation::Square => old.pow(2),
            Operation::Multiply(n) => old * n,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<WorryLevel>,
    operation: Operation,
    divisor: WorryLevel,
    monkey_index_true: usize,
    monkey_index_false: usize,
    inspection_count: usize,
}

fn parse(input: &Vec<String>) -> Vec<Monkey> {
    input
        .chunks(7)
        .into_iter()
        .map(|chunk| {
            let items = chunk[1]["  Starting items: ".len()..]
                .split(", ")
                .map(|s| WorryLevel::from_str(s).unwrap())
                .collect();
            let operation = match chunk[2]
                .chars()
                .nth("  Operation: new = old ".len())
                .unwrap()
            {
                '+' => Operation::Add(
                    WorryLevel::from_str(&chunk[2]["  Operation: new = old + ".len()..]).unwrap(),
                ),
                _ => WorryLevel::from_str(&chunk[2]["  Operation: new = old * ".len()..])
                    .map_or(Operation::Square, |i| Operation::Multiply(i)),
            };
            let divisor = WorryLevel::from_str(&chunk[3]["  Test: divisible by ".len()..]).unwrap();
            let monkey_index_true =
                usize::from_str(&chunk[4]["    If true: throw to monkey ".len()..]).unwrap();
            let monkey_index_false =
                usize::from_str(&chunk[5]["    If false: throw to monkey ".len()..]).unwrap();
            Monkey {
                items,
                operation,
                divisor,
                monkey_index_true,
                monkey_index_false,
                inspection_count: 0,
            }
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, 10605);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());
        assert_eq!(answer, 2713310158);
    }
}
