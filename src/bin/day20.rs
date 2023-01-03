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

fn problem1_solution(input: &Vec<String>) -> isize {
    decrypt(input, 1, 1)
}

fn problem2_solution(input: &Vec<String>) -> isize {
    decrypt(input, 811589153, 10)
}

fn decrypt(input: &Vec<String>, multiplier: isize, mix_count: usize) -> isize {
    let data: Vec<isize> = input
        .iter()
        .map(|line| line.parse::<isize>().unwrap() * multiplier)
        .collect();
    let zero_id = index_of(&data, 0);
    let mut ids: Vec<usize> = (zero_id + 1..data.len()).chain(0..=zero_id).collect();
    for _ in 0..mix_count {
        apply_mixing(&mut ids, &data);
    }
    [1000, 2000, 3000]
        .iter()
        .map(|index| data[ids[(index - 1) % ids.len()]])
        .sum()
}

fn apply_mixing(ids: &mut Vec<usize>, data: &Vec<isize>) {
    let wrap = ids.len() as isize - 1;
    for (id, &shift) in data.iter().enumerate() {
        if shift != 0 {
            let old_pos = index_of(ids, id);
            let new_pos = (old_pos as isize + shift).rem_euclid(wrap) as usize;
            ids.remove(old_pos);
            ids.insert(new_pos, id);
        }
    }
}

fn index_of<T: PartialEq + Copy>(items: &Vec<T>, value: T) -> usize {
    items.iter().position(|&item| item == value).unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "1
2
-3
3
-2
0
4";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, 3);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());
        assert_eq!(answer, 1623178306);
    }
}
