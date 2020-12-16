use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solution() {
    let file = File::open("src/day_13/notes.txt").expect("Could not open file");
    let file = BufReader::new(file);
    let lines: Vec<String> = file.lines().map(|line| line.unwrap()).collect();
    let mut buses = vec![];

    for x in lines[1].split(",") {
        if x != "x" {
            buses.push(x.parse::<i64>().unwrap());
        }
        else {
            buses.push(-1);
        }
    }

    //x = index(mod buses[index])

    let mut sum = 0;
    let mut modulus = 1;
    for index in 0..buses.len() {
        if &buses[index] != &-1 {
            let b = index as i64;
            println!("b: {}", b);
            modulus *= buses[index];
            let mut n = 1;
            for bus in &buses {
                if bus != &-1 && bus != &buses[index] {
                    n *= bus;
                }
            }
            println!("n: {}", n);
            let x: i64;
            let first_mod = n % buses[index] as i64;
            println!("first_mod: {}", first_mod);
            let mut counter = 1;
            'find_x: loop {
                let attempt = first_mod * counter;
                if attempt % buses[index] == 1 {
                    x = counter;
                    break 'find_x;
                }
                counter += 1;
            }

            println!("x: {}", x);
            let product = b * n * x;
            sum += product;
        }
    }
    println!("Modulus: {}", modulus);
    println!("Sum: {}", sum);
    let result = sum % modulus;
    println!("Result: {}", result);
    println!("Max: {}", i64::MAX);
}