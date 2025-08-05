use crate::art_handler::{get_card_art, get_message};
use crate::player_handler::{card_art_index, draw, hand_value};

pub fn dealer_turn(state: &mut crate::card_handler::GameState) {
    println!("{}", get_message("Dealer's turn:", None));
    while hand_value(&state.dealer_cards) < 17 {
        let card = draw(state);
        state.dealer_cards.push(card.clone());
        state.dealer_card_count = state.dealer_cards.len() as i32;
        println!("{}", get_message("Dealer draws:", Some(state)));
    }
    println!("{}", get_message("Dealer's cards:", Some(state)));
    let card_art = get_card_art();
    // Print the ASCII art for each dealer card side by side
    let card_arts: Vec<Vec<&str>> = state
        .dealer_cards
        .iter()
        .map(|card| card_art[card_art_index(card)].lines().collect())
        .collect();
    if !card_arts.is_empty() {
        for line_idx in 0..card_arts[0].len() {
            for card in &card_arts {
                print!("{} ", card[line_idx]);
            }
            println!();
        }
    }
}
