use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solution() {
    let file = File::open("src/day_9/xmas.txt").expect("Could not open file");
    let file = BufReader::new(file);
    let lines: Vec<u64> = file.lines().map(|line| line.unwrap().parse::<u64>().unwrap()).collect();
    'outer_loop: for index in 25..lines.len() {
        let num = lines[index];
        for index1 in (index - 25)..index {
            for index2 in (index - 24)..index {
                if num == lines[index1] + lines[index2] {
                    continue 'outer_loop;
                }
            } 
        }
        let mut answer = 0;
        'inner_loop: for starting_index in 0..index {
            let mut sum = lines[starting_index];
            let mut ending_index = starting_index + 1;
            let mut nums = vec![lines[starting_index]];
            while sum <= num {
                sum += lines[ending_index];
                nums.push(lines[ending_index]);
                if sum == num {
                    answer = nums.iter().max().unwrap() + nums.iter().min().unwrap();
                    break 'inner_loop;
                }
                ending_index += 1;
            }
        }
        println!("The weakness is {}", answer);
    }
}