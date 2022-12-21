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
    rate: usize,
    tunnels: Vec<usize>,
}

impl Valve {
    fn parse(s: &str, valve_names: &Vec<&str>) -> Self {
        let (rate_str, tunnels_str) = s[23..].split(';').collect_tuple().unwrap();
        let rate = usize::from_str(rate_str).unwrap();
        let tunnels = tunnels_str
            .split(&[' ', ','])
            .skip(5)
            .step_by(2)
            .map(|name| valve_name_to_id(valve_names, name))
            .collect_vec();
        Valve {
            rate,
            tunnels,
        }
    }
}

fn valve_name_to_id(valve_names: &Vec<&str>, name: &str) -> usize {
    valve_names.iter().position(|&s| s == name).unwrap()
}

fn problem1_solution(input: &Vec<String>) -> usize {
    solve::<1>(input, 29)
}

fn problem2_solution(input: &Vec<String>) -> usize {
    solve::<2>(input, 25)
}

fn solve<const NUM_ACTORS: usize>(input: &Vec<String>, t_minus_one: usize) -> usize {
    let (start_at, valves) = parse_valves(input);
    let valves_ref = &valves;
    println!("{:?}", valves);

    println!("From {:?}, I can go to:", &valves[0]);
    for &tunnel in valves[0].tunnels.iter() {
        println!("  {} = {:?}", tunnel, &valves[tunnel]);
    }

    let mut best_valve_rates = valves.iter().map(|v| v.rate).collect_vec();
    best_valve_rates.sort();
    best_valve_rates.reverse();

    let mut optimal_moves_to_current: HashMap<(usize, usize), usize> = HashMap::new();
    let mut optimal_moves_to_next: HashMap<(usize, usize), usize> = HashMap::new();
    let original_unopened_valves = valves.iter().positions(|v| v.rate > 0).fold(0, |acc, idx| acc | (1usize << idx));
    optimal_moves_to_current.insert((1usize << start_at, original_unopened_valves), 0usize);

    for t in 0..t_minus_one {
        let t_remaining = t_minus_one - t;
        optimal_moves_to_next.clear();

        println!("Contemplating options at t = {} (time remaining after move = {}, starting count {})", t, t_remaining, optimal_moves_to_current.len());
        remove_suboptimal::<2>(&mut optimal_moves_to_current, t_remaining, &best_valve_rates);

        for (&(pos, unopened), &score) in optimal_moves_to_current.iter() {
            let idx0 = pos.trailing_zeros() as usize;
            let explore = if NUM_ACTORS == 1 {
                explore_from(idx0, t_remaining, valves_ref, 0, unopened, score)
            } else {
                let idx1 = (usize::BITS - 1 - pos.leading_zeros()) as usize;
                Box::new(explore_from(idx0, t_remaining, valves_ref, 0, unopened, score).flat_map(move |(mask, unopened, score)| {
                    explore_from(idx1, t_remaining, valves_ref, mask, unopened, score)
                }))
            };
            for (mask, unopened, score) in explore {
                optimal_moves_to_next.entry((mask, unopened)).and_modify(|v| *v = (*v).max(score)).or_insert(score);
            }
        }
        swap(&mut optimal_moves_to_current, &mut optimal_moves_to_next);
    }

    let best_sequence_score = *optimal_moves_to_current.values().max().unwrap();
    println!("{}: {}", t_minus_one, best_sequence_score);
    best_sequence_score
}

fn remove_suboptimal<const SIZE: usize>(optimal_moves_to_current: &mut HashMap<(usize, usize), usize>, t_remaining: usize, best_valve_rates: &Vec<usize>) {
    let current_best = *optimal_moves_to_current.values().max().unwrap();
    let max_addition: usize = (1..=t_remaining).rev().step_by(2).flat_map(|tt| [tt; SIZE]).zip(best_valve_rates).map(|(tt, r)| tt * r).sum();
    println!("  Current best is {}, upper bound is {} (current best + {})", current_best, current_best + max_addition, max_addition);
    optimal_moves_to_current.retain(|_, v| *v + max_addition >= current_best);
    println!("  Drained suboptimal elements, count is now {}", optimal_moves_to_current.len());
}

fn explore_from<'a>(from_idx: usize, t_remaining: usize, valves: &'a Vec<Valve>, mask: usize, unopened: usize, score: usize) -> Box<dyn Iterator<Item = (usize, usize, usize)> + 'a> {
    let Valve { rate, tunnels, .. } = &valves[from_idx];
    let from_rate = *rate;
    Box::new(std::iter::once(from_idx).chain(tunnels.iter().copied()).map(move |to_idx| score_move(from_idx, from_rate, t_remaining, mask, unopened, score, to_idx)))
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

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, 1651);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());

        assert_eq!(answer, 1707);
    }
}
