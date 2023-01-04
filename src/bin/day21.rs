use std::{
    collections::HashMap,
    convert::identity,
    fs::File,
    hash::Hash,
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

fn problem1_solution(input: &Vec<String>) -> isize {
    let monkeys = parse(input);
    eval(&monkeys, &"root").unwrap()
}

fn problem2_solution(input: &Vec<String>) -> isize {
    let mut expr = parse(input);
    expr.remove(&"humn");
    if let Some(Op::Add(l, r)) = expr.get(&"root") {
        branch(&expr, l, r, identity, identity)
    } else {
        panic!("Unexpected root");
    }
}

#[derive(Debug)]
enum Op<K> {
    Yell(isize),
    Add(K, K),
    Sub(K, K),
    Mul(K, K),
    Div(K, K),
}

fn parse<'a>(input: &'a Vec<String>) -> HashMap<&'a str, Op<&'a str>> {
    input
        .iter()
        .map(|line| {
            let op: Op<&'a str> = line[6..]
                .parse::<isize>()
                .map(Op::Yell)
                .unwrap_or_else(|_| {
                    let lhs = &line[6..10];
                    let rhs = &line[13..17];
                    match line.as_bytes()[11] {
                        b'+' => Op::Add(lhs, rhs),
                        b'-' => Op::Sub(lhs, rhs),
                        b'*' => Op::Mul(lhs, rhs),
                        _ => Op::Div(lhs, rhs),
                    }
                });
            (&line[0..4], op)
        })
        .collect()
}

fn eval<K>(expr: &HashMap<K, Op<K>>, root: &K) -> Option<isize>
where
    K: Eq + Hash,
{
    Some(match expr.get(root)? {
        Op::Yell(x) => *x,
        Op::Add(l, r) => eval(expr, l)? + eval(expr, r)?,
        Op::Sub(l, r) => eval(expr, l)? - eval(expr, r)?,
        Op::Mul(l, r) => eval(expr, l)? * eval(expr, r)?,
        Op::Div(l, r) => eval(expr, l)? / eval(expr, r)?,
    })
}

fn solve<K>(expr: &HashMap<K, Op<K>>, root_key: &K, root_value: isize) -> isize
where
    K: Eq + Hash,
{
    if let Some(op) = expr.get(root_key) {
        match op {
            Op::Yell(_) => panic!("Unexpectedly yelling"),
            Op::Add(l, r) => branch(expr, l, r, |lv| root_value - lv, |rv| root_value - rv),
            Op::Sub(l, r) => branch(expr, l, r, |lv| lv - root_value, |rv| root_value + rv),
            Op::Mul(l, r) => branch(expr, l, r, |lv| root_value / lv, |rv| root_value / rv),
            Op::Div(l, r) => branch(expr, l, r, |lv| lv / root_value, |rv| root_value * rv),
        }
    } else {
        root_value
    }
}

fn branch<K: Eq + Hash, FL: FnOnce(isize) -> isize, FR: FnOnce(isize) -> isize>(
    expr: &HashMap<K, Op<K>>,
    l: &K,
    r: &K,
    fl: FL,
    fr: FR,
) -> isize {
    match (eval(expr, l), eval(expr, r)) {
        (Some(lv), None) => solve(expr, r, fl(lv)),
        (None, Some(rv)) => solve(expr, l, fr(rv)),
        _ => panic!("incorrect number of arms"),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, 152);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());
        assert_eq!(answer, 301);
    }
}
