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
    solve(input, vec![locate_cell(input, 'S')])
}

fn problem2_solution(input: &Vec<String>) -> usize {
    let starting_points = input
        .into_iter()
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
    let heights = &input
        .into_iter()
        .map(|line| {
            line.as_bytes()
                .into_iter()
                .map(|b| match b {
                    b'S' => 0,
                    b'E' => b'z' - b'a',
                    _ => b - b'a',
                })
                .collect_vec()
        })
        .collect_vec();
    let height = heights.len();
    let width = heights.into_iter().map(|hs| hs.len()).max().unwrap();
    let (end_x, end_y) = locate_cell(input, 'E');
    let mut distances: Vec<Vec<Option<usize>>> = heights
        .into_iter()
        .map(|v| v.into_iter().map(|_| None).collect_vec())
        .collect_vec();
    let mut distance: usize = 0;
    while distances[end_y][end_x].is_none() && distance < 2000 {
        let mut next_points: Vec<(usize, usize)> = (&starting_points)
            .into_iter()
            .copied()
            .flat_map(|(x, y)| {
                distances[y][x] = Some(distance);
                let h = heights[y][x];
                [
                    visit_cell(
                        heights,
                        &distances,
                        h,
                        x.checked_add(1).filter(|x| *x < width),
                        Some(y),
                    ),
                    visit_cell(heights, &distances, h, x.checked_sub(1), Some(y)),
                    visit_cell(
                        heights,
                        &distances,
                        h,
                        Some(x),
                        y.checked_add(1).filter(|y| *y < height),
                    ),
                    visit_cell(heights, &distances, h, Some(x), y.checked_sub(1)),
                ]
            })
            .filter_map(|cell| cell)
            .collect();
        next_points.sort();
        next_points.dedup();
        starting_points.clear();
        starting_points.extend(&next_points);
        distance += 1;
    }
    distances[end_y][end_x].unwrap()
}

fn locate_cell(input: &Vec<String>, cell: char) -> (usize, usize) {
    let (end_y, end_line) = input
        .into_iter()
        .find_position(|line| line.contains(cell))
        .unwrap();
    let end_x = end_line.chars().position(|c| c == cell).unwrap();
    (end_x, end_y)
}

fn visit_cell(
    heights: &Vec<Vec<u8>>,
    distances: &Vec<Vec<Option<usize>>>,
    height: u8,
    x: Option<usize>,
    y: Option<usize>,
) -> Option<(usize, usize)> {
    x.zip(y)
        .filter(|(x, y)| (heights[*y][*x] <= height + 1) && distances[*y][*x].is_none())
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
