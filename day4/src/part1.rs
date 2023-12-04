use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1() -> std::io::Result<()> {
    let cards = read_cards()?;

    let mut total_points: u32 = 0;

    //Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    for card in cards.iter() {
        let mut points: u32 = 0;

        for number in card.actual_numbers.iter() {
            if number == &0 { continue }
            if card.winning_numbers.iter().any(|x| x == number) {
                if points == 0 {
                    points = 1;
                } else {
                    points = points * 2;
                }
            }
        }

        println!("Game {} has {1} points", card.id, points);
        total_points += points;
    }
    println!("{}", total_points);

    Ok(())
}

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    actual_numbers: Vec<u32>
}

fn read_cards() -> io::Result<Vec<Card>> {
    let mut cards: Vec<Card> = vec![]; 

    let lines = read_lines("./input.txt")?;
    for line in lines {
        let text = line?;

        let split1: Vec<String> = text.split(": ").map(|s| s.to_string()).collect();
        let split2: Vec<String> = split1[1].split(" | ").map(|s| s.to_string()).collect();

        let id_string: String = match split1[0].split(" ").map(String::from).collect::<Vec<String>>().last() {
            Some(id_string) => id_string.clone(),
            None => "".to_string()
        };
        let id: u32 = id_string.parse::<u32>().unwrap_or_default();
        //println!("{}", id);
        let winning_numbers: Vec<u32> = split2[0].split(" ").map(|s| s.parse::<u32>().unwrap_or_default()).collect();
        let actual_numbers: Vec<u32> = split2[1].split(" ").map(|s| s.parse::<u32>().unwrap_or_default()).collect();
        
        cards.push(Card {
            id,
            winning_numbers,
            actual_numbers
        });
    }
    Ok(cards)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
