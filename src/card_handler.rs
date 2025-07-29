use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;

/// Creates a new shuffled deck of 52 cards as VecDeque<String>.
pub fn create_deck() -> VecDeque<String> {
    let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
    let ranks = [
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
    ];
    let mut deck: Vec<String> = Vec::with_capacity(52);
    for suit in suits.iter() {
        for rank in ranks.iter() {
            deck.push(format!("{} of {}", rank, suit));
        }
    }
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);
    VecDeque::from(deck)
}

/// Returns the value of a single card (as per Blackjack rules).
pub fn get_card_value(card: &str) -> i32 {
    let rank = card.split_whitespace().next().unwrap_or("");
    match rank {
        "A" => 11,
        "K" | "Q" | "J" => 10,
        _ => rank.parse::<i32>().unwrap_or(0),
    }
}

/// Returns the total value of a hand, accounting for Aces as 1 or 11.
pub fn get_hand_value(hand: &[String]) -> i32 {
    let mut total = 0;
    let mut aces = 0;
    for card in hand {
        let value = get_card_value(card);
        if value == 11 {
            aces += 1;
        }
        total += value;
    }
    while total > 21 && aces > 0 {
        total -= 10;
        aces -= 1;
    }
    total
}
