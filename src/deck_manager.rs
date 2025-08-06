use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_deck() -> Vec<String> {
    let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
    let ranks = [
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
    ];
    suits
        .iter()
        .flat_map(|s| ranks.iter().map(move |r| format!("{r} {s}")))
        .collect()
}

pub fn shuffle_deck(deck: &mut [String]) {
    deck.shuffle(&mut thread_rng());
}

pub fn create_and_shuffle_deck() -> Vec<String> {
    let mut deck = create_deck();
    shuffle_deck(&mut deck);
    deck
}
