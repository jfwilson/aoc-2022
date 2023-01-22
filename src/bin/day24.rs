use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    mem::swap,
    path::Path,
};

const INPUT_FILE: &str = concat!("./data/", env!("CARGO_BIN_NAME"), ".txt");

fn main() -> Result<()> {
    let input_file = File::open(Path::new(INPUT_FILE))?;
    let lines: Vec<String> = BufReader::new(input_file)
        .lines()
        .collect::<Result<Vec<String>>>()?;

    println!("problem1 = {}", problem1_solution(&lines).unwrap());
    println!("problem2 = {}", problem2_solution(&lines).unwrap());
    Ok(())
}

fn problem1_solution(input: &Vec<String>) -> Option<usize> {
    let max_x = input[0].len() - 2;
    let max_y = input.len() - 2;
    let start_x = input[0].find('.').unwrap();
    let start_y = 0;
    let end_x = input[1 + max_y].find('.').unwrap();
    let end_y = max_y;

    solve(input, max_x, max_y, 0, start_x, start_y, end_x, end_y)
}

fn problem2_solution(input: &Vec<String>) -> Option<usize> {
    let max_x = input[0].len() - 2;
    let max_y = input.len() - 2;
    let start_x = input[0].find('.').unwrap();
    let end_x = input[1 + max_y].find('.').unwrap();

    let t1 = solve(input, max_x, max_y, 0, start_x, 0, end_x, max_y)?;
    let t2 = solve(input, max_x, max_y, t1, end_x, max_y + 1, start_x, 1)?;
    solve(input, max_x, max_y, t2, start_x, 0, end_x, max_y)
}

fn solve(
    input: &Vec<String>,
    max_x: usize,
    max_y: usize,
    start_t: usize,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
) -> Option<usize> {
    let mut next_reachable: Vec<(usize, usize)> = vec![(start_x, start_y)];
    let mut this_reachable: Vec<(usize, usize)> = Vec::new();
    for next_t in (start_t + 1)..1000 {
        swap(&mut this_reachable, &mut next_reachable);
        for (from_x, from_y) in this_reachable.drain(..) {
            if from_y == end_y && from_x == end_x {
                return Some(next_t);
            }
            if from_y == start_y || is_cell_free(input, next_t, from_x, from_y) {
                next_reachable.push((from_x, from_y));
            }
            if from_x < max_x {
                let to_x = from_x + 1;
                if is_cell_free(input, next_t, to_x, from_y) {
                    next_reachable.push((to_x, from_y));
                }
            }
            if from_x > 1 {
                let to_x = from_x - 1;
                if is_cell_free(input, next_t, to_x, from_y) {
                    next_reachable.push((to_x, from_y));
                }
            }
            if from_y < max_y {
                let to_y = from_y + 1;
                if is_cell_free(input, next_t, from_x, to_y) {
                    next_reachable.push((from_x, to_y));
                }
            }
            if from_y > 1 {
                let to_y = from_y - 1;
                if is_cell_free(input, next_t, from_x, to_y) {
                    next_reachable.push((from_x, to_y));
                }
            }
        }
        next_reachable.sort_unstable();
        next_reachable.dedup();

        // println!("t = {}, {:?}", next_t, next_reachable);
        // print!("#");
        // for x in 1..=max_x {
        //     print!("{}", if x == start_x { '.' } else { '#' });
        // }
        // println!("#");
        // for y in 1..=max_y {
        //     print!("#");
        //     for x in 1..=max_x {
        //         print!("{}", if next_reachable.contains(&(x, y)) { '!' } else if is_cell_free(input, next_t, x, y) { '.' } else { 'x' });
        //     }
        //     println!("#");
        // }
        // print!("#");
        // for x in 1..=max_x {
        //     print!("{}", if x == end_x { '.' } else { '#' });
        // }
        // println!("#");
    }
    None
}

fn is_cell_free(input: &Vec<String>, t: usize, x: usize, y: usize) -> bool {
    let max_x = input[0].len() - 2;
    let max_y = input.len() - 2;
    let min_1 = 1;
    let row = input[y].as_bytes();

    (1..=max_y).contains(&y)
        && (row[min_1 + (x - min_1 + t).rem_euclid(max_x)] != b'<')
        && (row[max_x - (max_x - x + t).rem_euclid(max_x)] != b'>')
        && (input[min_1 + (y - min_1 + t).rem_euclid(max_y)].as_bytes()[x] != b'^')
        && (input[max_y - (max_y - y + t).rem_euclid(max_y)].as_bytes()[x] != b'v')
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, Some(18));
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());
        assert_eq!(answer, Some(54));
    }
}
