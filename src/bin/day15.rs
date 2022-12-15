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

    println!("problem1 = {}", problem1_solution(&lines, 2000000));
    println!("problem2 = {}", problem2_solution(&lines, 4000000));
    Ok(())
}

fn problem1_solution(input: &Vec<String>, y: i32) -> usize {
    let mut occupied: BTreeMap<i32, bool> = BTreeMap::new();
    for line in input {
        let (sx, sy, bx, by) = parse_sensor(line);
        let distance = (bx - sx).abs() + (by - sy).abs();
        println!("{:?}: {:?}: {}", (sx, sy), (bx, by), distance);
        let dx = distance - (y - sy).abs();
        for x in (sx - dx)..=(sx + dx) {
            let is_beacon = *occupied.get(&x).unwrap_or(&false) || (x == bx && y == by);
            occupied.insert(x, is_beacon);
        }
    }
    occupied.values().filter(|is_beacon| !**is_beacon).count()
}

fn problem2_solution(input: &Vec<String>, count: i32) -> i64 {
    let sensors = &input
        .iter()
        .map(|line| {
            let (sx, sy, bx, by) = parse_sensor(line);
            let distance = (bx - sx).abs() + (by - sy).abs();
            (sx, sy, distance)
        })
        .collect_vec();

    'row: for y in 0..=count {
        let mut x: i32 = 0;
        'col: while x <= count {
            for (sx, sy, d) in sensors {
                let dx = d - (y - sy).abs();
                if (x - sx).abs() <= dx {
                    if (sx + dx) >= count {
                        continue 'row;
                    } else {
                        x = sx + dx + 1;
                        continue 'col;
                    }
                }
            }
            println!("{}, {}", x, y);
            return i64::from(x) * 4000000 + i64::from(y);
        }
    }
    0
}

fn parse_sensor(line: &String) -> (i32, i32, i32, i32) {
    line.split(&['=', ',', ':'])
        .filter_map(|s| i32::from_str(s).ok())
        .collect_tuple()
        .unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data(), 10);
        assert_eq!(answer, 26);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data(), 20);
        assert_eq!(answer, 56000011);
    }
}
