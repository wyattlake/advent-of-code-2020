use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn choose(x: i64, y: i64) -> i64 {
    factorial(x)/(factorial(y) * (factorial(x - y)))
}

pub fn factorial(x: i64) -> i64 {
    let mut num = x;
    let mut result = 1;
    while num > 1 {
        result *= num;
        num -= 1;
    }
    result
}

pub fn solution() {
    let file = File::open("src/day_10/adaptors.txt").expect("Could not open file");
    let file = BufReader::new(file);
    let lines: Vec<i64> = file.lines().map(|line| line.unwrap().parse::<i64>().unwrap()).collect();
    let mut adaptors = vec![];
    for adaptor in lines {
        adaptors.push(adaptor);
    }
    adaptors.sort();
    let mut count = 1;
    let mut removable_numbers = 0;
    let mut removed = vec![];
    let mut adaptors_clone = adaptors.clone();

    for x in -((adaptors.len() - 1) as i64)..-2 {
        let index = -x as usize;
        if adaptors[index] - adaptors[index - 2] < 4 {
            println!("Removing {}", adaptors[index - 1]);
            adaptors.remove(index - 1);
            removed.push(adaptors[index - 1]);
            removable_numbers += 1;
        }
    }

    for index in 0..(adaptors.len() - 2) {
        if adaptors_clone[index + 2] - adaptors_clone[index] < 4 {
            if !removed.contains(&adaptors_clone[index + 1]) {
                removed.push(adaptors_clone[index + 1]);
                removable_numbers += 1;
            }
        }
    }

    for x in 0..removable_numbers {
        count += choose(removable_numbers, x);
    }

    println!("Count {}", count);
}