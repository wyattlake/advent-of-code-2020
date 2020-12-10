use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solution() {
    let file = File::open("src/day_8/file.txt").expect("Could not open file");
    let file = BufReader::new(file);
    let lines: Vec<String> = file.lines().map(|line| line.unwrap()).collect();
    let mut current_line: i32 = 1;
    let mut accumulator = 0;
    let mut lines_visited = vec![];
    'program: loop {
        let instruction = lines[(current_line - 1) as usize][0..3].to_string();
        println!("{} {}", current_line, lines[(current_line - 1) as usize]);
        if !lines_visited.contains(&current_line) {
            match &*instruction {
                "nop" => {
                    lines_visited.push(current_line - 1);
                    current_line += 1;
                }
                "acc" => {
                    lines_visited.push(current_line - 1);
                    if lines[(current_line - 1) as usize].chars().nth(4).unwrap() == '+' {
                        accumulator += lines[(current_line - 1) as usize][5..].parse::<i32>().unwrap();
                    }
                    else {
                        accumulator -= lines[(current_line - 1) as usize][5..].parse::<i32>().unwrap(); 
                    }
                    current_line += 1;
                }
                "jmp" => {
                    lines_visited.push(current_line - 1);
                    if lines[(current_line - 1) as usize].chars().nth(4).unwrap() == '+' {
                        current_line += lines[(current_line - 1) as usize][5..].parse::<i32>().unwrap();
                    }
                    else {
                        current_line -= lines[(current_line - 1) as usize][5..].parse::<i32>().unwrap(); 
                    }
                    println!("jumping to {}", current_line);
                }
                _ => { panic!("Invalid instruction"); }
            }
            if current_line > lines.len() as i32 {
                break 'program;
            }
        }
        else {
            println!("acc: {}", accumulator);
            break 'program;
        }
    }
}