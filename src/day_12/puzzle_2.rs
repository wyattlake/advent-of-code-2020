use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn rotate(degrees: i32, east: i32, north: i32) -> (i32, i32) {
    match degrees {
        90 => {
            let temp = east;
            let new_east = north;
            let new_north = -temp;
            (new_east, new_north)
        }
        180 => {
            (-east, -north) 
        }
        270 => {
            let temp = east;
            let new_east = -north;
            let new_north = temp;
            (new_east, new_north) 
        }
        -90 => {
            let temp = east;
            let new_east = -north;
            let new_north = temp;
            (new_east, new_north) 
        }
        -180 => {
            (-east, -north) 
        }
        -270 => {
            let temp = east;
            let new_east = north;
            let new_north = -temp;
            (new_east, new_north)
        }
        _ => {
            panic!("Invalid degrees: {}", degrees);
        }
    }
}

pub fn solution() {
    let file = File::open("src/day_12/instructions.txt").expect("Could not open file");
    let file = BufReader::new(file);
    let lines: Vec<String> = file.lines().map(|line| line.unwrap()).collect();
    let mut waypoint_east = 10;
    let mut waypoint_north = 1;
    let mut ship_east = 0;
    let mut ship_north = 0;
    for line in 0..lines.len() {
        let instruction = lines[line].chars().nth(0).unwrap();
        let number = lines[line][1..].parse::<i32>().unwrap();
        match instruction {
            'F' => {
                ship_east += waypoint_east * number;
                ship_north += waypoint_north * number;
            }
            'N' => {
                waypoint_north += number;
            }
            'S' => {
                waypoint_north -= number;
            }
            'E' => {
                waypoint_east += number;
            }
            'W' => {
                waypoint_east -= number;
            }
            'R' => {
                let new_coords = rotate(number, waypoint_east, waypoint_north);
                waypoint_east = new_coords.0;
                waypoint_north = new_coords.1;
            }
            'L' => {
                let new_coords = rotate(-number, waypoint_east, waypoint_north);
                waypoint_east = new_coords.0;
                waypoint_north = new_coords.1;
            }
            _ => {
                panic!("Invalid instruction: {}", instruction);
            }
        }
    }
    println!("Manhattan Distance: {}", ship_east.abs() + ship_north.abs());
}