use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Seat {
    pub row_lower: f32,
    pub row_upper: f32,
    pub column_lower: f32,
    pub column_upper: f32,
}

impl Seat {
    pub fn new() -> Seat {
        Seat {
            row_lower: 0.0,
            row_upper: 127.0,
            column_lower: 0.0,
            column_upper: 7.0,
        }
    }

    pub fn divide(&mut self, letter: char) {
        match letter {
            'F' => {
                self.row_upper -= ((self.row_upper - self.row_lower) / 2.0).round();
            }
            'B' => {
                self.row_lower += ((self.row_upper - self.row_lower) / 2.0).round();
            }
            'L' => {
                self.column_upper -= ((self.column_upper - self.column_lower) / 2.0).round();
            }
            'R' => {
                self.column_lower += ((self.column_upper - self.column_lower) / 2.0).round();
            }
            _ => {
                panic!("Invalid input for divide");
            }
        }
    }
}

pub fn solution() {
    let file = File::open("src/day_5/seats.txt").expect("Could not open file");
    let file = BufReader::new(file);
    let lines: Vec<String> = file.lines().map(|line| line.unwrap()).collect();
    let mut seat_rows: Vec<i32> = vec![];
    let mut seat_columns: Vec<i32> = vec![];
    let mut ids: Vec<i32>= vec![];
    for line in &lines {
        let mut seat = Seat::new();
        let chars = line.chars();
        for x in chars {
            seat.divide(x);
        }
        seat_rows.push(seat.row_upper as i32);
        seat_columns.push(seat.column_upper as i32);
        let id = ((seat.row_lower * 8.0) as i32) + seat.column_upper as i32;
        ids.push(id);
    }
    ids.sort();
    for x in 0..1000 {
        if !ids.contains(&x) && ids.contains(&(x + 1)) && ids.contains(&(x - 1)) {
            println!("ID: {}", x);
        }
    }
}
