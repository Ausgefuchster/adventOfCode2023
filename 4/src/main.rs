use std::{cmp::min, collections::HashMap, env, fs};

#[derive(Debug, Clone)]
struct Card {
    id: u32,
    winning: Vec<u32>,
    hand: Vec<u32>,
}

fn main() {
    let file = env::args().nth(1).expect("Please supply a file name");
    let input = fs::read_to_string(file).expect("Cannot find file");

    let sum: u32 = input
        .lines()
        .map(build_card)
        .map(|card| {
            let winning_cards = card
                .hand
                .iter()
                .filter(|number| card.winning.contains(number))
                .count();
            if winning_cards == 0 {
                return 0;
            }
            1 << (winning_cards - 1)
        })
        .sum();
    println!("First Sum: {}", sum);

    let cards: Vec<Card> = input.lines().map(build_card).collect();
    let mut map: HashMap<u32, (u32, u32)> = HashMap::new();

    for card in cards.iter() {
        let winning_cards = card
            .hand
            .iter()
            .filter(|number| card.winning.contains(number))
            .count();
        map.insert(card.id, (1_u32, winning_cards as u32));
    }

    for card in cards.iter() {
        let (card_count, winning_cards) = *map.get(&card.id).unwrap();

        for k in card.id + 1..min(cards.len() as u32 + 1, card.id + winning_cards + 1) {
            map.insert(
                k,
                (map.get(&k).unwrap().0 + card_count, map.get(&k).unwrap().1),
            );
        }
    }

    let sum: u32 = map.values().map(|(copies, _)| copies).sum();
    println!("Second Sum: {}", sum);
}

fn build_card(line: &str) -> Card {
    let card_id = get_card_id(line);

    let cards: Vec<String> = line
        .split(':')
        .nth(1)
        .expect("No cards")
        .split('|')
        .map(|card| card.to_string().trim().to_string())
        .collect();

    let winning: Vec<u32> = cards[0]
        .split(' ')
        .filter(|card| !card.is_empty())
        .map(|card| card.parse().expect("Cannot parse card"))
        .collect();
    let hand: Vec<u32> = cards[1]
        .split(' ')
        .filter(|card| !card.is_empty())
        .map(|card| card.parse().expect("Cannot parse card"))
        .collect();

    Card {
        id: card_id,
        winning,
        hand,
    }
}

fn get_card_id(line: &str) -> u32 {
    line.split(':')
        .next()
        .expect("No card id")
        .replace("Card", "")
        .trim()
        .parse()
        .expect("Cannot parse card id")
}
