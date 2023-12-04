use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1() {
    let mut schematic: Vec<Vec<char>> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {

        for line in lines {
            if let Ok(text) = line {
                schematic.push(text.chars().collect());
            }
        }
    }

    let blank = ".".chars().next().unwrap_or_default();
    let mut valid_parts: Vec<u32> = Vec::new();

    for (r, line) in schematic.iter().enumerate() {
        for (c, char) in line.iter().enumerate() {
            if !char.eq(&blank) && !char.is_digit(10) {
                let mut left = &blank;
                let mut right = &blank;

                if c > 0 {
                    left = &line[c - 1];
                }
                if c < line.len() - 1 {
                    right = &line[c + 1];
                }

                let mut upper_left = &blank;
                let mut upper_center = &blank;
                let mut upper_right = &blank;

                let mut lower_left = &blank;
                let mut lower_center = &blank;
                let mut lower_right = &blank;

                if r > 0 {
                    if c > 0 {
                        upper_left = &schematic[r - 1][c - 1];
                    }

                    upper_center = &schematic[r - 1][c];

                    if c < schematic[r - 1].len() - 1 {
                        upper_right = &schematic[r - 1][c + 1];
                    }
                }

                if r < schematic.len() - 1 {
                    if c > 0 {
                        lower_left = &schematic[r + 1][c - 1];
                    }

                    lower_center = &schematic[r + 1][c];

                    if c < schematic[r + 1].len() - 1 {
                        lower_right = &schematic[r + 1][c + 1];
                    }
                }

                if !left.eq(&blank) {
                    valid_parts.push(find_neighbors(&schematic, &blank, r, c - 1));
                }
                if !right.eq(&blank) {
                    valid_parts.push(find_neighbors(&schematic, &blank, r, c + 1));
                }
                if !upper_left.eq(&blank) && upper_center.eq(&blank) {
                    valid_parts.push(find_neighbors(&schematic, &blank, r - 1, c - 1));
                }
                if !upper_right.eq(&blank) && upper_center.eq(&blank) {
                    valid_parts.push(find_neighbors(&schematic, &blank, r - 1, c + 1));
                }
                if !upper_center.eq(&blank) {
                    valid_parts.push(find_neighbors(&schematic, &blank, r - 1, c));
                }
                if !lower_left.eq(&blank) && lower_center.eq(&blank) {
                    valid_parts.push(find_neighbors(&schematic, &blank, r + 1, c - 1));
                }
                if !lower_right.eq(&blank) && lower_center.eq(&blank) {
                    valid_parts.push(find_neighbors(&schematic, &blank, r + 1, c + 1));
                }
                if !lower_center.eq(&blank) {
                    valid_parts.push(find_neighbors(&schematic, &blank, r + 1, c));
                }

                // println!("{0} {1} {2}", upper_left, upper_center, upper_right);
                // println!("{0} {1} {2}", left, char, right);
                // println!("{0} {1} {2}", lower_left, lower_center, lower_right);
                // println!("");
            }
        }
    }

    println!("{:?}", valid_parts);
    println!("{}", valid_parts.iter().sum::<u32>());
}

fn find_neighbors(schematic: &Vec<Vec<char>>, blank: &char, row: usize, col: usize) -> u32 {
    let line = &schematic[row];

    let center = &line[col];
    let mut left2 = blank;
    let mut left = blank;
    let mut right = blank;
    let mut right2 = blank;
    // . . x . .
    if col > 1 {
        left2 = &line[col - 2];
    }
    if col > 0 {
        left = &line[col - 1];
    }
    if col < line.len() - 1 {
        right = &line[col + 1];
    }
    if col < line.len() - 2 {
        right2 = &line[col + 2];
    }

    let mut hundreds: u32 = 0;
    let tens: u32;
    let ones: u32;

    if left2.is_digit(10) && left.is_digit(10) {
        // x x x . .
        hundreds = left2.to_digit(10).unwrap_or_default() * 100;
        tens = left.to_digit(10).unwrap_or_default() * 10;
        ones = center.to_digit(10).unwrap_or_default();
    } else if right2.is_digit(10) && right.is_digit(10) {
        // . . x x x
        hundreds = center.to_digit(10).unwrap_or_default() * 100;
        tens = right.to_digit(10).unwrap_or_default() * 10;
        ones = right2.to_digit(10).unwrap_or_default();
    } else {
        if right.is_digit(10) && left.is_digit(10) {
            // . x x x .
            hundreds = left.to_digit(10).unwrap_or_default() * 100;
            tens = center.to_digit(10).unwrap_or_default() * 10;
            ones = right.to_digit(10).unwrap_or_default();
        } else if !right.is_digit(10) {
            // . x x . .
            tens = left.to_digit(10).unwrap_or_default() * 10;
            ones = center.to_digit(10).unwrap_or_default();
        } else {
            // . . x x .
            tens = center.to_digit(10).unwrap_or_default() * 10;
            ones = right.to_digit(10).unwrap_or_default();
        }
    }

    let combined = hundreds + tens + ones;
    return combined;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
