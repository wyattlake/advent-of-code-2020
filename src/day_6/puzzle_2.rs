use std::fs;
use regex::Regex;

#[derive(PartialEq, Debug)]
struct Response {
    character: char,
    count: usize,
}

impl Response {

    pub fn add(character: char, list: &mut Vec<Response>) -> bool {
        for x in list {
            if character == x.character {
                x.count += 1;
                return true
            }
        }
        false
    }

    pub fn create(character: char, list: &mut Vec<Response>) {
        list.push(
            Response {
                character,
                count: 1,
            }
        )
    }

    pub fn get_count(&self) -> usize {
        self.count
    }
}

pub fn solution() {
    let contents = fs::read_to_string("src/day_6/answers.txt")
        .expect("Something went wrong reading the file");
    let passports: Vec<&str> = contents.split("\n\n").collect();
    let mut counter = 0;
    for group in passports {
        println!("len: {}", group.len());
        let chars = group.chars();
        let mut character_list: Vec<Response> = vec![];
        let regex = Regex::new(r"^[?:a-z]*?$").unwrap();
        for character in chars {
            if regex.is_match(&String::from(character)) {
                let in_list = Response::add(character, &mut character_list);
                if !in_list {
                    Response::create(character, &mut character_list);
                }
            }
        }
        println!("list: {:?}", character_list);
        for character in character_list {
            if character.get_count() == group.split("\n").collect::<Vec<&str>>().len() {
                counter += 1;
            }
        }
    }
    println!("There were {} responses.", counter);
}