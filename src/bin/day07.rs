use std::{
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

    println!("problem1 = {}", problem1_solution(&lines));
    println!("problem2 = {}", problem2_solution(&lines));
    Ok(())
}

struct Dir {
    total_size: usize
}

impl Dir {
    const fn new() -> Dir {
        Dir { total_size: 0 }
    }

    fn add_file(&mut self, size: usize) {
        self.total_size += size;
    }
}

fn parse_dirs(input: &Vec<String>) -> Vec<Dir> {
    let mut finished_dirs: Vec<Dir> = Vec::new();
    let mut cwd: Vec<Dir> = Vec::new();
    for line in input.iter() {
        if line.eq("$ cd ..") {
            let dir = cwd.pop().unwrap();
            finished_dirs.push(dir);
        } else if line.starts_with("$ cd ") {
            let dir = Dir::new();
            cwd.push(dir);
        } else if let Some(size) = line.split(' ').next().and_then(|size| usize::from_str(size).ok()) {
            for dir in cwd.iter_mut() {
                dir.add_file(size);
            }
        }
    }
    finished_dirs.extend(cwd.drain(0..));
    finished_dirs
}

fn problem1_solution(input: &Vec<String>) -> usize {
    parse_dirs(input).into_iter().map(|dir| dir.total_size).filter(|size| *size <= 100000).sum()
}

fn problem2_solution(input: &Vec<String>) -> usize {
    let dirs = &parse_dirs(input);
    let used_space = dirs.into_iter().map(|dir| dir.total_size).max().unwrap();
    let space_needed_to_free = used_space - 40000000;
    dirs.into_iter().map(|dir| dir.total_size).filter(|size| *size >= space_needed_to_free).min().unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, 95437);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());

        assert_eq!(answer, 24933642);
    }
}
