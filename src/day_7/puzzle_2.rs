use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn bag_count(name: &str, parent_number: &u64, bag_map: &HashMap<String, Vec<String>>, counter: &mut u64) -> u64 {
    if bag_map.get_key_value(name).unwrap().1.len() == 0 {
        counter.clone()
    }
    else {
        for bag_name in bag_map.get_key_value(name).unwrap().1 {
            let x = bag_name.chars().nth(0).unwrap().to_digit(10).unwrap() as u64;
            println!("number: {}, parent_number: {}, adding: {}", x, parent_number, x * parent_number);
            *counter = *counter + (x * parent_number);
            bag_count(&bag_name[2..], &(x * parent_number), bag_map, counter);
        }
        counter.clone()
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
        let first_bag = split[1][10..].to_string();
        if first_bag != "no other" {
            list.push(first_bag);
        }
        for bag in 2..split.len() {
            if split[bag].len() > 3 {
                let mut removal_index = 0;
                for index in 0..split[bag].len() {
                    if split[bag].chars().nth(index).unwrap().is_digit(10) {
                        removal_index = index;
                        break;
                    }
                }
                list.push(split[bag][removal_index..].to_string());
            }
        }
        bag_map.insert(name, list);
    }
    println!("{:?}", &bag_map);
    let mut count: u64 = 0;
    bag_count("shiny gold", &1, &bag_map, &mut count);
    println!("Count is {}", count);
}
