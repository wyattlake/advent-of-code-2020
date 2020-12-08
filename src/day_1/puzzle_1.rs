use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn puzzle_1() {
    let file = File::open("src/day_1/numbers.txt").expect("Could not open file");
    let file = BufReader::new(file);
    let lines: Vec<i32> = file.lines().map(|line| line.unwrap().parse::<i32>().unwrap()).collect();
    for line1 in &lines {
        for line2 in &lines {
            if line1 + line2 == 2020 {
                println!("{0} + {1} = 2020 and {0} * {1} = {2}", &line1, &line2, &(line1 * line2));
            }
        }
    }
}
