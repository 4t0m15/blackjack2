use crate::art_handler::get_card_art;
use crate::card_handler::{read_char, GameState};
use std::io::{self, Write};

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

pub fn print_player_cards(state: &GameState) {
    println!("Your total: {}", hand_value(&state.player_cards));
    let card_art = get_card_art();
    let card_arts: Vec<Vec<&str>> = state
        .player_cards
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

pub fn player_turn(state: &mut GameState) -> bool {
    loop {
        print!("Choose an action: (h)it, (s)tand");
        if state.player_card_count == 2 && state.money >= state.bet {
            print!(", (d)ouble down");
        }
        print!(": ");
        io::stdout().flush().ok();

        let action = read_char();
        match action {
            'h' => {
                let card = draw(state);
                state.player_cards.push(card.clone());
                state.player_card_count = state.player_cards.len() as i32;
                println!("You got: {}", card);
                print_player_cards(state);

                if hand_value(&state.player_cards) > 21 {
                    println!("You busted! Dealer wins.");
                    state.games_lost += 1;
                    return false;
                }
            }
            's' => return true,
            'd' if state.player_card_count == 2 && state.money >= state.bet => {
                state.money -= state.bet;
                state.bet *= 2;
                let card = draw(state);
                state.player_cards.push(card.clone());
                state.player_card_count = state.player_cards.len() as i32;
                println!("You doubled down and drew: {}", card);
                print_player_cards(state);

                if hand_value(&state.player_cards) > 21 {
                    println!("You busted! Dealer wins.");
                    state.games_lost += 1;
                    return false;
                }
                return true;
            }
            'd' if state.money < state.bet => {
                println!("Not enough money to double down!");
            }
            _ => {
                println!("Invalid action, please choose again.");
            }
        }
    }
}

pub fn player_wins(state: &mut GameState) {
    use crate::art_handler::get_message;
    println!("\x1b[1;32m{}\x1b[0m", get_message("You Win!", None));
    state.money += state.bet * 2;
    state.games_won += 1;
}
