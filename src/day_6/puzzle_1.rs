use std::fs;
use regex::Regex;

pub fn solution() {
    let contents = fs::read_to_string("src/day_6/answers.txt")
        .expect("Something went wrong reading the file");
    let passports: Vec<&str> = contents.split("\n\n").collect();
    let mut counter = 0;
    for group in passports {
        println!("len: {}", group.len());
        let chars = group.chars();
        let mut character_list = vec![];
        let regex = Regex::new(r"^[?:a-z]*?$").unwrap();
        for character in chars {
            if regex.is_match(&String::from(character)) && !character_list.contains(&character) {
                character_list.push(character);
            }
        }
        counter += character_list.len();
    }
    println!("There were {} responses.", counter);
}