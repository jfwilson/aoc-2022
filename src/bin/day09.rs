use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    ops::{AddAssign, Neg},
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

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Default)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    const fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }

    fn signum(&self) -> Coord {
        Coord {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
}

impl Neg for Coord {
    type Output = Self;
    fn neg(self) -> Self {
        Coord {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl AddAssign<&Coord> for Coord {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

fn problem1_solution(input: &Vec<String>) -> usize {
    solve::<2>(input)
}

fn problem2_solution(input: &Vec<String>) -> usize {
    solve::<10>(input)
}

fn solve<const N: usize>(input: &Vec<String>) -> usize
where
    [Coord; N]: Default,
{
    let mut tail_visits: Vec<Coord> = Vec::new();
    let mut offsets: [Coord; N] = Default::default();
    for line in input {
        let direction = &match line.as_bytes()[0] {
            b'L' => Coord::new(-1, 0),
            b'R' => Coord::new(1, 0),
            b'U' => Coord::new(0, 1),
            _ => Coord::new(0, -1),
        };
        let count = usize::from_str(&line[2..]).unwrap();
        for _ in 0..count {
            offsets[0] += direction;
            for i in 0..(N - 1) {
                let upstream_offset = &offsets[i];
                if upstream_offset.x.abs().max(upstream_offset.y.abs()) > 1 {
                    let signum = upstream_offset.signum();
                    offsets[i + 1] += &signum;
                    offsets[i] += &-signum;
                }
            }
            tail_visits.push(offsets[N - 1].clone());
        }
    }
    tail_visits.sort_unstable();
    tail_visits.dedup();
    tail_visits.len()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    fn load_test_data(input: &str) -> Vec<String> {
        input.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data(INPUT));
        assert_eq!(answer, 13);
    }

    const LARGER_INPUT: &'static str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data(INPUT));
        assert_eq!(answer, 1);

        let answer = problem2_solution(&load_test_data(LARGER_INPUT));
        assert_eq!(answer, 36);
    }
}
