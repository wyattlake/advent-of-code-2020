use std::fs;

pub fn solution() {
    let contents = fs::read_to_string("src/day_4/passports.txt")
        .expect("Something went wrong reading the file");
    let passports: Vec<&str> = contents.split("\n\n").collect();
    let mut counter = 0;
    let list = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for x in passports {
        let mut valid = true;
        for y in list.iter() {
            if !x.contains(y) {
                valid = false;
            }
        }
        if valid { counter += 1; }
    }
    println!("There are {} valid passports", counter);
}