use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
    str::FromStr, collections::HashMap, mem::swap,
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
    let (start_at, valves) = parse_valves(input);
    println!("{:?}", valves);

    println!("From {:?}, I can go to:", &valves[0]);
    for &tunnel in valves[0].tunnels.iter() {
        println!("  {} = {:?}", tunnel, &valves[tunnel]);
    }

    let mut p1 = Problem1Solver {
        valves: &valves,
        moves: Vec::with_capacity(30),
    };

    let (best_sequence_score, best_sequence) = p1.search_moves::<30>(start_at, 0, 0, 0);

    println!("{}: {:?}", best_sequence_score, best_sequence);
    best_sequence_score
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Key {
    location: usize,
    unopened: usize,
}

impl Key {
    const fn new(indices: [usize; 2], unopened: usize) -> Key {
        Key { location: (1usize << indices[0]) | (1usize << indices[1]), unopened }
    }

    fn indices(&self) -> [usize; 2] {
        let idx0 = self.location.trailing_zeros() as usize;
        let idx1 = (usize::BITS - 1 - self.location.leading_zeros()) as usize;
        [idx0, idx1]
    }
}

fn problem2_solution(input: &Vec<String>) -> usize {
    let (start_at, valves) = parse_valves(input);
    println!("{:?}", valves);

    println!("From {:?}, I can go to:", &valves[0]);
    for &tunnel in valves[0].tunnels.iter() {
        println!("  {} = {:?}", tunnel, &valves[tunnel]);
    }

    let mut optimal_moves_to_current = HashMap::new();
    let original_unopened_valves = valves.iter().positions(|v| v.rate > 0).fold(0, |acc, idx| acc | (1usize << idx));
    optimal_moves_to_current.insert(Key::new([start_at, start_at], original_unopened_valves), 0usize);
    let mut optimal_moves_to_next: HashMap<Key, usize> = HashMap::new();

    let mut best_valve_rates = valves.iter().map(|v| v.rate).collect_vec();
    best_valve_rates.sort();
    best_valve_rates.reverse();

    let t_minus_one = 25;
    for t in 0..t_minus_one {
        let t_remaining = t_minus_one - t;
        optimal_moves_to_next.clear();

        println!("Contemplating options at t = {} (time remaining after move = {}, starting count {})", t, t_remaining, optimal_moves_to_current.len());
        remove_suboptimal::<2>(&mut optimal_moves_to_current, t_remaining, &best_valve_rates);

        for (&k, &base_score) in optimal_moves_to_current.iter() {
            let [idx0, idx1] = k.indices();
            for (mask0, unopened0, score0) in explore_from(idx0, t_remaining, &valves, 0, k.unopened, base_score) {
                for (mask1, unopened1, score1) in explore_from(idx1, t_remaining, &valves, mask0, unopened0, score0) {
                    let nk = Key { location: mask1, unopened: unopened1 };
                    optimal_moves_to_next.entry(nk).and_modify(|v| *v = (*v).max(score1)).or_insert(score1);
                }
            }
        }
        swap(&mut optimal_moves_to_current, &mut optimal_moves_to_next);
    }

    let best_sequence_score = *optimal_moves_to_current.values().max().unwrap();
    println!("{}: {}", t_minus_one, best_sequence_score);
    best_sequence_score
}

fn score_move(from_idx: usize, from_rate: usize, t_remaining: usize, mask: usize, unopened: usize, score: usize, to_idx: usize) -> (usize, usize, usize) {
    let mut result = (1usize << to_idx, unopened, score);
    if to_idx == from_idx {
        result.1 &= !result.0;
        if result.1 != unopened {
            result.2 += from_rate * t_remaining;
        }
    }
    result.0 |= mask;
    result
}

fn explore_from<'a>(from_idx: usize, t_remaining: usize, valves: &'a Vec<Valve>, mask: usize, unopened: usize, score: usize) -> Box<dyn Iterator<Item = (usize, usize, usize)> + 'a> {
    let Valve { rate, tunnels, .. } = &valves[from_idx];
    let from_rate = *rate;
    Box::new(std::iter::once(from_idx).chain(tunnels.iter().copied()).map(move |to_idx| score_move(from_idx, from_rate, t_remaining, mask, unopened, score, to_idx)))
}

fn remove_suboptimal<const SIZE: usize>(optimal_moves_to_current: &mut HashMap<Key, usize>, t_remaining: usize, best_valve_rates: &Vec<usize>) {
    let current_best = *optimal_moves_to_current.values().max().unwrap();
    let max_addition: usize = (1..=t_remaining).rev().step_by(2).flat_map(|tt| [tt; SIZE]).zip(best_valve_rates).map(|(tt, r)| tt * r).sum();
    println!("  Current best is {}, upper bound is {} (current best + {})", current_best, current_best + max_addition, max_addition);
    optimal_moves_to_current.retain(|_, v| *v + max_addition >= current_best);
    println!("  Drained suboptimal elements, count is now {}", optimal_moves_to_current.len());
}

struct Problem1Solver<'a> {
    valves: &'a Vec<Valve>,
    moves: Vec<Move>,
}

impl<'a> Problem1Solver<'a> {
    fn search_moves<const TIME: usize>(
        &mut self,
        id: usize,
        open_valves: usize,
        dont_go_back_to: usize,
        score_lower_bound: usize,
    ) -> (usize, [Move; TIME]) {
        // stay still
        let mut result = (score_lower_bound, [Move::MoveTo(id); TIME]);
        let t = self.moves.len();
        result.1[0..t].copy_from_slice(&self.moves[0..]);
        let t_remaining_after_move = TIME - t - 1;
        if t_remaining_after_move > 0 {
            // open valve (if rate > 0 and not already open)
            let valve = &self.valves[id];
            if (valve.rate > 0) && (open_valves & valve.mask) == 0 {
                self.moves.push(Move::Open);
                let open_valve_result = self.search_moves(
                    id,
                    open_valves | valve.mask,
                    valve.mask,
                    score_lower_bound + valve.rate * t_remaining_after_move,
                );
                self.moves.pop();
                if open_valve_result.0 > result.0 {
                    result = open_valve_result;
                }
            }
            // search tunnels (that we haven't just come from)
            let new_dont_go_back_to = dont_go_back_to | valve.mask;
            for next in valve.tunnels.iter() {
                if (new_dont_go_back_to & (1usize << next)) == 0 {
                    self.moves.push(Move::MoveTo(*next));
                    let move_result = self.search_moves(
                        *next,
                        open_valves,
                        new_dont_go_back_to,
                        score_lower_bound,
                    );
                    self.moves.pop();
                    if move_result.0 > result.0 {
                        result = move_result;
                    }
                }
            }
        }
        result
    }
}

fn parse_valves(input: &Vec<String>) -> (usize, Vec<Valve>) {
    let valve_names = input.iter().map(|s| &s[6..8]).collect_vec();
    (
        valve_names.iter().position(|&s| s == "AA").unwrap(),
        input
            .iter()
            .map(|s| Valve::parse(s, &valve_names))
            .collect_vec(),
    )
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

    // #[test]
    // fn problem1() {
    //     let answer = problem1_solution(&load_test_data());
    //     assert_eq!(answer, 1651);
    // }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());

        assert_eq!(answer, 1707);
    }
}
