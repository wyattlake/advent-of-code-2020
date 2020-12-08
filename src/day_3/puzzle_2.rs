use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solution() {
    let file = File::open("src/day_3/map.txt").expect("Could not open file");
    let file = BufReader::new(file);
    let lines: Vec<String> = file.lines().map(|line| line.unwrap()).collect();
    let result: u64 = traverse_map(1, 1, &lines) * traverse_map(3, 1, &lines) * traverse_map(5, 1, &lines) * traverse_map(7, 1, &lines) * traverse_map(1, 2, &lines);
    println!("The result is {}", result);
}

fn traverse_map(x_traverse: usize, y_traverse: usize, lines: &Vec<String>) -> u64 {
    let mut counter = 0;
    for y in 0..(lines.len() / y_traverse) {
        let x = (y * x_traverse) % lines[0].len();
        if lines[y * y_traverse].chars().nth(x).unwrap() == '#' {
            counter += 1;
        }
    }
    println!("There are {0} trees traversing with x: {1}, y: {2}", counter, x_traverse, y_traverse);
    counter
}