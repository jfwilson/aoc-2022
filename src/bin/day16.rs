use itertools::Itertools;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Result},
    iter::once,
    mem::swap,
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
        Valve { rate, tunnels }
    }
}

fn valve_name_to_id(valve_names: &Vec<&str>, name: &str) -> usize {
    valve_names.iter().position(|&s| s == name).unwrap()
}

fn problem1_solution(input: &Vec<String>) -> usize {
    solve(input, 29, |pos_bitset| {
        [pos_bitset.trailing_zeros() as usize]
    })
}

fn problem2_solution(input: &Vec<String>) -> usize {
    solve(input, 25, |pos_bitset| {
        [
            pos_bitset.trailing_zeros() as usize,
            (63 - pos_bitset.leading_zeros()) as usize,
        ]
    })
}

fn solve<const NUM_ACTORS: usize, F>(
    input: &Vec<String>,
    t_minus_one: usize,
    expand_indices: F,
) -> usize
where
    F: Fn(usize) -> [usize; NUM_ACTORS],
{
    let (start_at, valves) = parse_valves(input);
    println!("{:?}", valves);

    let mut best_valve_rates = valves.iter().map(|v| v.rate).collect_vec();
    best_valve_rates.sort();
    best_valve_rates.reverse();

    let mut optimal_moves_to_current: HashMap<(usize, usize), usize> = HashMap::new();
    let mut optimal_moves_to_next: HashMap<(usize, usize), usize> = HashMap::new();
    let original_unopened_valves = valves
        .iter()
        .positions(|v| v.rate > 0)
        .fold(0, |acc, idx| acc | (1usize << idx));
    optimal_moves_to_current.insert((1usize << start_at, original_unopened_valves), 0usize);

    for t in 0..t_minus_one {
        let t_remaining = t_minus_one - t;
        optimal_moves_to_next.clear();

        println!(
            "Contemplating options at t = {} (time remaining after move = {}, starting count {})",
            t,
            t_remaining,
            optimal_moves_to_current.len()
        );
        remove_suboptimal::<NUM_ACTORS>(
            &mut optimal_moves_to_current,
            t_remaining,
            &best_valve_rates,
        );

        for (&(pos_bitset, unopened), &score) in optimal_moves_to_current.iter() {
            let actor_positions = expand_indices(pos_bitset);
            let mut states: Box<dyn Iterator<Item = (usize, usize, usize)>> =
                Box::new(once((0, unopened, score)));
            for from_idx in actor_positions {
                let valve = &valves[from_idx];
                let open_bonus = valve.rate * t_remaining;
                states = expand_search(from_idx, open_bonus, &valve.tunnels, states)
            }
            for (pos_bitset, unopened, score) in states {
                optimal_moves_to_next
                    .entry((pos_bitset, unopened))
                    .and_modify(|v| *v = (*v).max(score))
                    .or_insert(score);
            }
        }
        swap(&mut optimal_moves_to_current, &mut optimal_moves_to_next);
    }

    let best_sequence_score = *optimal_moves_to_current.values().max().unwrap();
    println!("{}: {}", t_minus_one, best_sequence_score);
    best_sequence_score
}

fn remove_suboptimal<const NUM_ACTORS: usize>(
    optimal_moves_to_current: &mut HashMap<(usize, usize), usize>,
    t_remaining: usize,
    best_valve_rates: &Vec<usize>,
) {
    let current_best = *optimal_moves_to_current.values().max().unwrap();
    let max_addition: usize = (1..=t_remaining)
        .rev()
        .step_by(2)
        .flat_map(|tt| [tt; NUM_ACTORS])
        .zip(best_valve_rates)
        .map(|(tt, r)| tt * r)
        .sum();
    println!(
        "  Current best is {}, upper bound is {} (current best + {})",
        current_best,
        current_best + max_addition,
        max_addition
    );
    optimal_moves_to_current.retain(|_, v| *v + max_addition >= current_best);
    println!(
        "  Drained suboptimal elements, count is now {}",
        optimal_moves_to_current.len()
    );
}

fn expand_search<'a, I>(
    from_idx: usize,
    open_bonus: usize,
    tunnel_indices: &'a Vec<usize>,
    states_so_far: I,
) -> Box<dyn Iterator<Item = (usize, usize, usize)> + 'a>
where
    I: Iterator<Item = (usize, usize, usize)> + 'a,
{
    Box::new(
        states_so_far.flat_map(move |(prev_bitset, prev_unopened, prev_score)| {
            once(from_idx)
                .chain(tunnel_indices.iter().copied())
                .map(move |to_idx| {
                    let to_bitset = 1usize << to_idx;
                    let new_unopened = if to_idx == from_idx {
                        prev_unopened & !to_bitset
                    } else {
                        prev_unopened
                    };
                    let new_score = if new_unopened != prev_unopened {
                        prev_score + open_bonus
                    } else {
                        prev_score
                    };
                    (prev_bitset | to_bitset, new_unopened, new_score)
                })
        }),
    )
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
