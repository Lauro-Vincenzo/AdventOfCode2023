use crate::cards::Card;
use strman;

mod cards;

fn main() {
    strman::read_file("input.txt");
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
