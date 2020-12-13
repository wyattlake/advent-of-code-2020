use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;

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
    'loop1: for x in (clamp(input_x, 0, seating[0].len() - 1) + 1)..seating[0].len() {
        if seating[input_y][x] == '#' || seating[input_y][x] == '-' {
            count += 1;
            break 'loop1;
        }
        else if seating[input_y][x] == 'L' || seating[input_y][x] == '+' {
            break 'loop1;
        }
    }
    'loop2: for x in 1..(clamp(input_x + 1, 1, usize::MAX)) {
        let flipped_x = clamp(input_x, 1, usize::MAX) - x;
        if seating[input_y][flipped_x] == '#' || seating[input_y][flipped_x] == '-' {
            count += 1;
            break 'loop2;
        }
        else if seating[input_y][flipped_x] == 'L' || seating[input_y][flipped_x] == '+' {
            break 'loop2;       
        }
    }
    'loop3: for y in (clamp(input_y, 0, seating.len() - 1) + 1)..seating.len() {
        if seating[y][input_x] == '#' || seating[y][input_x] == '-' {
            count += 1;
            break 'loop3;
        }
        else if seating[y][input_x] == 'L' || seating[y][input_x] == '+' {
            break 'loop3;       
        }
    }
    'loop4: for y in 1..clamp(input_y + 1, 1, usize::MAX) {
        let flipped_y = clamp(input_y, 1, usize::MAX) - y;
        if seating[flipped_y][input_x] == '#' || seating[flipped_y][input_x] == '-' {
            count += 1;
            break 'loop4;
        }
        else if seating[flipped_y][input_x] == 'L' || seating[flipped_y][input_x] == '+' {
            break 'loop4;       
        }
    }
    'loop5: for factor in 1..cmp::min(seating[0].len() - input_x, seating.len() - input_y) {
        if seating[input_y + factor][input_x + factor] == '#' || seating[input_y + factor][input_x + factor] == '-' {
            count += 1;
            break 'loop5;
        }
        else if seating[input_y + factor][input_x + factor] == 'L' || seating[input_y + factor][input_x + factor] == '+' {
            break 'loop5;       
        } 
    }
    'loop6: for factor in 1..cmp::min(input_x + 1, input_y + 1) {
        if seating[input_y - factor][input_x - factor] == '#' || seating[input_y - factor][input_x - factor] == '-' {
            count += 1;
            break 'loop6;
        }
        else if seating[input_y - factor][input_x - factor] == 'L' || seating[input_y - factor][input_x - factor] == '+' {
            break 'loop6;       
        } 
    }
    'loop7: for factor in 1..cmp::min(input_x + 1, seating.len() - input_y) {
        if seating[input_y + factor][input_x - factor] == '#' || seating[input_y + factor][input_x - factor] == '-' {
            count += 1;
            break 'loop7;
        }
        else if seating[input_y + factor][input_x - factor] == 'L' || seating[input_y + factor][input_x - factor] == '+' {
            break 'loop7;       
        } 
    }
    'loop8: for factor in 1..cmp::min(seating[0].len() - input_x, input_y + 1) {
        if seating[input_y - factor][input_x + factor] == '#' || seating[input_y - factor][input_x + factor] == '-' {
            count += 1;
            break 'loop8;
        }
        else if seating[input_y - factor][input_x + factor] == 'L' || seating[input_y - factor][input_x + factor] == '+' {
            break 'loop8;       
        } 
    }
    count
}

pub fn compute_seating(seating: &mut Vec<Vec<char>>) {
    for x in 0..seating[0].len() {
        for y in 0..seating.len() {
            let count = neighbor_count(x, y, seating);
            if seating[y][x] == '#' {
                if count >= 5 {
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