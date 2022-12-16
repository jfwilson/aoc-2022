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

fn calc_score(valves: &Vec<Valve>, moves: &[Move]) -> usize {
    moves
        .iter()
        .fold((0, 0, &valves[0]), |(mut acc, mut rate, mut valve), m| {
            acc += rate;
            match m {
                Move::Open => rate += valve.rate,
                Move::MoveTo(id) => valve = &valves[*id],
            }
            (acc, rate, valve)
        })
        .0
}

fn problem1_solution(input: &Vec<String>) -> usize {
    let valves = parse_valves(input);
    println!("{:?}", valves);

    println!("From {:?}, I can go to:", &valves[0]);
    for &tunnel in valves[0].tunnels.iter() {
        println!(
            "  {} = {:?}: {}",
            tunnel,
            &valves[tunnel],
            calc_available_rate2(&valves, tunnel, valves[0].mask, 0)
        );
    }
    let mut best_sequence = [Move::Open; 30];
    let mut best_sequence_score: usize = 0;
    let mut moves: Vec<Move> = Vec::with_capacity(30);

    search_moves(
        &valves,
        &mut moves,
        0,
        0,
        &mut best_sequence,
        &mut best_sequence_score,
        0,
    );

    best_sequence_score
}

fn search_moves(
    valves: &Vec<Valve>,
    moves: &mut Vec<Move>,
    open_valves: usize,
    id: usize,
    best_sequence: &mut [Move; 30],
    best_sequence_score: &mut usize,
    mut dont_go_back_to: usize,
) {
    let valve = &valves[id];
    if (valve.rate > 0) && (open_valves & valve.mask) == 0 {
        dont_go_back_to = valve.mask;
        moves.push(Move::Open);
        if moves.len() == 30 {
            let score = calc_score(valves, moves);
            if score > *best_sequence_score {
                *best_sequence_score = score;
                best_sequence.clone_from_slice(&moves[0..]);

                println!("New best: {:?} = {}", best_sequence, best_sequence_score);
            }
        } else {
            search_moves(
                valves,
                moves,
                open_valves | valve.mask,
                id,
                best_sequence,
                best_sequence_score,
                dont_go_back_to,
            );
        }
        moves.pop();
    } else {
        dont_go_back_to |= valve.mask;
    }
    for next in valve
        .tunnels
        .iter()
        .filter(|&nid| (dont_go_back_to & (1usize << nid)) == 0)
    {
        if calc_available_rate2(valves, *next, dont_go_back_to, open_valves) > 0 {
            moves.push(Move::MoveTo(*next));
            if moves.len() == 30 {
                let score = calc_score(valves, moves);
                if score > *best_sequence_score {
                    *best_sequence_score = score;
                    best_sequence.clone_from_slice(&moves[0..]);

                    println!("New best: {:?} = {}", best_sequence, best_sequence_score);
                }
            } else {
                search_moves(
                    valves,
                    moves,
                    open_valves,
                    *next,
                    best_sequence,
                    best_sequence_score,
                    dont_go_back_to,
                );
            }
            moves.pop();
        }
    }
    let mut stay_still: [Move; 30] = [Move::MoveTo(id); 30];
    stay_still[0..moves.len()].copy_from_slice(&moves[0..]);
    let score = calc_score(valves, &stay_still);
    if score > *best_sequence_score {
        *best_sequence_score = score;
        *best_sequence = stay_still;

        println!("New best: {:?} = {}", best_sequence, best_sequence_score);
    }
}

fn calc_available_rate(
    valves: &Vec<Valve>,
    from_id: usize,
    visited_bitmap: &mut usize,
    open_bitmap: usize,
    acc: &mut usize,
) {
    let valve = &valves[from_id];
    if (*visited_bitmap & valve.mask) == 0 {
        *visited_bitmap |= valve.mask;
        if (open_bitmap & valve.mask) == 0 {
            *acc += valve.rate;
        }
        for next_id in valve.tunnels.iter() {
            calc_available_rate(valves, *next_id, visited_bitmap, open_bitmap, acc);
        }
    }
}

fn calc_available_rate2(
    valves: &Vec<Valve>,
    from_id: usize,
    mut visited_bitmap: usize,
    open_bitmap: usize,
) -> usize {
    let mut acc = 0;
    calc_available_rate(valves, from_id, &mut visited_bitmap, open_bitmap, &mut acc);
    acc
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
    fn test_score() {
        let valves = parse_valves(&load_test_data());
        let answer = calc_score(
            &valves,
            &vec![Move::MoveTo(1), Move::Open, Move::MoveTo(0), Move::Open],
        );
        assert_eq!(answer, 26);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());

        assert_eq!(answer, 3);
    }
}
