use std::cmp;
use std::cmp::Ordering;

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
    pub value: i32,
}

pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn compute_point(&self) -> HandPoint {
        let point = HandPoint::HighCard;

        let unique_cards_number = Hand::get_unique_cards_number(&self.cards);
        let max_card_occurence = Hand::get_max_same_card_occurrencies(&self.cards);

        let point: HandPoint = match unique_cards_number {
            1 => HandPoint::FiveOfAKind,
            2 => {
                if max_card_occurence == 4 {
                    HandPoint::FourOfAKind
                } else {
                    HandPoint::FullHouse
                }
            }
            3 => {
                if max_card_occurence == 3 {
                    HandPoint::ThreeOfAKind
                } else {
                    HandPoint::TwoPairs
                }
            }
            4 => HandPoint::Pair,
            5 => HandPoint::HighCard,
            _ => panic!("Found no point!"),
        };

        return point;
    }

    fn get_max_same_card_occurrencies(cards: &Vec<Card>) -> i32 {
        let mut max_occurrence = 0;

        for card in cards {
            let occurence = cards.into_iter().filter(|&x| x == card).count();
            if occurence > max_occurrence {
                max_occurrence = occurence;
            }
        }

        return i32::try_from(max_occurrence).expect("Conversion Failed!");
    }

    fn get_unique_cards_number(cards: &Vec<Card>) -> i32 {
        let mut unique_cards = cards.clone();

        unique_cards.sort();
        unique_cards.dedup();

        return i32::try_from(unique_cards.len()).expect("Conversion Failed!");
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
