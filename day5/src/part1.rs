use std::fs;
use std::io;
use std::io::{Error, ErrorKind};
use std::ops::Range;
use std::collections::HashMap;

pub fn part1() -> io::Result<()> {
    let almanac = read_almanac()?;

    let mut table: HashMap<u32, Vec<u32>> = HashMap::new();

    for seed in almanac.seeds.iter() {
        let mut seed_vec: Vec<u32> = vec![*seed];

        for (i, map) in almanac.maps.iter().enumerate() {
            let operator = match seed_vec.last().cloned() {
                Some(x) => x,
                None => return Err(Error::new(ErrorKind::Other, "No Last Element"))
            };

            for row in map.rows.iter() {
                if (row.src_range_start..row.src_range_start + row.range_length).contains(&operator) {
                    if *seed >= row.src_range_start {
                        seed_vec.push((seed - row.src_range_start) + row.dest_range_start);
                    }
                }
            }

            if seed_vec.len() != i + 1 {
                seed_vec.push(operator);
            }
        }

        table.insert(*seed, seed_vec);
    }

    println!("{:?}", table);

    Ok(())
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,
    maps: Vec<Map>
}

#[derive(Debug)]
struct Map {
    source_type: String,
    destination_type: String,
    rows: Vec<Row>
}

#[derive(Debug)]
struct Row {
    dest_range_start: u32,
    src_range_start: u32,
    range_length: u32
}

fn read_almanac() -> io::Result<Almanac> {
    let text: String = fs::read_to_string("./inputTest.txt")?;

    let mut seeds: Vec<u32> = vec![]; 
    let mut maps: Vec<Map> = vec![];

    let lines: Vec<String> = text.lines().map(String::from).collect();

    for (row, line) in lines.iter().enumerate() {
        if line.starts_with("seeds: ") {
            let (_, seed_string) = line.split_at(7);
            seeds = seed_string.to_string().split_whitespace().flat_map(|s| s.parse::<u32>()).collect();
        } else if line.starts_with(|c: char| c.is_alphabetic()) {
            let (map_string, _) = line.split_at(line.len() - 5);
            let map_types: Vec<String> = map_string.split("-to-").map(String::from).collect();

            let mut i = row + 1;
            let mut rows: Vec<Row> = vec![];
            while i < lines.len() && !lines[i].eq("") {
                let row_line = lines[i].clone();
                let numbers: Vec<u32> = row_line.split_whitespace().flat_map(|s| s.parse::<u32>()).collect();

                rows.push(Row {
                    dest_range_start: numbers[0],
                    src_range_start: numbers[1],
                    range_length: numbers[2]
                });

                i = i +1;
            }

            maps.push(Map {
                source_type: map_types[0].clone(),
                destination_type: map_types[1].clone(),
                rows
            });
        }
    }

    Ok(Almanac {
        seeds,
        maps
    })
}
