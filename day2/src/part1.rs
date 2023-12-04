use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

// Example Line:
//  Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red

pub fn part1() {
    let mut games: Vec<Game> = Vec::new();
    let mut id_total = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(text) = line {
                //  Game 4: 1 green, 3 red, 6 blue, 3 green, 6 red, 3 green, 15 blue, 14 red
                let swap = text.replace(";", ",");
                println!("{}", swap);
                //  Game 4 | 1 green, 3 red, 6 blue, 3 green, 6 red, 3 green, 15 blue, 14 red
                let split1: Vec<&str> = swap.split(": ").collect();

                let id_spit: Vec<&str> = split1[0].split("Game ").collect();
                let id = id_spit[1].parse::<u32>().unwrap_or_default();

                let mut game = Game {
                    id,
                    red: 0,
                    green: 0,
                    blue: 0,
                };

                //  1 green | 3 red | 6 blue | 3 green | 6 red | 3 green | 15 blue | 14 red
                let round_split = split1[1].split(", ");
                for cube_text in round_split {
                    let cube_split: Vec<&str> = cube_text.split(" ").collect();
                    //  1 | green
                    let num_cubes = cube_split[0].parse::<u32>().unwrap();

                    if cube_split[1].eq("red") {
                        if game.red < num_cubes { game.red = num_cubes; }
                    } else if cube_split[1].eq("green") {
                        if game.green < num_cubes { game.green = num_cubes; }
                    } else if cube_split[1].eq("blue") {
                        if game.blue < num_cubes { game.blue = num_cubes; }
                    }
                }

                games.push(game);
            }
        }
    }

    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;

    for game in games.iter_mut() {
        let mut valid = false;

        if game.red <= red_max && game.green <= green_max && game.blue <= blue_max {
            valid = true;
        }

        println!("Game {0} is {1}", game.id, valid);
        println!("  Red Min: {}", game.red);
        println!("  Green Min: {}", game.green);
        println!("  Blue Min: {}", game.blue);

        if valid {
            id_total += game.id;
        }
    }

    println!("{}", id_total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
