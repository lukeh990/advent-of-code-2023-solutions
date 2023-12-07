use std::fs;
use std::io;

pub fn part1() -> io::Result<()> {
    read_almanac()?;
    Ok(())
}

struct Almanac {
    seeds: Vec<u32>,
    maps: Vec<Map>
}

struct Map {
    source_type: String,
    destination_type: String,
    rows: Vec<Row>
}

struct Row {
    dest_range_start: u32,
    src_range_start: u32,
    range_length: u32
}

fn read_almanac() -> io::Result<Almanac> {
    let text: String = fs::read_to_string("./inputTest.txt")?;

    let mut seeds: Vec<u32> = vec![]; 
    let mut maps: Vec<Map> = vec![];

    for (i, line) in text.lines().enumerate() {
        if line.starts_with("seeds: ") {
            let (_, seed_string) = line.split_at(7);
            seeds = seed_string.to_string().split_whitespace().flat_map(|s| s.parse::<u32>()).collect();
        }

        if line.starts_with(|c: char| c.is_alphabetic()) {
            let (map_string, _) = line.split_at(line.len() - 4);
            let map_types: Vec<String> = map_string.split("-to-").map(String::from).collect();
            
            

            let map = Map {
                source_type: map_types[0],
                destination_type: map_types[1],
                rows
            };
        }
    }

    Ok(Almanac {
        seeds,
        maps
    })
}
