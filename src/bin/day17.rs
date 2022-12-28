use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

const INPUT_FILE: &str = concat!("./data/", env!("CARGO_BIN_NAME"), ".txt");

fn main() -> Result<()> {
    let input_file = File::open(Path::new(INPUT_FILE))?;
    let lines: String = BufReader::new(input_file)
        .lines()
        .collect::<Result<String>>()?;

    println!("problem1 = {}", problem1_solution(&lines));
    println!("problem2 = {}", problem2_solution(&lines));
    Ok(())
}

const SHAPES: [[u8; 4]; 5] = [
    [0b0011110, 0, 0, 0],
    [0b0001000, 0b0011100, 0b0001000, 0],
    [0b0011100, 0b0000100, 0b0000100, 0],
    [0b0010000, 0b0010000, 0b0010000, 0b0010000],
    [0b0011000, 0b0011000, 0, 0],
];

fn problem1_solution(input: &str) -> usize {
    solve(input, 2022)
}

fn problem2_solution(input: &str) -> usize {
    solve(input, 1000000000000)
}

fn solve(input: &str, max_time: usize) -> usize {
    let shifts = input.chars().map(|c| c == '>').collect_vec();
    let mut shift_index = 0;
    let mut filled: Vec<u8> = Vec::new();
    for t in 0..max_time {
        if false {
            display(t, &filled);
        }
        let mut put_at = filled.iter().position(|&row| row == 0).unwrap_or(filled.len()) + 3;
        let mut shape = SHAPES[t % 5];
        'place_shape: loop {
            let shift_right = shifts[shift_index % shifts.len()];
            shift_index += 1;
            // try to shift
            if shift_right && shape.iter().all(|row| row.trailing_zeros() > 0) && no_collisions(shape, Shift::Right, &filled, put_at) {
                for row in shape.iter_mut() {
                    *row >>= 1;
                }
            } else if !shift_right && shape.iter().all(|row| row.leading_zeros() > 1) && no_collisions(shape, Shift::Left, &filled, put_at) {
                for row in shape.iter_mut() {
                    *row <<= 1;
                }
            }
            // try to drop
            if let Some(next_put_at) = put_at.checked_sub(1) {
                if no_collisions(shape, Shift::None, &filled, next_put_at) {
                    put_at = next_put_at;
                    continue 'place_shape;
                }
            }
            // cannot drop - place shape here
            filled.resize(filled.len().max(put_at + 4), 0);
            for i in 0..4 {
                filled[put_at + i] |= shape[i];
            }
            break;
        }
    }
    filled.len() - filled.iter().rev().take_while(|&row| row.eq(&0)).count()
}

fn display(t: usize, filled: &Vec<u8>) {
    println!("{}", t);
    for row in filled.iter().rev() {
        println!("{}", format!("{:#09b}", row).replace("0", ".").replace("1", "#"));
    }
}

fn no_collisions(shape: [u8; 4], shift: Shift, filled: &Vec<u8>, put_at: usize) -> bool {
    shape.iter().enumerate().all(|(idx, &row)| {
        filled.get(put_at + idx).unwrap_or(&0) & shift.shift(row) == 0
    })
}

enum Shift {
    None,
    Right,
    Left
}

impl Shift {
    fn shift(&self, lhs: u8) -> u8 {
        match self {
            Shift::None => lhs,
            Shift::Right => lhs >> 1,
            Shift::Left => lhs << 1,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn problem1() {
        let answer = problem1_solution(INPUT);
        assert_eq!(answer, 3068);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(INPUT);

        assert_eq!(answer, 1514285714288);
    }
}
