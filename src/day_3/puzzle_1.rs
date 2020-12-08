use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solution() {
    let file = File::open("src/day_3/map.txt").expect("Could not open file");
    let file = BufReader::new(file);
    let lines: Vec<String> = file.lines().map(|line| line.unwrap()).collect();
    let mut counter = 0;
    for y in 0..lines.len() {
        let x = (y * 3) % lines[0].len();
        println!("x: {}, y: {}", x, y);
        if lines[y].chars().nth(x).unwrap() == '#' {
            counter += 1;
        }
    }
    println!("There are {} trees", counter);
}