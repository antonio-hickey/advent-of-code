/* Day - 7: Camel Cards */

use std::fs;

fn main() {
    // Read puzzle input into string
    let input = fs::read_to_string("./puzzle_data.txt").expect("some puzzle input data");
    println!("part one solution: {}", solution(&input, 1));
    println!("part two solution: {}", solution(&input, 2));
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
            // Same as part 1 solution but sorting by score with jokers included
            // the hard part of this part of the puzzle was just the parsing of the
            // puzzle data into something meaningful
            camel_cards.hands.sort_by(|a, b| a.score_with_joker.cmp(&b.score_with_joker));
            camel_cards
                .hands
                .iter()
                .enumerate()
                .fold(0, |acc, (rank, hand)| acc + (rank + 1) as i64 * hand.bid)
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
    score_with_joker: i64,
    bid: i64,
}
impl CamelCardHand {
    /// Create a new card hand instance
    fn new(hand: &[Card], bid: i64) -> Self {
        let score = Self::score(hand);
        let score_with_joker = Self::score_with_joker(hand);
        Self {
            cards: hand.to_vec(),
            score,
            score_with_joker,
            bid,
        }
    }

    /// Compute a score for a hand of cards
    fn score(hand: &[Card]) -> i64 {
        // Sort the cards (makes it easier to distinguish the card hand type)
        let mut cards: Vec<Card> = hand.into();
        cards.sort();

        // Figure out how many joker cards there are in the hand
        let n_jokers = cards.iter().filter(|card| **card == Card::Joker).count();

        // Figure out what kind of hand type each hand is based on the cards
        let hand_type = if cards[0].is_equal(cards[4]) {
            CamelCardHandType::FiveOfAKind
        } else if cards[0].is_equal(cards[3]) || cards[1].is_equal(cards[4]) {
            // Check if theres a joker in the hand that can convert the
            // hand from a four of a kind to a five of a kind.
            if n_jokers == 1 {
                CamelCardHandType::FiveOfAKind
            } else {
                CamelCardHandType::FourOfAKind
            }
        } else if (cards[0].is_equal(cards[1]) && cards[2].is_equal(cards[4]))
            || (cards[0].is_equal(cards[2]) && cards[3].is_equal(cards[4]))
        {
            // A full house cant have a joker by definition
            CamelCardHandType::FullHouse
        } else if cards[0].is_equal(cards[2]) || cards[1].is_equal(cards[3]) || cards[2].is_equal(cards[4]) {
            // Match how many jokers there are in the hand to see how much
            // the 3 of a kind can be upgraded
            match n_jokers {
                0 => CamelCardHandType::ThreeOfAKind,
                1 => CamelCardHandType::FourOfAKind,
                2 => CamelCardHandType::FiveOfAKind,
                _ => panic!("no less than 0 and no more than 2 jokers should be possible")
            }
        } else {
            let pair_count = cards
                .as_slice()
                .windows(2)
                .filter(|pair| pair[0].is_equal(pair[1]))
                .count();

            if n_jokers == 0 {
                match pair_count {
                    0 => CamelCardHandType::HighCard,
                    1 => CamelCardHandType::OnePair,
                    2 => CamelCardHandType::TwoPair,
                    _ => panic!("Only 7 types of card hands broooo")
                }
            } else if n_jokers == 1 {
                match pair_count {
                    0 => CamelCardHandType::OnePair,
                    1 => CamelCardHandType::ThreeOfAKind,
                    2 => CamelCardHandType::FullHouse,
                    _ => panic!("Only 7 types of card hands broooo"),
                }
            } else if n_jokers == 2 {
                match pair_count {
                    0 => CamelCardHandType::ThreeOfAKind,
                    1 => CamelCardHandType::FourOfAKind,
                    _ => panic!("Only 7 types of card hands broooo"), 
                }
            } else if (n_jokers == 3 && pair_count == 1) || n_jokers >= 4 {
                CamelCardHandType::FiveOfAKind
            } else if n_jokers == 3 {
                CamelCardHandType::FourOfAKind
            } else {
                CamelCardHandType::HighCard
            }
        };

        // Compute a hands score based on hand type and cards
        let mut score = hand_type as usize;
        for card in hand {
            score = (score << 4) | (*card as usize);
        }
        score as i64
    }

    /// Compute a score for a hand of cards with jokers which can act
    /// as wildcards instead of jacks
    fn score_with_joker(hand: &[Card]) -> i64 {
        let hand_with_joker: Vec<Card> = hand.iter().map(|card| {
            if *card == Card::Jack {
                Card::Joker
            } else {
                *card
            }
        }).collect();

        Self::score(&hand_with_joker)
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
    Joker,
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
impl Card {
    /// Custom comparison to not count jokers as equal yet
    fn is_equal(self, other: Card) -> bool {
        if self == Self::Joker || other == Self::Joker {
            false
        } else {
            self == other
        }
    }
}
