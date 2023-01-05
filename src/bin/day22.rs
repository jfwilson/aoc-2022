use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path, iter::repeat, convert::identity,
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

#[derive(Debug)]
enum Op {
    Move(usize),
    TurnLeft,
    TurnRight
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
                    let cells = repeat(row).flat_map(identity).copied();
                    x = if d == 0 {
                        min_x + scan(cells, x - min_x, count, row.len())
                    } else {
                        max_x - scan(cells.rev(), max_x - x, count, row.len())
                    };
                } else {
                    // col move
                    let min_y = grid.iter().enumerate().find(|(_, row)| row.as_bytes().get(x).filter(|c| **c != b' ').is_some()).unwrap().0;
                    let max_y = grid.iter().enumerate().rev().find(|(_, row)| row.as_bytes().get(x).filter(|c| **c != b' ').is_some()).unwrap().0;
                    let col = &grid[min_y..=max_y];
                    let cells = repeat(col).flat_map(identity).map(|r| r.as_bytes()[x]);
                    y = if d == 1 {
                        min_y + scan(cells, y - min_y, count, col.len())
                    } else {
                        max_y - scan(cells.rev(), max_y - y, count, col.len())
                    };
                }
            },
            Op::TurnLeft => d = (d + 3) & 3,
            Op::TurnRight => d = (d + 1) & 3,
        }
        println!("{:9}: ({:2},{:2}) facing {}", format!("{:3?}", op), x, y, d);
    }
    1004 + y * 1000 + x * 4 + d
}

fn parse_ops(last: &String) -> Vec<Op> {
    let ops: Vec<Op> = last.split_inclusive(&TURNS).flat_map(|part| {
        let i = part.len() - 1;
        match part.as_bytes()[i] {
            b'L' => [Some(Op::Move(part[0..i].parse().unwrap())), Some(Op::TurnLeft)],
            b'R' => [Some(Op::Move(part[0..i].parse().unwrap())), Some(Op::TurnRight)],
            _ => [Some(Op::Move(part.parse().unwrap())), None],
        }
    }).flat_map(identity).collect();
    ops
}

fn scan<I: Iterator<Item = u8>>(cells: I, start_offset: usize, count: usize, len: usize) -> usize {
    (start_offset + cells.skip(1 + start_offset).take_while(|c| *c == b'.').take(count).count()) % len
}

fn problem2_solution(input: &Vec<String>) -> usize {
    input.len()
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
        let answer = problem2_solution(&load_test_data());
        assert_eq!(answer, 5031);
    }
}
