use std::{ops::Div, vec};

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
pub enum HandPoint {
    HighCard,
    Pair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
pub struct Card {
    pub value: u32,
}

pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn compute_score(&self) -> f64{
        let main_score = match Self::compute_point(self){
            HandPoint::HighCard  => 100,
            HandPoint::Pair  => 200,
            HandPoint::TwoPairs  => 300,
            HandPoint::ThreeOfAKind  => 400,
            HandPoint::FullHouse  => 500,
            HandPoint::FourOfAKind  => 600,
            HandPoint::FiveOfAKind => 700,
        };

        let mut added_score : f64 = 0.0;
        let mut position_multiplier = 1.0;
        for card in &self.cards{
            added_score = added_score + card.value as f64 * position_multiplier;
            position_multiplier = position_multiplier.div(100.0);
        }
        main_score as f64 + added_score
    }

    pub fn compute_cards_repetitions(cards: &Vec<Card>) -> Vec<i32>{
        let mut unique_cards = cards.clone();
        
        let jokers = cards.iter().filter(|&x| x.value == 1).count();
        //println!("Jokers are {jokers}");

        unique_cards.sort();
        unique_cards.dedup();
        let unique_cards = unique_cards;
        let mut repetitions = vec![];
        for unique_card in unique_cards {
            if unique_card.value == 1{
                continue;
            }
            let occurence = cards.iter().filter(|&x| x == &unique_card).count();
            let total_occurrences = jokers + occurence;
            repetitions.push(total_occurrences as i32);
        }

        // handle 5 J case
        if repetitions.is_empty() {
            repetitions.push(5);
        }
        repetitions.sort();
        repetitions.reverse();
        //println!("Rep is {:?}", repetitions);

        repetitions
    }

    pub fn compute_point(&self) -> HandPoint {
        let _point = HandPoint::HighCard;

        let card_repetitions : Vec<i32> = Self::compute_cards_repetitions(&self.cards);
        
        let point: HandPoint = match card_repetitions[0] {
            1 => HandPoint::HighCard,
            2 => {
                if card_repetitions[1] == 2 {
                    HandPoint::TwoPairs
                } else {
                    HandPoint::Pair
                }
            }
            3 => {
                if card_repetitions.contains(&2){
                    HandPoint::FullHouse
                } else {
                    HandPoint::ThreeOfAKind
                }
            }
            4 => HandPoint::FourOfAKind,
            5 => HandPoint::FiveOfAKind,
            _ => panic!("Found no point!"),
        };

        println!("Point is {:?}", point);
        point
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIZE: usize = 5;

    // cards test structs
    const CARDS_HIGH: [Card; SIZE] = [
        Card { value: 1 },
        Card { value: 2 },
        Card { value: 3 },
        Card { value: 4 },
        Card { value: 5 },
    ];
    const CARDS_FIVE: [Card; SIZE] = [
        Card { value: 1 },
        Card { value: 1 },
        Card { value: 1 },
        Card { value: 1 },
        Card { value: 1 },
    ];
    const CARDS_PAIR: [Card; SIZE] = [
        Card { value: 1 },
        Card { value: 1 },
        Card { value: 2 },
        Card { value: 3 },
        Card { value: 4 },
    ];
    const CARDS_THREE: [Card; SIZE] = [
        Card { value: 1 },
        Card { value: 1 },
        Card { value: 1 },
        Card { value: 3 },
        Card { value: 4 },
    ];
    const CARDS_FOUR: [Card; SIZE] = [
        Card { value: 1 },
        Card { value: 1 },
        Card { value: 1 },
        Card { value: 1 },
        Card { value: 4 },
    ];
    const CARDS_TWO_PAIRS: [Card; SIZE] = [
        Card { value: 1 },
        Card { value: 1 },
        Card { value: 2 },
        Card { value: 2 },
        Card { value: 4 },
    ];
    const CARDS_FULL_HOUSE: [Card; SIZE] = [
        Card { value: 1 },
        Card { value: 1 },
        Card { value: 1 },
        Card { value: 3 },
        Card { value: 3 },
    ];

    // Test get_max_same_card_occurrencies
    #[test]
    fn get_five_occurences() {
        let occurences = Hand::get_max_same_card_occurrencies(&CARDS_FIVE.to_vec());
        assert_eq!(occurences, 5);
    }

    #[test]
    fn get_four_occurences() {
        let occurences = Hand::get_max_same_card_occurrencies(&CARDS_FOUR.to_vec());
        assert_eq!(occurences, 4);
    }

    #[test]
    fn get_three_occurences() {
        let occurences = Hand::get_max_same_card_occurrencies(&CARDS_THREE.to_vec());
        let occurences_2 = Hand::get_max_same_card_occurrencies(&CARDS_FULL_HOUSE.to_vec());

        assert_eq!(occurences, 3);
        assert_eq!(occurences_2, 3);
    }

    #[test]
    fn get_two_occurences() {
        let occurences = Hand::get_max_same_card_occurrencies(&CARDS_TWO_PAIRS.to_vec());
        let occurences_2 = Hand::get_max_same_card_occurrencies(&CARDS_PAIR.to_vec());

        assert_eq!(occurences, 2);
        assert_eq!(occurences_2, 2);
    }

    #[test]
    fn get_one_occurence() {
        let occurences = Hand::get_max_same_card_occurrencies(&CARDS_HIGH.to_vec());
        assert_eq!(occurences, 1);
    }

    // Test compute point
    #[test]
    fn compute_five_of_a_kind() {
        let testHand = Hand {
            cards: CARDS_FIVE.to_vec(),
        };
        let point = testHand.compute_point();
        assert_eq!(point, HandPoint::FiveOfAKind)
    }

    #[test]
    fn compute_four_of_a_kind() {
        let testHand = Hand {
            cards: CARDS_FOUR.to_vec(),
        };
        let point = testHand.compute_point();
        assert_eq!(point, HandPoint::FourOfAKind)
    }
    #[test]
    fn compute_three_of_a_kind() {
        let testHand = Hand {
            cards: CARDS_THREE.to_vec(),
        };
        let point = testHand.compute_point();
        assert_eq!(point, HandPoint::ThreeOfAKind)
    }
    #[test]
    fn compute_pair() {
        let testHand = Hand {
            cards: CARDS_PAIR.to_vec(),
        };
        let point = testHand.compute_point();
        assert_eq!(point, HandPoint::Pair)
    }
    #[test]
    fn compute_two_pairs() {
        let testHand = Hand {
            cards: CARDS_TWO_PAIRS.to_vec(),
        };
        let point = testHand.compute_point();
        assert_eq!(point, HandPoint::TwoPairs)
    }
    #[test]
    fn compute_full_house() {
        let testHand = Hand {
            cards: CARDS_FULL_HOUSE.to_vec(),
        };
        let point = testHand.compute_point();
        assert_eq!(point, HandPoint::FullHouse)
    }
    #[test]
    fn compute_high_card() {
        let testHand = Hand {
            cards: CARDS_HIGH.to_vec(),
        };
        let point = testHand.compute_point();
        assert_eq!(point, HandPoint::HighCard)
    }
}
