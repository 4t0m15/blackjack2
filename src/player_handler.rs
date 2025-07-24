use crate::card_handler::GameState;
use std::io::{self, Write};
use crate::art_handler::{get_card_art, get_message};

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
    println!("{} {}", get_message("Your cards", Some(state)), state.player_cards.join(", "));
    println!("{}", get_message("Your total", Some(state)));
    let card_art = get_card_art();
    let card_arts: Vec<Vec<&str>> = state.player_cards
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
    use crate::card_handler::{hand_value, draw};
    loop {
        print!("Do you want to (h)it, (s)tand, or (d)ouble down? ");
        io::stdout().flush().ok();
        let c = read_char();
        if c == 'h' {
            let card = draw(state);
            state.player_cards.push(card.clone());
            state.player_card_count = state.player_cards.len() as i32;
            println!("You got: {}", card);
            print_player_cards(state);
            if hand_value(&state.player_cards) > 21 {
                println!("Bust! You went over 21!");
                state.games_lost += 1;
                return false;
            }
        } else if c == 's' {
            return true;
        } else if c == 'd' && state.money >= state.bet {
            state.money -= state.bet;
            state.bet *= 2;
            let card = draw(state);
            state.player_cards.push(card.clone());
            state.player_card_count = state.player_cards.len() as i32;
            println!("You got: {}", card);
            print_player_cards(state);
            if hand_value(&state.player_cards) > 21 {
                println!("Bust! You went over 21!");
                state.games_lost += 1;
                return false;
            }
            return true;
        } else {
            println!("Please type 'h', 's', or 'd'.");
        }
    }
}

pub fn player_wins(state: &mut GameState) {
    use crate::card_handler::get_message;
    println!("\x1b[1;32m{}\x1b[0m", get_message("You Win!", None));
    state.money += state.bet * 2;
    state.games_won += 1;
}

pub fn dealer_wins(state: &mut GameState) {
    use crate::card_handler::get_message;
    println!("\x1b[1;31m{}\x1b[0m", get_message("Dealer Wins!", None));
    state.games_lost += 1;
}

fn read_char() -> char {
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok();
    line.trim().chars().next().unwrap_or('\n')
}
