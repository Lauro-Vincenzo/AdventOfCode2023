use std::collections::BTreeMap;

use crate::cards::Card;
use crate::cards::Hand;
use strman;

mod cards;

struct BiddingHand{
    hand : Hand,
    bid : u32,
}

fn main() {
    let file_content = strman::read_file("input.txt");
    let mut hands_ranked : BTreeMap<u32, BiddingHand> = BTreeMap::new();

    for line in file_content.lines(){
        let mut split: std::str::SplitWhitespace<'_> = line.split_whitespace();
        let hand_str = split.next().unwrap().to_string();
        let bid_str = split.next().unwrap().to_string();
        let hand = parse_hand(hand_str).expect("Hand Parse Failed!");
        let bid = bid_str.parse::<u32>().expect("Conversion Failed!");
        hands_ranked.insert(hand.compute_score(), BiddingHand{hand,bid});
    }

    let sortedHands = hands_ranked.into_values().rev();

    let mut rank = 1;
    let mut total_score = 0;
    for hand in sortedHands{
        total_score += hand.bid * rank;
    }
}

fn parse_hand(read_string: String) -> Option<Hand>{
    todo!();
}

fn parse_card(read_char: char) -> Card {
    let value = match read_char {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        '1' => 1,
        _ => panic!("Wrong character found!"),
    };
    return Card { value };
}
