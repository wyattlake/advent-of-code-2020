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
        let first_index = range.first().unwrap().parse::<usize>().unwrap() - 1;
        let second_index = range.last().unwrap().parse::<usize>().unwrap() - 1;
        let key_character = list[1].chars().nth(0).unwrap();
        let passcode = list[2];
        let first = passcode.chars().nth(first_index).unwrap();
        let second = passcode.chars().nth(second_index).unwrap();
        if (first != key_character && second == key_character) || (second != key_character && first == key_character){
            println!("{} is a valid passcode", line);
            counter += 1;
        }
    }
    println!("There were {} correct passcodes", counter);
}