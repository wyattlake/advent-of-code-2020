use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solution() {
    let file = File::open("src/day_13/notes.txt").expect("Could not open file");
    let file = BufReader::new(file);
    let lines: Vec<String> = file.lines().map(|line| line.unwrap()).collect();
    let time = lines[0].parse::<i64>().unwrap();
    let mut buses = vec![];
    for x in lines[1].split(",") {
        if x != "x" {
            buses.push(x.parse::<i64>().unwrap());
        }
    }
    let mut min = i64::MAX;
    let mut min_id = 0;
    for bus_num in buses {
        let bus_time = ((time as f32) / (bus_num as f32)).ceil() as i64 * bus_num;
        let wait_time = bus_time - time;
        if wait_time < min {
            min = wait_time;
            min_id = bus_num;
        }
    }
    println!("Result {}", min * min_id);
}