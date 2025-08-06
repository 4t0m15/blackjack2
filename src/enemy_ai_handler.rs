use crate::art_handler::get_card_art;
use crate::game_state::GameState;
use crate::player_handler::{card_art_index, draw, hand_value};

fn is_soft_17(hand: &[String]) -> bool {
    if hand_value(hand) != 17 {
        return false;
    }

    // Check if hand contains an Ace counted as 11
    let mut total = 0;
    let mut has_ace = false;

    for card in hand {
        if let Some(rank) = card.split_whitespace().next() {
            match rank {
                "A" => {
                    has_ace = true;
                    total += 11;
                }
                "K" | "Q" | "J" => total += 10,
                _ => total += rank.parse::<i32>().unwrap_or(0),
            }
        }
    }

    // If we have an ace and total is 17, it's soft 17
    has_ace && total == 17
}

#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn dealer_turn(state: &mut GameState) {
    println!("Dealer's turn:");

    // First, reveal the hole card (second card)
    println!("Dealer reveals hole card: {}", state.dealer_cards[1]);
    print_dealer_cards(&state.dealer_cards);

    // Now dealer follows house rules (hit on soft 17, stand on hard 17)
    while hand_value(&state.dealer_cards) < 17 || is_soft_17(&state.dealer_cards) {
        let card = draw(state);
        state.dealer_cards.push(card.clone());
        state.dealer_card_count = state.dealer_cards.len() as i32;
        println!("Dealer draws: {card}");
        print_dealer_cards(&state.dealer_cards);
    }
}

fn print_dealer_cards(dealer_cards: &[String]) {
    println!("Dealer's total: {}", hand_value(dealer_cards));
    let card_art = get_card_art();
    let card_arts: Vec<Vec<&str>> = dealer_cards
        .iter()
        .map(|card| card_art[card_art_index(card)].lines().collect())
        .collect();
    if card_arts.is_empty() {
        return;
    }
    for line_idx in 0..card_arts[0].len() {
        for card in &card_arts {
            print!("{} ", card[line_idx]);
        }
        println!();
    }
}
