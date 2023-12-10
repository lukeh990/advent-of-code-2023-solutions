use std::fs;
use std::error;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn part1() -> Result<()> {
    let _ = read_game();
    Ok(())
}

enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum CardType {
    Ace,
    King,
    Queen,
    Joker,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Card {
    card_type: CardType
}

struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
    rank: u32,
    bid: u32
}

fn read_game() -> Result<Vec<Hand>> {
    let hands: Vec<Hand> = vec![];

    let text = fs::read_to_string("./inputTest.txt")?;

    for line in text.lines() {
        let strings: Vec<String> = line.split_whitespace().map(String::from).collect();
        let hand_string = strings[0].clone();
        let bid: u32 = strings[1].parse()?;

        let mut cards: Vec<Card> = vec![];
        for symbol in hand_string.chars() {
            let card_type = match symbol {
                'A' => CardType::Ace,
                'K' => CardType::King,
                'Q' => CardType::Queen,
                'J' => CardType::Joker,
                'T' => CardType::Ten,
                '9' => CardType::Nine,
                '8' => CardType::Eight,
                '7' => CardType::Seven,
                '6' => CardType::Six,
                '5' => CardType::Five,
                '4' => CardType::Four,
                '3' => CardType::Three,
                '2' => CardType::Two,
                _ => return Err("Bad Symbol".into())
            };

            cards.push(Card {
                card_type
            });
        }

        if cards.len() != 5 {
            return Err("Card number is not 5".into());
        }

        let _ = hand_type(cards);
    }

    Ok(hands)
}

fn hand_type(cards: Vec<Card>) -> Result<HandType> {
    let mut card_counts: HashMap<Card, u32> = HashMap::new();

    for card in cards.iter() {
        let current_count: u32 = match card_counts.get(card) {
            Some(x) => *x,
            None => 0
        };

        card_counts.insert(*card, current_count + 1);
    }
 
    println!("{:?}", card_counts);

    if card_counts.len() == 1 {
        return Ok(HandType::FiveOfAKind);
    } else if card_counts.len() == 2 {
        let first_card = match card_counts.values().nth(0) {
            Some(x) => *x as i32,
            None => return Err("Unable to get value".into())
        };
        
        let second_card = match card_counts.values().nth(1) {
            Some(x) => *x as i32,
            None => return Err("Unable to get value".into())
        };

        if (first_card - second_card).abs() == 3 {
            return Ok(HandType::FourOfAKind);
        } else if (first_card - second_card).abs() == 1 {
            return Ok(HandType::FullHouse);
        }
    } else if card_counts.len() == 3 {
        return Ok(HandType::ThreeOfAKind);
        return Ok(HandType::TwoPair);
    } else if card_counts.len() == 4 {
        return Ok(HandType::OnePair);
    } else if card_counts.len() == 5 {
        return Ok(HandType::HighCard);
    }

    Err(format!("No Valid Hand Type {:?}", card_counts).into())
}
