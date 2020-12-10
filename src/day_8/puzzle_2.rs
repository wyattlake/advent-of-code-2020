use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solution() {
    let file = File::open("src/day_8/file.txt").expect("Could not open file");
    let file = BufReader::new(file);
    let lines: Vec<String> = file.lines().map(|line| line.unwrap()).collect();
    for switch_jump in 0..59 {
        let mut current_line: i32 = 1;
        let mut accumulator = 0;
        let mut lines_visited = vec![];
        let mut jump_counter = 0;
        'program: loop {
            let instruction = lines[(current_line - 1) as usize][0..3].to_string();
            if !lines_visited.contains(&(current_line - 1)) {
                match &*instruction {
                    "nop" => {
                        if jump_counter == switch_jump {
                            lines_visited.push(current_line - 1);
                            if lines[(current_line - 1) as usize].chars().nth(4).unwrap() == '+' {
                                current_line += lines[(current_line - 1) as usize][5..].parse::<i32>().unwrap();
                            }
                            else {
                                current_line -= lines[(current_line - 1) as usize][5..].parse::<i32>().unwrap(); 
                            }
                        }
                        else {
                            lines_visited.push(current_line - 1);
                            current_line += 1;
                        }
                        jump_counter += 1;
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
                    }
                    _ => { panic!("Invalid instruction"); }
                }
                if current_line > lines.len() as i32 {
                    println!("acc: {}", accumulator);
                    break 'program;
                }
            }
            else {
                //println!("line: {}", current_line);
                break 'program;
            }
        }
    }
    for switch_jump in 0..238 {
        let mut current_line: i32 = 1;
        let mut accumulator = 0;
        let mut lines_visited = vec![];
        let mut jump_counter = 0;
        'program2: loop {
            let instruction = lines[(current_line - 1) as usize][0..3].to_string();
            if !lines_visited.contains(&(current_line - 1)) {
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
                        //print!("({}, {})", switch_jump, jump_counter);
                        if jump_counter != switch_jump {
                            lines_visited.push(current_line - 1);
                            if lines[(current_line - 1) as usize].chars().nth(4).unwrap() == '+' {
                                current_line += lines[(current_line - 1) as usize][5..].parse::<i32>().unwrap();
                            }
                            else {
                                current_line -= lines[(current_line - 1) as usize][5..].parse::<i32>().unwrap(); 
                            }
                        }
                        else {
                            lines_visited.push(current_line - 1);
                            current_line += 1;
                        }
                        jump_counter += 1;
                    }
                    _ => { panic!("Invalid instruction"); }
                }
                if current_line > lines.len() as i32 {
                    println!("acc: {}", accumulator);
                    break 'program2;
                }
            }
            else {
                //println!("line: {}", current_line);
                break 'program2;
            }
        }
    }
}