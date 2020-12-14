use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn forward(number: i32, direction_raw: &i32, east: &mut i32, north: &mut i32) {
    let direction = direction_raw % 360;
    match direction {
        0 => {
            *east += number;
        }
        90 => {
            *north -= number; 
        }
        180 => {
            *east -= number; 
        }
        270 => {
            *north += number; 
        }
        -90 => {
            *north += number; 
        }
        -180 => {
            *east -= number; 
        }
        -270 => {
            *north -= number; 
        }
        _ => {
            panic!("Invalid direction: {}", direction);
        }
    }
}

pub fn solution() {
    let file = File::open("src/day_12/instructions.txt").expect("Could not open file");
    let file = BufReader::new(file);
    let lines: Vec<String> = file.lines().map(|line| line.unwrap()).collect();
    let mut east = 0;
    let mut north = 0;
    let mut direction = 0;
    for line in 0..lines.len() {
        let instruction = lines[line].chars().nth(0).unwrap();
        let number = lines[line][1..].parse::<i32>().unwrap();
        println!("number: {}", number);
        match instruction {
            'F' => {
                forward(number, &direction, &mut east, &mut north);
            }
            'N' => {
                north += number;
            }
            'S' => {
                north -= number;
            }
            'E' => {
                east += number;
            }
            'W' => {
                east -= number;
            }
            'R' => {
                direction += number;
            }
            'L' => {
                direction -= number;
            }
            _ => {
                panic!("Invalid instruction: {}", instruction);
            }
        }
        println!("east: {}, north: {}", east, north);
    }
    println!("Manhattan Distance: {}", east.abs() + north.abs());
}