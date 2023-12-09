use std::io;
use std::io::{Error, ErrorKind};
use std::fs;

pub fn part1() -> io::Result<()> {
    let games = read_games()?;

    let mut ways_to_win: Vec<u32> = vec![];

    for game in games.iter() {
        let mut valid_solutions: Vec<u32> = vec![];

        for speed in 0..=game.time {
            let remaining_time = game.time - speed;
            let distance = remaining_time * speed;
            
            if distance > game.distance {
                valid_solutions.push(speed);
            }
        }

        ways_to_win.push(valid_solutions.len() as u32);
    }

    let prod: u32 = ways_to_win.iter().product();
    println!("{}", prod);

    Ok(())
}

struct Game {
    time: u32,
    distance: u32
}

fn read_games() -> io::Result<Vec<Game>> {
    let text: String = fs::read_to_string("./input.txt")?;

    let lines: Vec<String> = text.lines().map(String::from).collect();
    
    let times: Vec<u32> = lines[0].split_at(10).1.split_whitespace().flat_map(|x| x.parse::<u32>()).collect();
    let distances: Vec<u32> = lines[1].split_at(10).1.split_whitespace().flat_map(|x| x.parse::<u32>()).collect();
    
    if times.len() != distances.len() {
        return Err(Error::new(ErrorKind::Other, "Not the same length"));
    }

    let mut games: Vec<Game> = vec![];

    for (i, time) in times.iter().enumerate() {
        games.push(Game {
            time: *time,
            distance: distances[i]
        });
    }

    Ok(games)
}
