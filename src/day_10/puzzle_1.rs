use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solution() {
    let file = File::open("src/day_10/adaptors.txt").expect("Could not open file");
    let file = BufReader::new(file);
    let lines: Vec<i32> = file.lines().map(|line| line.unwrap().parse::<i32>().unwrap()).collect();
    let mut adaptors = vec![];
    for adaptor in lines {
        adaptors.push(adaptor);
    }
    adaptors.sort();
    let mut three_counter = 1;
    let mut one_counter = 1;
    for x in 1..adaptors.len() {
        if adaptors[x] - adaptors[x-1] == 3 {
            three_counter += 1;
        }
        else if adaptors[x] - adaptors[x-1] == 1 {
            one_counter += 1;
        }
    }
    println!("Threes: {}", three_counter);
    println!("Ones: {}", one_counter);
    println!("{}", three_counter * one_counter);
}