use itertools::Itertools;
use std::{
    convert::identity,
    fs::File,
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

fn problem1_solution(input: &Vec<String>) -> usize {
    solve(input, vec![locate_cell(input, 'S')])
}

fn problem2_solution(input: &Vec<String>) -> usize {
    let starting_points = input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, c)| *c == 'a')
                .map(move |(x, _)| (x, y))
        })
        .collect_vec();
    solve(input, starting_points)
}

fn solve(input: &Vec<String>, mut starting_points: Vec<(usize, usize)>) -> usize {
    let heights = &parse_cell_heights(input);
    let (end_x, end_y) = locate_cell(input, 'E');
    let mut visited_cells: Vec<Vec<Option<usize>>> = heights
        .iter()
        .map(|v| v.iter().map(|_| None).collect_vec())
        .collect_vec();
    (usize::MIN..)
        .find_map(|steps_taken| {
            let mut next_points: Vec<(usize, usize)> = starting_points
                .iter()
                .copied()
                .flat_map(|(x, y)| {
                    visited_cells[y][x] = Some(steps_taken);
                    new_reachable_cells(heights, &visited_cells, x, y)
                })
                .filter_map(identity)
                .collect();
            next_points.sort();
            next_points.dedup();
            starting_points.clone_from(&next_points);
            visited_cells[end_y][end_x]
        })
        .unwrap()
}

fn parse_cell_heights(input: &Vec<String>) -> Vec<Vec<u8>> {
    input
        .iter()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|b| match b {
                    b'S' => 0,
                    b'E' => b'z' - b'a',
                    _ => b - b'a',
                })
                .collect_vec()
        })
        .collect_vec()
}

fn locate_cell(input: &Vec<String>, cell: char) -> (usize, usize) {
    input
        .iter()
        .find_position(|line| line.contains(cell))
        .and_then(|(y, row)| row.chars().position(|c| c == cell).map(|x| (x, y)))
        .unwrap()
}

fn new_reachable_cells(
    heights: &Vec<Vec<u8>>,
    visited_cells: &Vec<Vec<Option<usize>>>,
    x: usize,
    y: usize,
) -> [Option<(usize, usize)>; 4] {
    let max_height = heights[y][x] + 1;
    [
        (((x + 1)..heights[0].len()).next(), Some(y)),
        (x.checked_sub(1), Some(y)),
        (Some(x), ((y + 1)..heights.len()).next()),
        (Some(x), y.checked_sub(1)),
    ]
    .map(|(n_x, n_y)| {
        n_x.zip(n_y)
            .filter(|&(x, y)| heights[y][x] <= max_height && visited_cells[y][x].is_none())
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, 31);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());
        assert_eq!(answer, 29);
    }
}
