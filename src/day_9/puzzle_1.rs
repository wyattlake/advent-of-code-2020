use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solution() {
    let file = File::open("src/day_9/xmas.txt").expect("Could not open file");
    let file = BufReader::new(file);
    let lines: Vec<u64> = file.lines().map(|line| line.unwrap().parse::<u64>().unwrap()).collect();
    'outer_loop: for index in 25..lines.len() {
        let num = lines[index];
        for index1 in (index - 25)..index {
            for index2 in (index - 24)..index {
                if num == lines[index1] + lines[index2] {
                    continue 'outer_loop;
                }
            } 
        }
        println!("{} is invalid", num);
    }
}