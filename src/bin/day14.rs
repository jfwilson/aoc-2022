use itertools::Itertools;
use std::{
    collections::BTreeMap,
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
    solve(input, false)
}

fn problem2_solution(input: &Vec<String>) -> usize {
    solve(input, true)
}

fn solve(input: &Vec<String>, has_floor: bool) -> usize {
    let mut occupied: BTreeMap<i32, BTreeMap<i32, ()>> = BTreeMap::new();
    for line in input {
        for (from, to) in line
            .split(" -> ")
            .flat_map(|coord| coord.split(','))
            .filter_map(|s| i32::from_str(s).ok())
            .tuples::<(i32, i32)>()
            .tuple_windows()
        {
            fill_block(from, to, &mut occupied);
        }
    }
    if has_floor {
        let y = occupied
            .values()
            .filter_map(|col| col.keys().last())
            .max()
            .unwrap()
            + 2;
        fill_block(
            (occupied.keys().next().unwrap() - 1000, y),
            (occupied.keys().last().unwrap() + 1000, y),
            &mut occupied,
        )
    };
    for count in 0usize.. {
        if !is_free(&occupied, 500, 0) {
            return count;
        }
        if let Some(pos) = drop_from(&occupied, 500, 0) {
            occupied.get_mut(&pos.0).unwrap().insert(pos.1, ());
        } else {
            return count;
        }
    }
    panic!("Boom");
}

fn fill_block(from: (i32, i32), to: (i32, i32), occupied: &mut BTreeMap<i32, BTreeMap<i32, ()>>) {
    println!("Adding {:?} -> {:?}", from, to);
    for x in from.0.min(to.0)..=from.0.max(to.0) {
        if !occupied.contains_key(&x) {
            occupied.insert(x, BTreeMap::new());
        }
        let col: &mut BTreeMap<i32, ()> = occupied.get_mut(&x).unwrap();
        for y in from.1.min(to.1)..=from.1.max(to.1) {
            col.insert(y, ());
        }
    }
}

fn is_free(occupied: &BTreeMap<i32, BTreeMap<i32, ()>>, x: i32, y: i32) -> bool {
    occupied.get(&x).and_then(|col| col.get(&y)).is_none()
}

fn drop_from(
    occupied: &BTreeMap<i32, BTreeMap<i32, ()>>,
    x: i32,
    from_y: i32,
) -> Option<(i32, i32)> {
    let col = occupied.get(&x)?;
    let occupied_at = col.keys().copied().find(|y| y > &from_y)?;
    if is_free(occupied, x - 1, occupied_at) {
        drop_from(occupied, x - 1, occupied_at)
    } else if is_free(occupied, x + 1, occupied_at) {
        drop_from(occupied, x + 1, occupied_at)
    } else {
        Some((x, occupied_at - 1))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, 24);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());

        assert_eq!(answer, 93);
    }
}
