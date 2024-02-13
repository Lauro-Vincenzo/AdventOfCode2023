use std::collections::BTreeMap;
use std::ops::Add;
use ordered_float::OrderedFloat; // 1.0.2

use crate::cards::Card;
use crate::cards::Hand;


mod cards;

fn main() {
    let file_content = strman::read_file("input.txt");
    let mut bids_ranked : BTreeMap<OrderedFloat<f64>, u32> = BTreeMap::new();

    for line in file_content.lines(){
        let mut split: std::str::SplitWhitespace<'_> = line.split_whitespace();
        let hand_str = split.next().unwrap().to_string();
        let bid_str = split.next().unwrap().to_string();
        //print!("Hand {} has bid {}", hand_str, bid_str);
        let hand = parse_hand(hand_str).expect("Hand Parse Failed!");
        //println!(" and is worth {} points.",hand.compute_score());
        let bid = bid_str.parse::<u32>().expect("Conversion Failed!");

        bids_ranked.insert(OrderedFloat(hand.compute_score()), bid);
    }

    println!("Found {} hands.\n", bids_ranked.len());

    let sorted_bids = bids_ranked.into_values();

    let mut rank = 1;
    let mut total_score = 0;
    for bid in sorted_bids{
        //println!("Bid {} with rank {} scores {} points!", bid, rank, bid*rank);
        total_score += bid * rank;
        rank = rank.add(1);
    }
    println!("Total score is: {total_score}");
}

fn parse_hand(read_string: String) -> Option<Hand>{
    let mut cards = Vec::new();
    for char in read_string.chars(){
        cards.push(parse_card(char));
    }

    if cards.is_empty(){
        return None;
    }

    Some(Hand{cards})
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
    Card { value }
}
