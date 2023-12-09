use std::fs;
use std::io;
use std::io::{Error, ErrorKind};
use std::ops::Range;
use rayon::prelude::*;

pub fn part2() -> io::Result<()> {
    let almanac = read_almanac()?;

    let mut locations: Vec<u64> = vec![];

    for seed_range in almanac.seeds.iter() {
        println!("Range {} .. {1}", seed_range.start, seed_range.end);
        let range = seed_range.clone();
        let location_list: Vec<u64> = range.into_par_iter().flat_map(|seed| {
            // println!("Seed {}; Range {1} .. {2}", seed, seed_range.start, seed_range.end);
            let mut seed_vec: Vec<u64> = vec![seed];

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

            Ok(*location)
        }).collect();

        locations.extend(location_list);
    }

    let mut smallest: Option<u64> = None;

    for location in locations.iter() {
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
    seeds: Vec<Range<u64>>,
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

    let mut seeds: Vec<Range<u64>> = vec![]; 
    let mut maps: Vec<Map> = vec![];

    let lines: Vec<String> = text.lines().map(String::from).collect();

    for (row, line) in lines.iter().enumerate() {
        if line.starts_with("seeds: ") {
            let (_, seed_string) = line.split_at(7);
            let seed_list: Vec<u64> = seed_string.to_string().split_whitespace().flat_map(|s| s.parse::<u64>()).collect();
            for (i, seed) in seed_list.iter().enumerate() {
                if i % 2 == 0 {
                    let next = seed_list[i + 1];
                    let range_end = seed + next;

                    seeds.push(*seed..range_end);
                }
            }
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
