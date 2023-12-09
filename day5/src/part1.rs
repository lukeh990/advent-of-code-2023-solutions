use std::fs;
use std::io;
use std::io::{Error, ErrorKind};
use std::collections::HashMap;

pub fn part1() -> io::Result<()> {
    let almanac = read_almanac()?;

    let mut locations: HashMap<u64, u64> = HashMap::new();

    for seed in almanac.seeds.iter() {
        let mut seed_vec: Vec<u64> = vec![*seed];

        for (i, map) in almanac.maps.iter().enumerate() {
            let operator = match seed_vec.last().cloned() {
                Some(x) => x,
                None => return Err(Error::new(ErrorKind::Other, "No Last Element"))
            };

            for row in map.rows.iter() {
                if (row.src_range_start..row.src_range_start + row.range_length).contains(&operator) {
                    let offset = match (operator - row.src_range_start).checked_add(row.dest_range_start) {
                        Some(x) => x,
                        None => return Err(Error::new(ErrorKind::Other, format!("OVERFLOW {} + {1}", operator - row.src_range_start, row.dest_range_start)))
                    };

                    seed_vec.push(offset);
                }
            }

            if seed_vec.len() != i + 2 {
                seed_vec.push(operator);
            }
        }

        let location = match seed_vec.last() {
            Some(x) => x,
            None => return Err(Error::new(ErrorKind::Other, "No Last Element"))
        };

        locations.insert(*seed, *location);
    }

    let mut smallest: Option<u64> = None;

    for (_, location) in locations.iter() {
        if smallest.is_none() {
            smallest = Some(*location);
        } else if smallest.is_some_and(|x| x > *location) {
            smallest = Some(*location);
        }
    }

    println!("{}", smallest.unwrap());

    Ok(())
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>
}

#[derive(Debug)]
struct Map {
    rows: Vec<Row>
}

#[derive(Debug)]
struct Row {
    dest_range_start: u64,
    src_range_start: u64,
    range_length: u64
}

fn read_almanac() -> io::Result<Almanac> {
    let text: String = fs::read_to_string("./input.txt")?;

    let mut seeds: Vec<u64> = vec![]; 
    let mut maps: Vec<Map> = vec![];

    let lines: Vec<String> = text.lines().map(String::from).collect();

    for (row, line) in lines.iter().enumerate() {
        if line.starts_with("seeds: ") {
            let (_, seed_string) = line.split_at(7);
            seeds = seed_string.to_string().split_whitespace().flat_map(|s| s.parse::<u64>()).collect();
        } else if line.starts_with(|c: char| c.is_alphabetic()) {
            let mut i = row + 1;
            let mut rows: Vec<Row> = vec![];
            while i < lines.len() && !lines[i].eq("") {
                let row_line = lines[i].clone();
                let numbers: Vec<u64> = row_line.split_whitespace().flat_map(|s| s.parse::<u64>()).collect();

                rows.push(Row {
                    dest_range_start: numbers[0],
                    src_range_start: numbers[1],
                    range_length: numbers[2]
                });

                i = i +1;
            }

            maps.push(Map {
                rows
            });
        }
    }

    Ok(Almanac {
        seeds,
        maps
    })
}
