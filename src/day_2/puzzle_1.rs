use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solution() {
    let file = File::open("src/day_2/passwords.txt").expect("Could not open file");
    let file = BufReader::new(file);
    let lines: Vec<String> = file.lines().map(|line| line.unwrap()).collect();
    let mut counter = 0;
    for line in &lines {
        let list: Vec<&str> = line.split(" ").collect();
        let range: Vec<&str> = list[0].split("-").collect();
        let lower_bound = range.first().unwrap().parse::<usize>().unwrap();
        let upper_bound = range.last().unwrap().parse::<usize>().unwrap();
        let key_character = list[1].chars().nth(0).unwrap();
        let passcode = list[2];
        let character_count = passcode.matches(key_character).count();
        if character_count >= lower_bound && character_count <= upper_bound {
            println!("{} is a valid passcode", line);
            counter += 1;
        }
    }
    println!("There were {} correct passcodes", counter);
}