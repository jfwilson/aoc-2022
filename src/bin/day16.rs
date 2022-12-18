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
    let mut lines: Vec<String> = BufReader::new(input_file)
        .lines()
        .collect::<Result<Vec<String>>>()?;
    lines.sort();

    println!("problem1 = {}", problem1_solution(&lines));
    println!("problem2 = {}", problem2_solution(&lines));
    Ok(())
}

#[derive(Debug)]
struct Valve {
    mask: usize,
    rate: usize,
    tunnels: Vec<usize>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Move {
    Open,
    MoveTo(usize),
}

impl Valve {
    fn parse(s: &str, valve_names: &Vec<&str>) -> Self {
        let mask = 1usize << valve_name_to_id(valve_names, &s[6..8]);
        let (rate_str, tunnels_str) = s[23..].split(';').collect_tuple().unwrap();
        let rate = usize::from_str(rate_str).unwrap();
        let tunnels = tunnels_str
            .split(&[' ', ','])
            .skip(5)
            .step_by(2)
            .map(|name| valve_name_to_id(valve_names, name))
            .collect_vec();
        Valve {
            mask,
            rate,
            tunnels,
        }
    }
}

fn valve_name_to_id(valve_names: &Vec<&str>, name: &str) -> usize {
    valve_names.iter().position(|&s| s == name).unwrap()
}

fn problem1_solution(input: &Vec<String>) -> usize {
    let valves = parse_valves(input);
    println!("{:?}", valves);

    println!("From {:?}, I can go to:", &valves[0]);
    for &tunnel in valves[0].tunnels.iter() {
        println!("  {} = {:?}", tunnel, &valves[tunnel]);
    }

    let mut p1 = Problem1Solver {
        valves: &valves,
        best_sequence: [Move::Open; 30],
        best_sequence_score: 0,
        moves: Vec::with_capacity(30),
    };

    p1.search_moves(0, 0, 0, 0);

    p1.best_sequence_score
}

struct Problem1Solver<'a, const TIME: usize> {
    valves: &'a Vec<Valve>,
    moves: Vec<Move>,
    best_sequence: [Move; TIME],
    best_sequence_score: usize,
}

impl<'a, const TIME: usize> Problem1Solver<'a, TIME> {
    fn search_moves(
        &mut self,
        id: usize,
        open_valves: usize,
        dont_go_back_to: usize,
        score_lower_bound: usize,
    ) {
        if self.moves.len() == TIME {
            if score_lower_bound > self.best_sequence_score {
                self.best_sequence_score = score_lower_bound;
                self.best_sequence.clone_from_slice(&self.moves[0..]);

                println!(
                    "New best: {:?} = {}",
                    self.best_sequence, self.best_sequence_score
                );
            }
        } else {
            // stay still
            if score_lower_bound > self.best_sequence_score {
                self.best_sequence_score = score_lower_bound;
                self.best_sequence = [Move::MoveTo(id); TIME];
                self.best_sequence[0..self.moves.len()].clone_from_slice(&self.moves[0..]);

                println!(
                    "New best: {:?} = {}",
                    self.best_sequence, self.best_sequence_score
                );
            }
            // open valve (if rate > 0 and not already open)
            let Valve { mask, rate, .. } = &self.valves[id];
            if (*rate > 0) && (open_valves & mask) == 0 {
                self.moves.push(Move::Open);
                self.search_moves(
                    id,
                    open_valves | mask,
                    *mask,
                    score_lower_bound + rate * (TIME - self.moves.len()),
                );
                self.moves.pop();
            }
            // search tunnels (that we haven't just come from)
            let new_dont_go_back_to = dont_go_back_to | mask;
            for next in self.valves[id].tunnels.iter() {
                let nmask = 1usize << next;
                if (new_dont_go_back_to & nmask) == 0 {
                    self.moves.push(Move::MoveTo(*next));
                    self.search_moves(*next, open_valves, new_dont_go_back_to, score_lower_bound);
                    self.moves.pop();
                }
            }
        }
    }
}

fn parse_valves(input: &Vec<String>) -> Vec<Valve> {
    let valve_names = input.iter().map(|s| &s[6..8]).collect_vec();
    input
        .iter()
        .map(|s| Valve::parse(s, &valve_names))
        .collect_vec()
}

fn problem2_solution(input: &Vec<String>) -> usize {
    input
        .into_iter()
        .dedup_with_count()
        .map(|tuple| tuple.0)
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, 1651);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());

        assert_eq!(answer, 3);
    }
}
