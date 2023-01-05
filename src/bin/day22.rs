use std::{
    convert::identity,
    fs::File,
    io::{BufRead, BufReader, Result},
    iter::repeat,
    path::Path,
};

use itertools::Itertools;

const INPUT_FILE: &str = concat!("./data/", env!("CARGO_BIN_NAME"), ".txt");

fn main() -> Result<()> {
    let input_file = File::open(Path::new(INPUT_FILE))?;
    let lines: Vec<String> = BufReader::new(input_file)
        .lines()
        .collect::<Result<Vec<String>>>()?;

    println!("problem1 = {}", problem1_solution(&lines));

    let mut rings: Vec<Vec<(usize, usize, usize)>> = Vec::with_capacity(150);
    for i in 0..50 {
        rings.push(
            (50..150)
                .map(|j| (j, i, 0))
                .chain((0..100).map(|j| (99 - j, 149 - i, 2)))
                .collect(),
        );
        rings.push(
            (0..150)
                .map(|j| (50 + i, j, 1))
                .chain((0..50).map(|j| (49 - j, 150 + i, 2)))
                .collect(),
        );
        rings.push(
            (0..50)
                .map(|j| (50 + j, 50 + i, 0))
                .chain((0..50).map(|j| (100 + i, 49 - j, 3)))
                .chain((0..100).map(|j| (i, 199 - j, 3)))
                .collect(),
        );
    }
    println!("problem2 = {}", problem2_solution(&lines, &rings));
    Ok(())
}

#[derive(Debug)]
enum Op {
    Move(usize),
    TurnLeft,
    TurnRight,
}

const TURNS: [char; 2] = ['L', 'R'];

fn problem1_solution(input: &Vec<String>) -> usize {
    let grid = &input[0..input.len() - 2];
    let ops = parse_ops(input.last().unwrap());
    let mut y = 0;
    let mut x = grid[y].chars().position(|c| c == '.').unwrap();
    let mut d = 0;
    println!("{:9}: ({:2},{:2}) facing {}", "", x, y, d);
    for op in ops {
        match op {
            Op::Move(count) => {
                if d & 1 == 0 {
                    // row move
                    let min_x = grid[y].find(['.', '#']).unwrap();
                    let row = grid[y][min_x..].trim_end().as_bytes();
                    let max_x = min_x + row.len() - 1;
                    let cells = repeat(row).flatten().copied();
                    x = if d == 0 {
                        min_x + scan(cells, x - min_x, count, row.len())
                    } else {
                        max_x - scan(cells.rev(), max_x - x, count, row.len())
                    };
                } else {
                    // col move
                    let min_y = grid
                        .iter()
                        .enumerate()
                        .find(|(_, row)| row.as_bytes().get(x).filter(|c| **c != b' ').is_some())
                        .unwrap()
                        .0;
                    let max_y = grid
                        .iter()
                        .enumerate()
                        .rev()
                        .find(|(_, row)| row.as_bytes().get(x).filter(|c| **c != b' ').is_some())
                        .unwrap()
                        .0;
                    let col = &grid[min_y..=max_y];
                    let cells = repeat(col).flatten().map(|r| r.as_bytes()[x]);
                    y = if d == 1 {
                        min_y + scan(cells, y - min_y, count, col.len())
                    } else {
                        max_y - scan(cells.rev(), max_y - y, count, col.len())
                    };
                }
            }
            Op::TurnLeft => d = (d + 3) & 3,
            Op::TurnRight => d = (d + 1) & 3,
        }
        println!("{:9}: ({:2},{:2}) facing {}", format!("{:3?}", op), x, y, d);
    }
    1004 + y * 1000 + x * 4 + d
}

fn parse_ops(last: &String) -> Vec<Op> {
    let ops: Vec<Op> = last
        .split_inclusive(&TURNS)
        .flat_map(|part| {
            let i = part.len() - 1;
            match part.as_bytes()[i] {
                b'L' => [
                    Some(Op::Move(part[0..i].parse().unwrap())),
                    Some(Op::TurnLeft),
                ],
                b'R' => [
                    Some(Op::Move(part[0..i].parse().unwrap())),
                    Some(Op::TurnRight),
                ],
                _ => [Some(Op::Move(part.parse().unwrap())), None],
            }
        })
        .flat_map(identity)
        .collect();
    ops
}

fn scan<I: Iterator<Item = u8>>(cells: I, start_offset: usize, count: usize, len: usize) -> usize {
    (start_offset
        + cells
            .skip(1 + start_offset)
            .take_while(|c| *c == b'.')
            .take(count)
            .count())
        % len
}

fn problem2_solution(input: &Vec<String>, rings: &[Vec<(usize, usize, usize)>]) -> usize {
    println!("{:?}", rings);
    let mut counts = rings.iter().flatten().map(|(x, y, _)| (x, y)).counts();
    counts.retain(|_, v| *v != 2);
    if !counts.is_empty() {
        panic!("Some cells had incorrect counts: {:?}", counts);
    }

    let ops = parse_ops(input.last().unwrap());

    let initial_x = input[0].find('.').unwrap();
    let (x, y, d) = ops.into_iter().fold((initial_x, 0, 0), |(x, y, d), op| {
        let next = match op {
            Op::Move(count) => {
                let (ring, offset, is_forward) = rings
                    .iter()
                    .find_map(|r| {
                        r.iter()
                            .find_position(|(px, py, pd)| *px == x && *py == y && pd & 1 == d & 1)
                            .map(|(i, (_, _, pd))| (r, i, *pd == d))
                    })
                    .unwrap();
                let cells = repeat(ring).flatten();
                if is_forward {
                    *cells
                        .skip(1 + offset)
                        .take_while(|(px, py, _)| input[*py].as_bytes().get(*px) == Some(&b'.'))
                        .take(count)
                        .last()
                        .unwrap_or(&(x, y, d))
                } else {
                    cells
                        .rev()
                        .skip(ring.len() - offset)
                        .take_while(|(px, py, _)| input[*py].as_bytes().get(*px) == Some(&b'.'))
                        .take(count)
                        .last()
                        .map(|(px, py, pd)| (*px, *py, pd ^ 2))
                        .unwrap_or((x, y, d))
                }
            }
            Op::TurnLeft => (x, y, (d + 3) & 3),
            Op::TurnRight => (x, y, (d + 1) & 3),
        };
        println!(
            "{:9}: ({:2},{:2}) facing {}",
            format!("{:3?}", op),
            next.0,
            next.1,
            next.2
        );
        next
    });
    1004 + y * 1000 + x * 4 + d
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, 6032);
    }

    #[test]
    fn problem2() {
        let mut rings: [Vec<(usize, usize, usize)>; 3 * 4] = Default::default();
        for i in 0..4 {
            rings[0 + i].extend(
                (8..12)
                    .map(|j| (j, i, 0))
                    .chain((0..8).map(|j| (15 - j, 11 - i, 2)))
                    .chain((0..4).map(|j| (4 + i, 7 - j, 3))),
            );
            rings[4 + i].extend(
                (0..12)
                    .map(|j| (11 - i, j, 1))
                    .chain((0..4).map(|j| (i, 7 - j, 3))),
            );
            rings[8 + i].extend(
                (0..12)
                    .map(|j| (j, 4 + i, 0))
                    .chain((0..4).map(|j| (15 - i, 8 + j, 1))),
            );
        }

        let answer = problem2_solution(&load_test_data(), &rings);
        assert_eq!(answer, 5031);
    }
}
