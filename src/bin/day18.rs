use itertools::Itertools;
use std::{
    collections::HashMap,
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

    println!("problem1 = {}", problem1_solution(&lines));
    println!("problem2 = {}", problem2_solution(&lines));
    Ok(())
}

fn problem1_solution(input: &Vec<String>) -> usize {
    solve(input, false)
}

fn problem2_solution(input: &Vec<String>) -> usize {
    solve(input, true)
}

fn solve(input: &Vec<String>, fill_air_pockets: bool) -> usize {
    let mut coords: Vec<[i32; 3]> = input.iter().map(parse_line).collect();
    coords.sort_unstable();

    if fill_air_pockets {
        let min = [0, 1, 2].map(|axis| coords.iter().map(|c| c[axis]).min().unwrap());
        let max = [0, 1, 2].map(|axis| coords.iter().map(|c| c[axis]).max().unwrap());
        let mut air_cells: HashMap<[i32; 3], bool> = HashMap::new();
        for x in (min[0] + 1)..max[0] {
            for y in (min[1] + 1)..max[1] {
                for z in (min[2] + 1)..max[2] {
                    let p = [x, y, z];
                    if coords.binary_search(&p).is_err() && !air_cells.contains_key(&p) {
                        let (new_air_cells, is_external) = search_air(p, &coords, min, max);
                        air_cells.extend(new_air_cells.into_iter().map(|c| (c, is_external)));
                    }
                }
            }
        }
        air_cells.retain(|_, is_external| !*is_external);
        coords.extend(air_cells.keys());
        coords.sort_unstable();
    }

    let mut surface_area = 0;
    for coord in coords.iter() {
        surface_area += 6;
        for axis in 0..3 {
            for offset in [-1, 1] {
                let mut neighbour = *coord;
                neighbour[axis] += offset;
                if coords.binary_search(&neighbour).is_ok() {
                    surface_area -= 1;
                }
            }
        }
    }
    surface_area
}

fn search_air(
    origin: [i32; 3],
    rocks: &Vec<[i32; 3]>,
    min: [i32; 3],
    max: [i32; 3],
) -> (Vec<[i32; 3]>, bool) {
    let mut this_round = vec![origin];
    let mut next_round = Vec::new();
    let mut air_cells = this_round.clone();
    let mut is_external: bool = false;
    while !this_round.is_empty() {
        for coord in this_round.iter() {
            for axis in 0..3 {
                for offset in [-1, 1] {
                    let mut p = *coord;
                    p[axis] += offset;
                    if rocks.binary_search(&p).is_ok() {
                        // rock
                    } else if p[axis] <= min[axis] || p[axis] >= max[axis] {
                        // reached edge of bounds - this is external air
                        is_external = true;
                    } else {
                        match air_cells.binary_search(&p) {
                            Ok(_) => (), // already visited
                            Err(index) => {
                                // new air cell
                                air_cells.insert(index, p);
                                next_round.push(p);
                            }
                        }
                    }
                }
            }
        }
        this_round.clear();
        swap(&mut this_round, &mut next_round);
    }
    (air_cells, is_external)
}

fn parse_line(line: &String) -> [i32; 3] {
    let tuple: (i32, i32, i32) = line
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect_tuple()
        .unwrap();
    [tuple.0, tuple.1, tuple.2]
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, 64);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());
        assert_eq!(answer, 58);
    }
}
