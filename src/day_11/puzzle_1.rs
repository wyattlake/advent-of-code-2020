use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn clamp(value: usize, min: usize, max: usize) -> usize {
    if value > max {
        max
    }
    else if value < min {
        min
    }
    else {
        value
    }
}

pub fn neighbor_count(input_x: usize, input_y: usize, seating: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for x in (clamp(input_x, 1, usize::MAX) - 1)..(clamp(input_x, 0, seating[0].len() - 2) + 2) {
        for y in (clamp(input_y, 1, usize::MAX) - 1)..(clamp(input_y, 0, seating.len() - 2) + 2) {
            if seating[y][x] == '#' || seating[y][x] == '-' {
                count += 1;
            }
        }
    }
    if seating[input_y][input_x] == '#' {
        count -= 1;
    }
    count
}

pub fn compute_seating(seating: &mut Vec<Vec<char>>) {
    for x in 0..seating[0].len() {
        for y in 0..seating.len() {
            let count = neighbor_count(x, y, seating);
            if seating[y][x] == '#' {
                if count >= 4 {
                    seating[y][x] = '-';
                }
            }
            else if seating[y][x] == 'L' {
                if count == 0 {
                    seating[y][x] = '+';
                }
            }
        }
    }
    for x in 0..seating[0].len() {
        for y in 0..seating.len() {
            if seating[y][x] == '-' {
                seating[y][x] = 'L';
            }
            else if seating[y][x] == '+' {
                seating[y][x] = '#';
            }
        }
    }
}

pub fn solution() {
    let file = File::open("src/day_11/seating.txt").expect("Could not open file");
    let file = BufReader::new(file);
    let mut seating: Vec<Vec<char>> = file.lines().map(|line| line.unwrap().chars().collect()).collect();
    let mut past_seating: Vec<Vec<char>> = vec![vec![]];
    while &past_seating != &seating {
        past_seating = seating.clone();
        compute_seating(&mut seating);
    }
    let mut occupied_seat_count = 0;
    for x in 0..seating[0].len() {
        for y in 0..seating.len() {
            if seating[y][x] == '#' {
                occupied_seat_count += 1;
            }
        }
    }
    println!("Answer: {}", occupied_seat_count);
}