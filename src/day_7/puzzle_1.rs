use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn find_gold(name: &str, bag_map: &HashMap<String, Vec<String>>) -> bool {
    if name == "shiny gold" {
        true
    }
    else if bag_map.get_key_value(name).unwrap().1.len() == 0 {
        false
    }
    else {
        for bag_name in bag_map.get_key_value(name).unwrap().1 {
            if find_gold(bag_name, bag_map) {
                return true;
            }
        }
        false
    }
}

pub fn solution() {
    let file = File::open("src/day_7/rules.txt").expect("Could not open file");
    let file = BufReader::new(file);
    let lines: Vec<String> = file.lines().map(|line| line.unwrap()).collect();
    let mut bag_map: HashMap<String, Vec<String>> = HashMap::new();
    for line in &lines {
        let split: Vec<&str> = line.split(" bag").collect();
        let name = split[0].to_string();
        let mut list = vec![];
        let first_bag = split[1][12..].to_string();
        if first_bag != " other" {
            list.push(first_bag);
        }
        for bag in 2..split.len() {
            if split[bag].len() > 3 {
                let mut removal_index = 0;
                for index in 0..split[bag].len() {
                    if split[bag].chars().nth(index).unwrap().is_digit(10) {
                        removal_index = index + 2;
                        break;
                    }
                }
                list.push(split[bag][removal_index..].to_string());
            }
        }
        bag_map.insert(name, list);
    }
    let mut can_hold = vec![];
    for bag in &bag_map {
        if bag.0 != "shiny gold" {
            let found_gold = find_gold(bag.0, &bag_map);
            if found_gold {
                can_hold.push(bag.0);
            }
        }
    }
    println!("{} bags can hold shiny gold", can_hold.len());
}