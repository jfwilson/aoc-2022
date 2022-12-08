use itertools::Itertools;
use std::{
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
    let grid = &load_grid(input);
    let mut visible_count = 0;
    for (y, row) in grid.into_iter().enumerate() {
        for x in 0..row.len() {
            let tree_height = row[x];
            let is_visible_left = row[0..x].into_iter().all(|t| t < &tree_height);
            let is_visible_right = row[(x + 1)..].into_iter().all(|t| t < &tree_height);
            let is_visible_top = grid[0..y].into_iter().all(|r| r[x] < tree_height);
            let is_visible_bottom = grid[(y + 1)..].into_iter().all(|r| r[x] < tree_height);
            if is_visible_left || is_visible_right || is_visible_top || is_visible_bottom {
                visible_count += 1;
            }
        }
    }
    visible_count
}

fn problem2_solution(input: &Vec<String>) -> usize {
    let grid = &load_grid(input);
    let mut scenic_score_max = 0;
    for (y, row) in grid.into_iter().enumerate() {
        for x in 0..row.len() {
            let tree_height = row[x];
            let score_left = scenic_score(tree_height, row[0..x].into_iter().rev().copied());
            let score_right = scenic_score(tree_height, row[(x + 1)..].into_iter().copied());
            let score_top = scenic_score(tree_height, grid[0..y].into_iter().map(|r| r[x]).rev());
            let score_bottom = scenic_score(tree_height, grid[(y + 1)..].into_iter().map(|r| r[x]));
            let score_total = score_left * score_right * score_top * score_bottom;
            if score_total > scenic_score_max {
                scenic_score_max = score_total;
            }
        }
    }
    scenic_score_max
}

fn load_grid(input: &Vec<String>) -> Vec<Vec<u8>> {
    input
        .into_iter()
        .map(|row| {
            row.as_bytes()
                .into_iter()
                .map(|tree| tree - b'0')
                .collect_vec()
        })
        .collect_vec()
}

fn scenic_score<I: Iterator<Item = u8>>(tree_height: u8, view: I) -> usize {
    let v = &view.collect_vec();
    let mut count = v.into_iter().take_while(|t| **t < tree_height).count();
    if count < v.len() {
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "30373
25512
65332
33549
35390";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, 21);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());

        assert_eq!(answer, 8);
    }
}
