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

const T: usize = 10;

type Coord = (i32, i32);

const OFFSETS: [[Coord; 3]; 4] = [
    [(-1, -1), (0, -1), (1, -1)],
    [(-1, 1), (0, 1), (1, 1)],
    [(-1, -1), (-1, 0), (-1, 1)],
    [(1, -1), (1, 0), (1, 1)],
];

fn problem1_solution(input: &Vec<String>) -> usize {
    let mut elf_positions: Vec<Coord> = parse_input(input);
    for t in 0..T {
        display(t, &elf_positions);
        elf_positions = calc_next(&elf_positions, t);
    }

    display(T, &elf_positions);
    let (bl, tr) = range(&elf_positions);
    let area = (tr.0 - bl.0 + 1) * (tr.1 - bl.1 + 1);
    area as usize - elf_positions.len()
}

fn problem2_solution(input: &Vec<String>) -> usize {
    let mut elf_positions: Vec<Coord> = parse_input(input);
    let mut t = 0;
    let mut has_moved: bool = true;
    while has_moved {
        display(t, &elf_positions);
        let next_positions = calc_next(&elf_positions, t);
        has_moved = next_positions != elf_positions;
        elf_positions = next_positions;
        t += 1;
    }
    display(t, &elf_positions);
    t
}

fn parse_input(input: &Vec<String>) -> Vec<Coord> {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .positions(|c| c == '#')
                .map(move |x| (x as i32, y as i32))
        })
        .collect()
}

fn range(elf_positions: &Vec<Coord>) -> (Coord, Coord) {
    let x0 = elf_positions.iter().map(|c| c.0).min().unwrap();
    let x1 = elf_positions.iter().map(|c| c.0).max().unwrap();
    let y0 = elf_positions.iter().map(|c| c.1).min().unwrap();
    let y1 = elf_positions.iter().map(|c| c.1).max().unwrap();
    ((x0, y0), (x1, y1))
}

fn display(t: usize, elf_positions: &Vec<Coord>) {
    println!("{}:", t);
    let (bl, tr) = range(&elf_positions);
    for y in bl.1..=tr.1 {
        for x in bl.0..=tr.0 {
            let c = if elf_positions.contains(&(x, y)) {
                '#'
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!("");
    }
}

fn intentions(elf_positions: &Vec<Coord>, t: usize) -> Vec<Coord> {
    elf_positions
        .iter()
        .map(|&elf| {
            if OFFSETS
                .iter()
                .flatten()
                .any(|o| elf_positions.contains(&(elf.0 + o.0, elf.1 + o.1)))
            {
                (t..(t + 4))
                    .find_map(|tt| {
                        let offsets = OFFSETS[tt & 3];
                        if offsets
                            .iter()
                            .any(|&o| elf_positions.contains(&(elf.0 + o.0, elf.1 + o.1)))
                        {
                            None
                        } else {
                            Some((elf.0 + offsets[1].0, elf.1 + offsets[1].1))
                        }
                    })
                    .unwrap_or(elf)
            } else {
                elf
            }
        })
        .collect_vec()
}

fn calc_next(elf_positions: &Vec<Coord>, t: usize) -> Vec<Coord> {
    let mut proposed = intentions(elf_positions, t);
    let counts = proposed.iter().copied().counts();
    for i in 0..proposed.len() {
        if counts.get(&proposed[i]).copied().unwrap_or(0) > 1 {
            proposed[i] = elf_positions[i];
        }
    }
    proposed
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, 110);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());
        assert_eq!(answer, 20);
    }
}
