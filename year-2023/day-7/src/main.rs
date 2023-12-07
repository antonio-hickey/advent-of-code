/* Day - 7: Camel Cards */

use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

fn main() {
    // Read puzzle input into string
    let input = fs::read_to_string("./puzzle_data.txt").expect("some puzzle input data");
    println!("part one solution: {}", solution(&input, 1));
}

/// Function to solve for both parts of the puzzle
fn solution(input: &str, part: i8) -> i64 {
    // parse the puzzle input into a useful model
    let mut camel_cards = CamelCards::from_puzzle_input(input);

    match part {
        1 => {
            /* Part One Solution */
            // Sort the card hands by score, then iterate over each hand
            // and compute the total winnings by calculating a product of
            // each card hands rank and bid/bet and accumeulating into a total.
            camel_cards.hands.sort_by(|a, b| a.score.cmp(&b.score));
            camel_cards
                .hands
                .iter()
                .enumerate()
                .fold(0, |acc, (rank, hand)| acc + (rank + 1) as i64 * hand.bid)
        }
        2 => {
            /* Part Two Solution */
            todo!()
        }
        _ => panic!("Only 2 parts to the puzzle broooo"),
    }
}

#[derive(Debug)]
/// A useful model of the puzzle input data (poker game)
struct CamelCards {
    hands: Vec<CamelCardHand>,
}
impl CamelCards {
    /// Parse the puzzle input string into a useful model
    fn from_puzzle_input(puzzle_data: &str) -> Self {
        Self {
            hands: puzzle_data
                .lines()
                .map(|line| {
                    let (hand, bid) = line.split_once(' ').unwrap();
                    let bid = bid.parse().unwrap();

                    let hand: Vec<Card> = hand
                        .chars()
                        .map(|card| match card {
                            '2' => Card::Two,
                            '3' => Card::Three,
                            '4' => Card::Four,
                            '5' => Card::Five,
                            '6' => Card::Six,
                            '7' => Card::Seven,
                            '8' => Card::Eight,
                            '9' => Card::Nine,
                            'T' => Card::Ten,
                            'J' => Card::Jack,
                            'Q' => Card::Queen,
                            'K' => Card::King,
                            'A' => Card::Ace,
                            _ => panic!("only 13 types of cards broooooo"),
                        })
                        .collect();

                    CamelCardHand::new(&hand, bid)
                })
                .collect(),
        }
    }
}

#[derive(Debug)]
struct CamelCardHand {
    cards: Vec<Card>,
    score: i64,
    bid: i64,
}
impl CamelCardHand {
    /// Create a new card hand instance
    fn new(hand: &[Card], bid: i64) -> Self {
        let score = Self::score(hand);
        Self {
            cards: hand.to_vec(),
            score,
            bid,
        }
    }

    /// Compute a score for a hand of cards
    fn score(hand: &[Card]) -> i64 {
        // Sort the cards (makes it easier to distinguish the card hand type)
        let mut cards: Vec<Card> = hand.into();
        cards.sort();

        // Figure out what kind of hand type each hand is based on the cards
        let hand_type = if cards[0] == cards[4] {
            CamelCardHandType::FiveOfAKind
        } else if cards[0] == cards[3] || cards[1] == cards[4] {
            CamelCardHandType::FourOfAKind
        } else if (cards[0] == cards[1] && cards[2] == cards[4])
            || (cards[0] == cards[2] && cards[3] == cards[4])
        {
            CamelCardHandType::FullHouse
        } else if cards[0] == cards[2] || cards[1] == cards[3] || cards[2] == cards[4] {
            CamelCardHandType::ThreeOfAKind
        } else {
            let pair_count = cards
                .as_slice()
                .windows(2)
                .filter(|pair| pair[0] == pair[1])
                .count();
            match pair_count {
                0 => CamelCardHandType::HighCard,
                1 => CamelCardHandType::OnePair,
                2 => CamelCardHandType::TwoPair,
                _ => panic!("Only 7 types of card hands broooo"),
            }
        };

        // Compute a hands score based on hand type and cards
        let mut score = hand_type as usize;
        for card in hand {
            score = (score << 4) | (*card as usize);
        }
        score as i64
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
/// A useful model of each type of card hands there are
/// and their respective order/ranking from worst to best
enum CamelCardHandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
/// A useful model of a card in the puzzle
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
