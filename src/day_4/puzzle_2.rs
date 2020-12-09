use regex::Regex;
use std::fs;

pub fn solution() {
    let contents = fs::read_to_string("src/day_4/passports.txt")
        .expect("Something went wrong reading the file");
    let passports: Vec<&str> = contents.split("\n\n").collect();
    let mut counter = 0;
    let list = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    'passport_eval: for x in passports {
        let split_space_x: Vec<&str> = x.split(" ").collect();
        for y in list.iter() {
            if !x.contains(y) {
                continue 'passport_eval;
            }
        }
        let mut fully_split_x = vec![];
        for index in 0..split_space_x.len() {
            if split_space_x[index].contains("\n") {
                let removed_string = split_space_x[index];
                let newline_split_string: Vec<&str> = removed_string.split("\n").collect();
                for string in newline_split_string {
                    fully_split_x.push(string);
                }
            }
            else {
                fully_split_x.push(split_space_x[index]);
            }
        }
        for x in fully_split_x {
            match &x[..3] {
                "byr" => {
                    if x[4..8].parse::<i32>().unwrap() < 1920 || x[4..8].parse::<i32>().unwrap() > 2002 {
                        continue 'passport_eval; 
                    }
                }
                "iyr" => {
                    if x[4..8].parse::<i32>().unwrap() < 2010 || x[4..8].parse::<i32>().unwrap() > 2020 {
                        continue 'passport_eval; 
                    }
                }
                "eyr" => {
                    if x[4..8].parse::<i32>().unwrap() < 2020 || x[4..8].parse::<i32>().unwrap() > 2030 {
                        continue 'passport_eval; 
                    }
                }
                "hgt" => {
                    if x.len() < 7 {
                        continue 'passport_eval; 
                    }
                    let mut num_str = String::from(&x[4..]);
                    num_str.pop();
                    num_str.pop();
                    let num = num_str.parse::<i32>().unwrap();
                    let mut ending = String::from("");
                    ending.push(x.chars().nth(x.len() - 2).unwrap());
                    ending.push(x.chars().nth(x.len() - 1).unwrap());
                    match &*ending {
                        "in" => {
                            if num < 59 || num > 76 {
                                continue 'passport_eval
                            } 
                        }
                        "cm" => {
                            if num < 150 || num > 193 {
                                continue 'passport_eval
                            }
                        }
                        _ => continue 'passport_eval, 
                    }
                }
                "hcl" => {
                    let re = Regex::new(r"^[?:a-f|0-9]*?$").unwrap();
                    if x.chars().nth(4).unwrap() != '#' || !re.is_match(&x[5..]) {
                        continue 'passport_eval;
                    }
                }
                "ecl" => {
                    let colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                    let mut is_color = false;
                    if x.len() == 7 {
                        for color in &colors {
                            if x[4..].contains(color) {
                                is_color = true;
                            }
                        }
                    }
                    if !is_color {
                        continue 'passport_eval
                    }
                }
                "pid" => {
                    if x.len() != 13 {
                        continue 'passport_eval
                    }
                }
                _ => (),
            }
        }
        counter += 1;
    }
    println!("There are {} valid passports", counter);
}