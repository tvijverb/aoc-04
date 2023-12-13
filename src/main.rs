
// Rust
use std::fs::File;
use std::io::{BufReader, BufRead};

use {
    once_cell::sync::Lazy,
    regex::Regex,
};
use std::collections::HashMap;

// example card
// Card   1:  5 37 16  3 56 11 23 72  7  8 |  3 79 35 45 72 69 15 14 48 88 96 37 11 75 83 56 23  7 16 50 21 91 32 97 17

#[derive(Debug)]
pub struct ScratchCard {
    pub id: u32,
    pub winning_numbers: Vec<u32>,
    pub available_numbers: Vec<u32>,
    pub score: u32,
    pub num_winning_numbers: u32,
    pub num_copies: u32,
}


fn parse_line(line: &str) -> ScratchCard {
    let mut card = ScratchCard {
        id: 0,
        winning_numbers: Vec::new(),
        available_numbers: Vec::new(),
        score: 0,
        num_winning_numbers: 0,
        num_copies: 1,
    };

    // get card id using regex
    static CARDID_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
    let card_id_match = CARDID_REGEX.find(line).unwrap();
    card.id = card_id_match.as_str().parse::<u32>().unwrap();

    // get winning numbers between : and | characters
    let winning_numbers = line.split(":").collect::<Vec<&str>>()[1].split("|").collect::<Vec<&str>>()[0];
    let winning_numbers = winning_numbers.split(" ").collect::<Vec<&str>>();
    for winning_number in winning_numbers {
        if winning_number == "" {
            continue;
        }
        card.winning_numbers.push(winning_number.parse::<u32>().unwrap());
    }

    // get available numbers between | and eol
    let available_numbers = line.split("|").collect::<Vec<&str>>()[1];
    let available_numbers = available_numbers.split(" ").collect::<Vec<&str>>();
    for available_number in available_numbers {
        if available_number == "" {
            continue;
        }
        card.available_numbers.push(available_number.parse::<u32>().unwrap());
    }

    // calculate card score
    for winning_number in &card.winning_numbers {
        if card.available_numbers.contains(winning_number) {
            card.num_winning_numbers += 1;
            if card.score == 0 {
                card.score = 1;
            } else {
                card.score *= 2;
            }
        }
    }
    card
}

fn get_card_copies(card: &ScratchCard, max_card_id: u32, num_copies: i64) -> HashMap<u32, i64> {
    let mut card_copies_hashmap = HashMap::new();
    for n in 1..card.num_winning_numbers + 1{
        if card.id + n > max_card_id {
            continue;
        }
        card_copies_hashmap.entry(card.id + n).or_insert(0);
        let num_copies_card = card_copies_hashmap.get_mut(&(&card.id + &n)).unwrap();
        *num_copies_card += num_copies + 1;
    }
    card_copies_hashmap
}


fn main() -> std::io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut total_score = 0;
    let mut total_card_copies_hashmap: HashMap<u32, i64> = HashMap::new();
    let mut cards = Vec::new();
    let mut max_card_id = 0;
    let mut total_cards = 0;

    for line in reader.lines() {
        let line = line?;
        let card = parse_line(&line);
        total_score += card.score;
        max_card_id += 1;
        cards.push(card);
    }

    // find card copies
    for card in &cards {
        let card_copies_hashmap = get_card_copies(&card, max_card_id, *total_card_copies_hashmap.get(&card.id).unwrap_or(&0));
        for (card_id, num_copies) in card_copies_hashmap {
            total_card_copies_hashmap.entry(card_id).or_insert(0);
            let num_copies_card = total_card_copies_hashmap.get_mut(&card_id).unwrap();
            *num_copies_card += num_copies;
        }
        
    }

    // calculate total cards
    for n in 1..max_card_id + 1 {
        // num copies
        if total_card_copies_hashmap.contains_key(&n) {
            total_cards += total_card_copies_hashmap.get(&n).unwrap();
        }
        // default card we already have
        total_cards += 1;
    }

    println!("total_score: {}", total_score);
    println!("total_cards: {}", total_cards);

    Ok(())
}