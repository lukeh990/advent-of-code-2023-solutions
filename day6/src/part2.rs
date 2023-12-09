use std::fs;

pub fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let game = read_game()?;

    let mut valid_solutions: Vec<u64> = vec![];

    for speed in 0..=game.time {
        let remaining_time = game.time - speed;
        let distance = remaining_time * speed;

        if distance > game.distance {
            valid_solutions.push(speed);
        }
    }

    let prod: u64 = valid_solutions.len() as u64;
    println!("{}", prod);

    Ok(())
}

struct Game {
    time: u64,
    distance: u64
}

fn read_game() -> Result<Game, Box<dyn std::error::Error>> {
    let text: String = fs::read_to_string("./input.txt")?;

    let lines: Vec<String> = text.lines().map(String::from).collect();

    let time: u64 = lines[0].split_at(10).1.chars().filter(|c| !c.is_whitespace()).collect::<String>().parse()?;
    let distance: u64 = lines[1].split_at(10).1.chars().filter(|c| !c.is_whitespace()).collect::<String>().parse()?;

    Ok(Game {
        time,
        distance
    })
}
