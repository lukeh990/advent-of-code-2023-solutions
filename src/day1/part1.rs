use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1() {
    if let Ok(lines) = read_lines("./input.txt") {

        let mut all_numbers: Vec<u32> = Vec::new();

        for line in lines {
            if let Ok(ip) = line {
                let mut num_vec: Vec<u32> = Vec::new();
                for char in ip.chars() {
                    if char.is_digit(10) {
                        if let Some(digit) =  char.to_digit(10) {
                            num_vec.push(digit);
                        }
                    }
                }

                let first_element = num_vec[0];
                let last_element = num_vec[num_vec.len() - 1];

                println!("{0} -> {1} {2}", ip, first_element, last_element);
                all_numbers.push((first_element * 10) + last_element);
            }
        }

        let mut running_total = 0;

        for number in all_numbers.iter() {
            running_total += number;
        }

        println!("{}", running_total);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
