use crate::art_handler::get_card_art;
use crate::game_state::GameState;
use crate::player_handler::{card_art_index, draw, hand_value};

pub fn dealer_turn(state: &mut GameState) {
    println!("Dealer's turn:");
    println!("Dealer's cards: [{}, Hidden]", state.dealer_cards[0]);
    while hand_value(&state.dealer_cards) < 17 {
        let card = draw(state);
        state.dealer_cards.push(card.clone());
        state.dealer_card_count = state.dealer_cards.len() as i32;
        println!("Dealer draws: {card}");
    }
    println!(
        "Dealer's cards: {:?}",
        state
            .dealer_cards
            .iter()
            .map(|c| c.as_str())
            .collect::<Vec<_>>()
    );
    let card_art = get_card_art();
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
