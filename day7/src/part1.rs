use std::collections::HashMap;
use std::error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CardType {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Joker,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone)]
struct HandData {
    cards: Vec<CardType>,
    bid: u16,
}

impl HandData {
    pub fn new(cards: Vec<CardType>, bid: u16) -> HandData {
        HandData { cards, bid }
    }
}

#[derive(Debug, Clone)]
enum HandRanks {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone)]
struct RankedHand {
    rank: HandRanks,
    hand_data: HandData,
}

impl RankedHand {
    pub fn new(rank: HandRanks, hand_data: HandData) -> RankedHand {
        RankedHand { rank, hand_data }
    }
}

pub fn part1() -> Result<()> {
    let hands = create_hand_data("./inputTest.txt")?;
    let ranked = create_ranked_hands(hands)?;
    add_hand_weight(ranked);
    Ok(())
}

fn add_hand_weight(ranked: Vec<RankedHand>) {
    let mut ranked = ranked.clone();

    // TODO: Implement Sorting!!
    
    println!("{:?}", ranked);
}

fn create_ranked_hands(hands: Vec<HandData>) -> Result<Vec<RankedHand>> {
    let mut ranked: Vec<RankedHand> = vec![];

    for hand in hands {
        let cards = hand.cards.clone();
        let mut map: HashMap<CardType, u8> = HashMap::new();

        for card in cards {
            match map.get(&card) {
                None => {
                    map.insert(card, 1);
                }
                Some(value) => {
                    map.insert(card, value + 1);
                }
            }
        }

        let mut values: Vec<u8> = map.values().map(|value| value.to_owned()).collect();
        values.sort_by(|a, b| b.cmp(a));

        if values[0] == 5 {
            ranked.push(RankedHand::new(HandRanks::FiveOfAKind, hand))
        } else if values[0] == 4 {
            ranked.push(RankedHand::new(HandRanks::FourOfAKind, hand))
        } else if values[0] == 3 && values[1] == 2 {
            ranked.push(RankedHand::new(HandRanks::FullHouse, hand))
        } else if values[0] == 3 && values[1] == 1 {
            ranked.push(RankedHand::new(HandRanks::ThreeOfAKind, hand))
        } else if values[0] == 2 && values[1] == 2 {
            ranked.push(RankedHand::new(HandRanks::TwoPair, hand))
        } else if values[0] == 2 {
            ranked.push(RankedHand::new(HandRanks::OnePair, hand))
        } else {
            ranked.push(RankedHand::new(HandRanks::HighCard, hand))
        }
    }

    Ok(ranked)
}

fn create_hand_data<T>(filename: T) -> Result<Vec<HandData>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut hands: Vec<HandData> = vec![];

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let (card_string, bid) = line.split_at(6);

        let bid: u16 = bid.parse()?;

        let mut card_chars = card_string.chars();
        card_chars.next_back();
        let mut cards: Vec<CardType> = vec![];

        for char in card_chars {
            let card_type = match char {
                '2' => CardType::Two,
                '3' => CardType::Three,
                '4' => CardType::Four,
                '5' => CardType::Five,
                '6' => CardType::Six,
                '7' => CardType::Seven,
                '8' => CardType::Eight,
                '9' => CardType::Nine,
                'T' => CardType::Ten,
                'J' => CardType::Joker,
                'Q' => CardType::Queen,
                'K' => CardType::King,
                'A' => CardType::Ace,
                _ => return Err(format!("Non Card Char Detected: \'{}\'", char).into()),
            };

            cards.push(card_type)
        }

        hands.push(HandData::new(cards, bid));
    }

    Ok(hands)
}
