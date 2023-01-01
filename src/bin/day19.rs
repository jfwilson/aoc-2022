use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
    time::{Duration, SystemTime},
    vec,
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
    input
        .iter()
        .map(|line| {
            let (id, score) = score_blueprint(line, 24);
            id * score
        })
        .sum()
}

fn problem2_solution(input: &Vec<String>) -> usize {
    input
        .iter()
        .take(3)
        .map(|line| {
            let (_, score) = score_blueprint(line, 32);
            score
        })
        .product()
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
struct State {
    robots: [u8; 4],
    wallet: [u8; 4],
}

impl State {
    fn try_build(&self, robot_index: usize, robot_cost: [u8; 4]) -> Option<Self> {
        let State {
            mut robots,
            mut wallet,
        } = self;
        for i in 0..4 {
            if let Some(new_balance) = wallet[i].checked_sub(robot_cost[i]) {
                wallet[i] = new_balance;
            } else {
                return None;
            }
        }
        robots[robot_index] += 1;
        Some(State { robots, wallet })
    }

    fn increase_balance(&mut self, amounts: [u8; 4]) {
        for i in 0..4 {
            self.wallet[i] += amounts[i];
        }
    }

    fn score(&self, minutes_remaining: usize) -> usize {
        self.robots[3] as usize * minutes_remaining + self.wallet[3] as usize
    }

    fn beats(&self, other: &Self) -> bool {
        self.wallet[0] > other.wallet[0]
            || self.robots[0] > other.robots[0]
            || self.wallet[1] > other.wallet[1]
            || self.robots[1] > other.robots[1]
            || self.wallet[2] > other.wallet[2]
            || self.robots[2] > other.robots[2]
            || self.wallet[3] > other.wallet[3]
            || self.robots[3] > other.robots[3]
    }
}

fn score_blueprint(line: &str, num_minutes: usize) -> (usize, usize) {
    let (id, blueprint) = parse_blueprint(line);

    println!("Blueprint {}: {:?}", id, blueprint);
    let mut this_round = Vec::new();
    let mut next_round = vec![State {
        robots: [1, 0, 0, 0],
        wallet: [0; 4],
    }];
    let mut best_score = 0usize;
    for t in 0..num_minutes {
        let mut time = SystemTime::now();
        let minutes_remaining = num_minutes - t;
        best_score = next_round
            .iter()
            .map(|s| s.score(minutes_remaining))
            .max()
            .unwrap_or_default();
        let max_available = minutes_remaining * (minutes_remaining - 1) >> 1;
        // Only copy over entries that could be a high score and beat everything else in some way
        let mut i = 0;
        while i < next_round.len() {
            let elapsed = time.elapsed();
            if let Some(d) = elapsed.ok().filter(|&d| d > Duration::from_secs(1)) {
                println!("... {} of {}", i, next_round.len());
                time += d;
            }
            let state = next_round[i];
            i += 1;
            if state.score(minutes_remaining) + max_available >= best_score
                && next_round[i..].iter().all(|other| state.beats(other))
            {
                this_round.push(state);
            }
        }
        next_round.clear();
        println!(
            "Minute {}, search space size {}, best score {}, max_available {}",
            t,
            this_round.len(),
            best_score,
            max_available
        );
        for state in this_round.drain(..) {
            // Options: do nothing or build a robot
            let mut do_nothing = state;
            do_nothing.increase_balance(state.robots);
            next_round.push(do_nothing);
            for r in 0..4 {
                let robot_blueprint = blueprint[r];
                if let Some(mut new_state) = state.try_build(r, robot_blueprint) {
                    new_state.increase_balance(state.robots);
                    next_round.push(new_state);
                }
            }
        }
        next_round.sort_unstable();
        next_round.dedup();
    }
    best_score = next_round
        .iter()
        .map(|s| s.score(0))
        .max()
        .unwrap_or_default();
    println!("Blueprint {} has best score {}", id, best_score);
    (id, best_score)
}

fn parse_blueprint(line: &str) -> (usize, [[u8; 4]; 4]) {
    let t: (u8, u8, u8, u8, u8, u8, u8) = line
        .split(&[':', ' '])
        .filter_map(|s| s.parse::<u8>().ok())
        .inspect(|m| println!("{}", m))
        .collect_tuple()
        .unwrap();
    (
        t.0 as usize,
        [
            [t.1, 0, 0, 0],
            [t.2, 0, 0, 0],
            [t.3, t.4, 0, 0],
            [t.5, 0, t.6, 0],
        ],
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, 33);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());
        assert_eq!(answer, 56 * 62);
    }
}
