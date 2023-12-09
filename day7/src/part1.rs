use std::fs;

pub fn part1() -> Result<(), Box<dyn std::error::Error>> {

    Ok(())
}

enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OneCard,
    HighCard
}

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

struct Card {
    card_type: CardType
}

struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
    bid: u32
}

fn read_game() -> Result<Vec<Hand>, Box<dyn std::error::Error>> {
    let hands: Vec<Hand> = vec![];

    let text = fs::read_to_string("./inputTest.txt")?;



    Ok(hands)
}
