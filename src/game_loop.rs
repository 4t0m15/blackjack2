use crate::art_handler::{get_message, get_splash_screen, print_game_status};
use crate::deck_manager::create_and_shuffle_deck;
use crate::enemy_ai_handler;
use crate::game_state::GameState;
use crate::player_handler::{player_turn, print_player_cards};
use crate::win_checker::determine_winner;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub fn start_blackjack() {
    let mut state = GameState::new();
    start_blackjack_with_state(&mut state);
}

pub fn start_blackjack_with_state(state: &mut GameState) {
    print_splash_screen();
    delay();
    loop {
        // Check if player has any money before starting a new round
        if state.money <= 0 {
            println!("\x1b[1;31m{}\x1b[0m", get_message("Game Over", None));
            print!("{} ", get_message("Do you want to (t)ry again", None));
            io::stdout().flush().ok();
            let c = read_char();
            if c == 't' {
                state.money = crate::save_system::STARTING_MONEY;
                state.games_won = 0;
                state.games_lost = 0;
                // Save the reset state
                state.save_to_disk();
                // Don't reset history - keep the game history across restarts
                continue;
            }
            break;
        }
        
        setup_new_round(state);
        print_game_status(state);
        state.current_round_start_money = state.money;
        state.bet = get_bet(state);
        if state.bet == -1 {
            break;
        }
        state.money -= state.bet;
        state.was_double_down = false;
        println!("{}", get_message("Dealer shows", Some(state)));
        print_player_cards(state);
        
        let player_busted = !player_turn(state);
        if player_busted {
            // Player busted, record the result without dealer turn
            determine_winner(state);
        } else {
            // Player didn't bust, continue with dealer turn
            enemy_ai_handler::dealer_turn(state);
            determine_winner(state);
        }
        
        // Check if player ran out of money after this round
        if state.money <= 0 {
            println!("\x1b[1;31m{}\x1b[0m", get_message("Game Over", None));
            print!("{} ", get_message("Do you want to (t)ry again", None));
            io::stdout().flush().ok();
            let c = read_char();
            if c == 't' {
                state.money = crate::save_system::STARTING_MONEY;
                state.games_won = 0;
                state.games_lost = 0;
                // Save the reset state
                state.save_to_disk();
                // Don't reset history - keep the game history across restarts
                continue;
            }
            break;
        }
    }
}

fn print_splash_screen() {
    println!("{}", get_splash_screen());
    delay();
    println!("{}", get_message("loading...", None));
}

fn delay() {
    thread::sleep(Duration::from_secs(2));
}

#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
fn setup_new_round(state: &mut GameState) {
    state.card_deck = create_and_shuffle_deck();
    state.player_cards.clear();
    state.dealer_cards.clear();
    state.player_cards.push(state.card_deck[0].clone());
    state.player_cards.push(state.card_deck[1].clone());
    state.dealer_cards.push(state.card_deck[2].clone());
    state.dealer_cards.push(state.card_deck[3].clone());
    state.deck_index = 4;
    state.player_card_count = state.player_cards.len() as i32;
    state.dealer_card_count = state.dealer_cards.len() as i32;
}

fn get_bet(state: &GameState) -> i32 {
    loop {
        print!("How many coins do you want to bet? (m to return to main menu): ");
        io::stdout().flush().ok();
        let mut line = String::new();
        io::stdin().read_line(&mut line).ok();

        let trimmed = line.trim();
        if trimmed.eq_ignore_ascii_case("m") {
            return -1; // Special value to indicate returning to menu
        }

        if let Ok(n) = trimmed.parse::<i32>() {
            if n > 0 && n <= state.money {
                return n;
            }
        }
        println!("Please bet between 1 and {} coins.", state.money);
    }
}

#[must_use]
pub fn read_char() -> char {
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok();
    line.trim().chars().next().unwrap_or('\n')
}
