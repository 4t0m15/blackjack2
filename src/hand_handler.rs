use crate::card_handler::GameState;

pub fn hand_value(hand: &[String]) -> i32 {
    let mut total = 0;
    let mut aces = 0;
    for card in hand {
        let rank = card.split_whitespace().next().unwrap();
        match rank {
            "A" => {
                total += 11;
                aces += 1;
            }
            "K" | "Q" | "J" => total += 10,
            _ => total += rank.parse::<i32>().unwrap_or(0),
        }
    }
    while total > 21 && aces > 0 {
        total -= 10;
        aces -= 1;
    }
    total
}

pub fn card_art_index(card: &str) -> usize {
    match card.split_whitespace().next().unwrap() {
        "A" => 0,
        "2" => 1,
        "3" => 2,
        "4" => 3,
        "5" => 4,
        "6" => 5,
        "7" => 6,
        "8" => 7,
        "9" => 8,
        "10" => 9,
        "J" => 10,
        "Q" => 11,
        "K" => 12,
        _ => 0,
    }
}

pub fn draw(state: &mut GameState) -> String {
    let card = state.card_deck[state.deck_index as usize].clone();
    state.deck_index += 1;
    card
}
